use super::{models, PageGames, PagePlugins, SettingsWindow};
use adw::subclass::prelude::*;
use deps::{plugins::PluginRegistry, utils::composite_widget};
use gtk::{
    gio::{self, ListStore},
    glib,
    prelude::StaticType,
    traits::GtkWindowExt,
    CompositeTemplate,
};
use once_cell::unsync::OnceCell;
use std::cell::RefCell;

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
        for decl in [&plugins_epic::plugin_declaration] {
            self.registry.load::<plugins_epic::Plugin>().unwrap();
        }

        for plugin in self.registry.iter_plugins() {
            self.plugin_store.append(&models::Plugin::new(plugin));
        }
    }
}

impl Drop for AppWindowData {
    fn drop(&mut self) {
        // Keeping these alive prior to dropping PluginRegistry means
        // Rust will panic/throw (scary, I know) a STATUS_ACCESS_VIOLATION
        // This is due to the Weak<> drop implementation trying to
        // access unloaded dll memory to decrement its weak count
        self.game_store.remove_all();
        self.plugin_store.remove_all();
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

    #[template_callback]
    fn on_open_settings(&self) {
        SettingsWindow::new(self.data.borrow().get().unwrap().plugin_store.clone()).present();
    }
}

impl ObjectImpl for AppWindowInner {
    fn constructed(&self) {
        self.parent_constructed();

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
