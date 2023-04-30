use super::{composite_widget, PageAbout, PageGames, PagePlugins, PageSettings};
use gtk::{gio, glib, subclass::prelude::*, traits::WidgetExt, Button, CompositeTemplate, Stack};

composite_widget!(AppWindow => "L4AppWindow",
    @inner AppWindowInner,
    @parent gtk::ApplicationWindow,
    @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager,
    @uses PagePlugins, PageGames, PageSettings, PageAbout
);

#[derive(CompositeTemplate, Default)]
#[template(resource = "/me/workingrobot/l4/templates/app_window.ui")]
pub struct AppWindowInner {
    #[template_child]
    stack: TemplateChild<Stack>,
    #[template_child(id = "button-back")]
    button_back: TemplateChild<Button>,
}

#[gtk::template_callbacks]
impl AppWindowInner {
    fn init(&self) {}

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

impl WidgetImpl for AppWindowInner {}

impl WindowImpl for AppWindowInner {}

impl ApplicationWindowImpl for AppWindowInner {}

impl AppWindow {
    pub fn new(app: &gtk::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}
