use super::{models, PageGames, PagePlugins, SettingsWindow};
use adw::subclass::prelude::*;
use deps::{plugins::PluginRegistry, utils::composite_widget};
#[cfg(debug_assertions)]
use gtk::traits::WidgetExt;
use gtk::{
    gio::{self, prelude::*, ApplicationCommandLine, ListStore},
    glib,
    prelude::StaticType,
    traits::GtkWindowExt,
    CompositeTemplate,
};
use std::cell::{OnceCell, RefCell};

composite_widget!(AppWindow => "L4AppWindow",
    @inner AppWindowInner!,
    @parent adw::ApplicationWindow,
    @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager,
    @uses PagePlugins, PageGames, SettingsWindow
);

pub struct AppWindowData {
    registry: PluginRegistry,
    plugin_store: ListStore,
    game_store: ListStore,
}

impl AppWindowData {
    fn new(_outer: &AppWindowInner) -> Self {
        Self {
            registry: PluginRegistry::new(),
            plugin_store: ListStore::new(models::Plugin::static_type()),
            game_store: ListStore::new(models::Game::static_type()),
        }
    }

    fn load_plugins(&mut self) {
        self.registry.load::<plugins_epic::Plugin>().unwrap();

        for plugin in self.registry.iter_plugins() {
            self.plugin_store.append(&models::Plugin::new(plugin));
        }
    }
}

#[derive(CompositeTemplate, Default)]
#[template(resource = "/me/workingrobot/l4/templates/app_window.ui")]
pub struct AppWindowInner {
    #[template_child(id = "page-plugins")]
    page_plugins: TemplateChild<PagePlugins>,
    #[template_child(id = "page-games")]
    page_games: TemplateChild<PageGames>,

    data: RefCell<OnceCell<AppWindowData>>,
}

#[gtk::template_callbacks]
impl AppWindowInner {
    #[template_callback]
    fn on_open_inspector(&self) {
        gtk::Window::set_interactive_debugging(true);
    }

    #[template_callback]
    fn is_debug_mode(&self) -> bool {
        cfg!(debug_assertions)
    }

    pub fn on_protocol_callback(
        &self,
        command_line: &ApplicationCommandLine,
    ) -> std::io::Result<()> {
        let argv = command_line.arguments();

        if argv.len() < 3 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "argument list is too short",
            ));
        }

        let plugin_id = argv[1].to_str().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "plugin id is not valid utf-8",
            )
        })?;
        let data = argv[2].to_str().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "data is not valid utf-8")
        })?;

        let plugin_id = plugin_id.strip_prefix("proto-").ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "plugin id does not being with 'proto-'",
            )
        })?;

        self.data
            .borrow()
            .get()
            .unwrap()
            .registry
            .on_protocol_callback(plugin_id, data);

        Ok(())
    }

    #[template_callback]
    fn on_open_settings(&self) {
        SettingsWindow::new(self.data.borrow().get().unwrap().plugin_store.clone()).present();
    }
}

impl ObjectImpl for AppWindowInner {
    fn constructed(&self) {
        self.parent_constructed();

        #[cfg(debug_assertions)]
        self.obj().add_css_class("devel");

        _ = self.data.borrow().set(AppWindowData::new(self));
        let mut data = self.data.borrow_mut();
        let data = data.get_mut().unwrap();

        data.load_plugins();
        self.page_games.set_model(&data.game_store);
        self.page_plugins.set_model(&data.plugin_store);
    }
}

impl WidgetImpl for AppWindowInner {}

impl WindowImpl for AppWindowInner {}

impl ApplicationWindowImpl for AppWindowInner {}

impl AdwApplicationWindowImpl for AppWindowInner {}

impl AppWindow {
    pub fn new(app: &gtk::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}
