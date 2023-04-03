use gtk::glib;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub trait ModuleCtx {
    fn try_get_object<T: IsA<glib::Object>>(&self, name: &'static str) -> Option<T>;

    fn get_object<T: IsA<glib::Object>>(&self, name: &'static str) -> T {
        self.try_get_object(name)
            .unwrap_or_else(|| panic!("Failed to get object {}", name))
    }
}

pub trait Module {
    fn new(ctx: &impl ModuleCtx) -> Rc<RefCell<Self>>
    where
        Self: Sized;
}
