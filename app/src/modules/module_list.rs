use super::{registry::Registry, LoadPhase, ModuleCtx, ModuleInst};
use gtk::prelude::*;
use gtk::{glib, Builder};
use once_cell::unsync::OnceCell;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ModuleList {
    application: glib::WeakRef<gtk::Application>,
    builder: OnceCell<Builder>,
    modules: Vec<Rc<RefCell<dyn ModuleInst>>>,
    registry: Registry,
}

impl ModuleCtx for ModuleList {
    fn try_get_object<T: IsA<glib::Object>>(&self, name: &'static str) -> Option<T> {
        self.builder.get().and_then(|b| b.object(name))
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
            registry: Registry::new(),
        };

        this.load(LoadPhase::Initialize);

        this.builder
            .set(Builder::from_resource("/me/workingrobot/l4/main.ui"))
            .expect("Builder already created?");

        this.load(LoadPhase::UILoad);

        this
    }

    fn load(&mut self, phase: LoadPhase) {
        for module in self.registry.iter_phase(phase) {
            println!("Loading {}", module.0.name);
            self.modules.push(module.1(self));
        }
    }
}
