use super::{composite_widget, models, PageAbout, PageGames, PagePlugins, PageSettings};
use deps::plugins::PluginRegistry;
use gtk::{
    gio::{self, ListStore},
    glib,
    prelude::StaticType,
    subclass::prelude::*,
    traits::WidgetExt,
    Button, CompositeTemplate, Stack,
};
use once_cell::unsync::OnceCell;
use std::cell::RefCell;

composite_widget!(AppWindow => "L4AppWindow",
    @inner AppWindowInner!,
    @parent gtk::ApplicationWindow,
    @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager,
    @uses PagePlugins, PageGames, PageSettings, PageAbout
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
        std::fs::read_dir(std::env::current_exe().unwrap().parent().unwrap())
            .unwrap()
            .flatten()
            .filter(|e| e.metadata().map_or(false, |m| m.is_file()))
            .filter(|e| {
                e.file_name()
                    .to_str()
                    .map_or(false, |f| f.starts_with("plugins_") && f.ends_with(".dll"))
            })
            .for_each(|e| unsafe { self.registry.load(e.path()) }.unwrap());

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
    #[template_child]
    stack: TemplateChild<Stack>,
    #[template_child(id = "button-back")]
    button_back: TemplateChild<Button>,

    #[template_child(id = "page-plugins")]
    page_plugins: TemplateChild<PagePlugins>,
    #[template_child(id = "page-games")]
    page_games: TemplateChild<PageGames>,
    #[template_child(id = "page-settings")]
    page_settings: TemplateChild<PageSettings>,

    data: RefCell<OnceCell<AppWindowData>>,
}

#[gtk::template_callbacks]
impl AppWindowInner {
    #[template_callback]
    fn on_back_clicked(&self) {
        self.stack.set_visible_child_name("plugins");
    }

    #[template_callback]
    fn on_open_settings(&self) {
        self.stack.set_visible_child_name("settings");
    }

    #[template_callback]
    fn on_open_about(&self) {
        self.stack.set_visible_child_name("about");
    }

    #[template_callback]
    fn on_open_inspector(&self) {
        gtk::Window::set_interactive_debugging(true);
    }

    #[template_callback]
    fn on_open_games(&self) {
        self.stack.set_visible_child_name("games");
    }

    #[template_callback]
    fn on_stack_switch(&self) {
        self.button_back
            .set_visible(self.stack.visible_child_name().unwrap_or_default() != "plugins");
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
        self.page_settings.init_model(data.plugin_store.clone());
    }
}

impl WidgetImpl for AppWindowInner {}

impl WindowImpl for AppWindowInner {}

impl ApplicationWindowImpl for AppWindowInner {}

impl AppWindow {
    pub fn new(app: &gtk::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}
