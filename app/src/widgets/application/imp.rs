use crate::modules::{ModuleCtx, ModuleList, ThemePicker};
use crate::utils::dpi::UsesDpi;

use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gdk, glib, prelude::Cast, traits::GtkWindowExt};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Application {
    modules: Rc<RefCell<ModuleList>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Application {
    const NAME: &'static str = "L4Application";
    type Type = super::Application;
    type ParentType = adw::Application;
}

impl ObjectImpl for Application {}

impl ApplicationImpl for Application {
    fn startup(&self) {
        self.parent_startup();

        let manager = adw::StyleManager::default();
        manager.set_color_scheme(adw::ColorScheme::PreferDark);
    }

    fn activate(&self) {
        self.parent_activate();

        let window = self
            .modules
            .borrow()
            .get_object::<gtk::ApplicationWindow>("window");
        window.set_application(Some(self.obj().upcast_ref::<gtk::Application>()));
        window.minimize();
        window.present();

        let settings = gtk::Settings::default().expect("Could not get default settings");
        settings.set_gtk_xft_dpi(window.get_dpi().unwrap_or(96) as i32 * 1024);
        settings.set_gtk_theme_name(Some("Sweet-Ambar-Blue"));

        //self.modules.borrow_mut().add::<ThemePicker>();
    }
}

impl GtkApplicationImpl for Application {}

impl AdwApplicationImpl for Application {}
