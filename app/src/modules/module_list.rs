use super::{Module, ModuleCtx};
use gtk::prelude::*;
use gtk::{gdk, glib, Builder, IconTheme};
use std::cell::RefCell;
use std::rc::Rc;

pub struct ModuleList {
    builder: Builder,
    modules: Vec<Rc<RefCell<dyn Module>>>,
}

impl ModuleCtx for ModuleList {
    fn try_get_object<T: IsA<glib::Object>>(&self, name: &'static str) -> Option<T> {
        self.builder.object(name)
    }
}

impl Default for ModuleList {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleList {
    pub fn new() -> Self {
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

        Self {
            builder: Builder::from_resource("/me/workingrobot/l4/main.ui"),
            modules: vec![],
        }
    }

    pub fn add<T: Module + 'static>(&mut self) {
        self.modules.push(T::new(self));
    }

    pub fn initialize(&mut self) {
        self.add::<super::Inspector>();
    }
}
