use super::{Module, ModuleCtx};
use gdk4_win32::glib::subclass::register_type;
use gtk::prelude::*;
use gtk::{gdk, glib, Builder, IconTheme};
use std::cell::RefCell;
use std::rc::Rc;

pub struct ModuleList {
    application: glib::WeakRef<gtk::Application>,
    builder: Builder,
    modules: Vec<Rc<RefCell<dyn Module>>>,
}

impl ModuleCtx for ModuleList {
    fn try_get_object<T: IsA<glib::Object>>(&self, name: &'static str) -> Option<T> {
        self.builder.object(name)
    }

    fn get_application(&self) -> gtk::Application {
        self.application.upgrade().unwrap()
    }
}

impl ModuleList {
    pub fn new(application: &impl IsA<gtk::Application>) -> Self {
        let display = gdk::Display::default().expect("Could not get a display");
        let icon_theme = IconTheme::for_display(&display);
        icon_theme.add_resource_path("/me/workingrobot/l4");
        icon_theme.add_resource_path("/com/fontawesome/icons");

        println!("-- searches");
        for search_path in icon_theme.search_path() {
            println!("{}", search_path.display());
        }
        println!("-- resources");
        for search_path in icon_theme.resource_path() {
            println!("{search_path}");
        }
        println!("-- end");

        // Register types
        crate::widgets::PluginModel::static_type();

        let mut this = Self {
            application: application.as_ref().downgrade(),
            builder: Builder::from_resource("/me/workingrobot/l4/main.ui"),
            modules: vec![],
        };
        this.initialize();

        this
    }

    pub fn add<T: Module + 'static>(&mut self) {
        self.modules.push(T::new(self));
    }

    fn initialize(&mut self) {
        self.add::<super::Init>();
        self.add::<super::TitleButtons>();
        self.add::<super::Plugins>();
    }
}
