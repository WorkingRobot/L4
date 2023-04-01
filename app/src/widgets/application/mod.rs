mod imp;

use glib::Object;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends adw::Application, gtk::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Application {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn with_application_id(&self, application_id: &str) -> Self {
        Object::builder()
            .property("application-id", application_id)
            .build()
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
