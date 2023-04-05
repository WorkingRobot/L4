use super::*;
use once_cell::unsync::Lazy;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

type ModuleInitializer = fn(&ModuleList) -> Rc<dyn std::any::Any>;

pub struct Registry {
    imp: Lazy<BTreeMap<ModuleMeta, ModuleInitializer>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            imp: Lazy::new(Self::create),
        }
    }

    fn create() -> BTreeMap<ModuleMeta, ModuleInitializer> {
        let mut reg = BTreeMap::new();
        Self::register::<UIPreInit>(&mut reg);
        Self::register::<UIPostInit>(&mut reg);
        Self::register::<TitleButtons>(&mut reg);
        Self::register::<Plugins>(&mut reg);
        reg
    }

    fn register<T: Module + 'static>(reg: &mut BTreeMap<ModuleMeta, ModuleInitializer>) {
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

    pub fn iter_phase(&self, phase: LoadPhase) -> impl Iterator<Item = &ModuleInitializer> + '_ {
        self.imp
            .iter()
            .filter(move |e| e.0.phase == phase)
            .map(|e| e.1)
    }
}
