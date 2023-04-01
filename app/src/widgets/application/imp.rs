use crate::modules::{ModuleCtx, ModuleList, ThemePicker};

use crate::utils::dpi::UsesDpi;
use adw::subclass::prelude::*;
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
        manager.set_color_scheme(adw::ColorScheme::ForceDark);

        let display = gdk::Display::default().expect("Could not get a display");

        let provider = gtk::CssProvider::new();
        provider.connect_parsing_error(|_, section, error| {
            panic!("Could not parse css data ({} at {})", error, section);
        });
        provider.load_from_resource("/me/workingrobot/l4/main.css");

        #[allow(deprecated)]
        // add_provider_for_display isn't actually deprecated, but the rest of StyleContext is
        gtk::StyleContext::add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let window = self
            .modules
            .borrow()
            .get_object::<gtk::ApplicationWindow>("window");

        window.set_application(Some(self.obj().upcast_ref::<gtk::Application>()));
    }

    fn activate(&self) {
        self.parent_activate();

        let window = self
            .modules
            .borrow()
            .get_object::<gtk::ApplicationWindow>("window");
        window.minimize();
        window.present();

        let settings = gtk::Settings::default().expect("Could not get default settings");
        settings.set_gtk_xft_dpi(window.get_dpi().unwrap_or(96) as i32 * 1024);

        self.modules.borrow_mut().add::<ThemePicker>();
    }
}

impl GtkApplicationImpl for Application {}

impl AdwApplicationImpl for Application {}
