mod imp;

use glib::Object;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends adw::Application, gtk::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Application {
    pub fn from_application_id(application_id: &str) -> Self {
        Object::builder()
            .property("application-id", application_id)
            .build()
    }
}
