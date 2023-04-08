use super::{registry::ModuleRegistry, LoadPhase, ModuleCtx};
use gtk::prelude::*;
use gtk::{glib, Builder};
use once_cell::unsync::OnceCell;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ModuleList {
    application: glib::WeakRef<gtk::Application>,
    builder: OnceCell<Builder>,
    modules: Vec<Rc<dyn std::any::Any>>,
    registry: ModuleRegistry,
}

impl ModuleCtx for ModuleList {
    fn try_get_object<T: IsA<glib::Object>>(&self, name: &'static str) -> Option<T> {
        self.builder.get().and_then(|b| b.object(name))
    }

    fn try_get_module<T: super::Module>(&self) -> Option<Rc<RefCell<T>>> {
        for module in &self.modules {
            if let Ok(module) = Rc::downcast::<RefCell<T>>(module.to_owned()) {
                return Some(module);
            }
        }
        None
    }

    fn get_application(&self) -> gtk::Application {
        self.application.upgrade().unwrap()
    }
}

impl ModuleList {
    pub fn new(application: &impl IsA<gtk::Application>) -> Self {
        let mut this = Self {
            application: application.as_ref().downgrade(),
            builder: OnceCell::new(),
            modules: vec![],
            registry: ModuleRegistry::new(),
        };

        this.load(LoadPhase::Initialize);

        this.builder
            .set(Builder::from_resource("/me/workingrobot/l4/main.ui"))
            .unwrap();

        this.load(LoadPhase::UILoad);

        this
    }

    fn load(&mut self, phase: LoadPhase) {
        for initializer in self.registry.iter_phase(phase) {
            self.modules.push(initializer(self));
        }
    }
}
