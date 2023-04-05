use std::{cell::RefCell, rc::Rc};

use gtk::glib;
use gtk::prelude::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LoadPhase {
    Initialize,
    UILoad,
}

pub trait ModuleCtx {
    fn try_get_object<T: IsA<glib::Object>>(&self, name: &'static str) -> Option<T>;

    fn get_object<T: IsA<glib::Object>>(&self, name: &'static str) -> T {
        self.try_get_object(name)
            .unwrap_or_else(|| panic!("Failed to get object {}", name))
    }

    fn try_get_module<T: Module>(&self) -> Option<Rc<RefCell<T>>>;

    fn get_module<T: Module>(&self) -> Rc<RefCell<T>> {
        self.try_get_module::<T>()
            .unwrap_or_else(|| panic!("Failed to get module {}", std::any::type_name::<T>()))
    }

    fn get_application(&self) -> gtk::Application;
}

pub trait Module: Sized + 'static {
    const META: ModuleMeta;

    fn new(ctx: &impl ModuleCtx) -> Self;
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct ModuleMeta {
    pub phase: LoadPhase,
    pub priority: u16,
}
