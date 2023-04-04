use crate::modules::{ModuleCtx, ModuleList};

use adw::subclass::prelude::*;
use gtk::{glib, traits::GtkWindowExt};
use once_cell::unsync::OnceCell;

pub struct Application {
    modules: OnceCell<ModuleList>,
}

#[glib::object_subclass]
impl ObjectSubclass for Application {
    const NAME: &'static str = "L4Application";
    type Type = super::Application;
    type ParentType = adw::Application;

    fn new() -> Self {
        Self {
            modules: OnceCell::new(),
        }
    }
}

impl ObjectImpl for Application {}

impl ApplicationImpl for Application {
    fn activate(&self) {
        self.parent_activate();

        let modules = self
            .modules
            .get_or_init(|| ModuleList::new(self.obj().as_ref()));

        let window = modules.get_object::<gtk::ApplicationWindow>("window");
        window.minimize();
        window.present();
    }

    fn startup(&self) {
        self.parent_startup();
    }
}

impl GtkApplicationImpl for Application {}

impl AdwApplicationImpl for Application {}
