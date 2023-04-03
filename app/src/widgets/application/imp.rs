use crate::modules::{ModuleCtx, ModuleList};
use crate::utils::dpi::UsesDpi;

use adw::subclass::prelude::*;
use gtk::{glib, prelude::Cast, traits::GtkWindowExt};
use std::cell::RefCell;

pub struct Application {
    modules: RefCell<Option<ModuleList>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Application {
    const NAME: &'static str = "L4Application";
    type Type = super::Application;
    type ParentType = adw::Application;

    fn new() -> Self {
        Self {
            modules: RefCell::new(None),
        }
    }
}

impl ObjectImpl for Application {}

impl ApplicationImpl for Application {
    fn activate(&self) {
        self.parent_activate();

        let is_startup = self.modules.borrow().is_none();
        if is_startup {
            *self.modules.borrow_mut() = Some(ModuleList::new());
        }
        let binding = self.modules.borrow();
        let modules = binding.as_ref().unwrap();

        let window = modules.get_object::<gtk::ApplicationWindow>("window");
        window.set_application(Some(self.obj().upcast_ref::<gtk::Application>()));
        window.minimize();
        window.present();

        if is_startup {
            let manager = adw::StyleManager::default();
            manager.set_color_scheme(adw::ColorScheme::PreferDark);

            let settings = gtk::Settings::default().expect("Could not get default settings");
            settings.set_gtk_xft_dpi(window.get_dpi().unwrap_or(96) as i32 * 1024);
            settings.set_gtk_theme_name(Some("Sweet-Ambar-Blue"));
        }
    }

    fn startup(&self) {
        self.parent_startup();
    }
}

impl GtkApplicationImpl for Application {}

impl AdwApplicationImpl for Application {}
