use super::{LoadPhase, Metadata, Module, ModuleList};
use crate::modules;
use once_cell::unsync::Lazy;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

type Initializer = fn(&ModuleList) -> Rc<dyn std::any::Any>;

pub struct ModuleRegistry {
    imp: Lazy<BTreeMap<Metadata, Initializer>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            imp: Lazy::new(Self::create),
        }
    }

    fn create() -> BTreeMap<Metadata, Initializer> {
        let mut reg = BTreeMap::new();
        Self::register::<modules::UIPreInit>(&mut reg);
        Self::register::<modules::UIPostInit>(&mut reg);
        Self::register::<modules::TitleButtons>(&mut reg);
        Self::register::<modules::Plugins>(&mut reg);
        reg
    }

    fn register<T: Module + 'static>(reg: &mut BTreeMap<Metadata, Initializer>) {
        if reg
            .insert(T::META, |m| Rc::new(RefCell::new(T::new(m))))
            .is_some()
        {
            panic!(
                "{} cannot be registered because another module already has load phase {:?} at priority {}.",
                std::any::type_name::<T>(),
                T::META.phase,
                T::META.priority
            );
        }
    }

    pub fn iter_phase(&self, phase: LoadPhase) -> impl Iterator<Item = &Initializer> + '_ {
        self.imp
            .iter()
            .filter(move |e| e.0.phase == phase)
            .map(|e| e.1)
    }
}
