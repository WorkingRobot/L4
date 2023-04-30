use gtk::{gio, glib};

mod imp {
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate};
    use gtk::{glib::subclass::InitializingObject, Stack};
    use gtk::{prelude::*, Button};

    use crate::widgets::{PageAbout, PageGames, PagePlugins, PageSettings};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/me/workingrobot/l4/templates/app_window.ui")]
    pub struct AppWindow {
        #[template_child]
        stack: TemplateChild<Stack>,
        #[template_child(id = "button-back")]
        button_back: TemplateChild<Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppWindow {
        const NAME: &'static str = "L4AppWindow";
        type Type = super::AppWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            PagePlugins::ensure_type();
            PageGames::ensure_type();
            PageSettings::ensure_type();
            PageAbout::ensure_type();

            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl AppWindow {
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

    impl ObjectImpl for AppWindow {}

    impl WidgetImpl for AppWindow {}

    impl WindowImpl for AppWindow {}

    impl ApplicationWindowImpl for AppWindow {}
}

glib::wrapper! {
    pub struct AppWindow(ObjectSubclass<imp::AppWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl AppWindow {
    pub fn new(app: &gtk::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}
