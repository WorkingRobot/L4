use super::*;
use once_cell::unsync::Lazy;
use std::cell::RefCell;
use std::collections::{
    btree_map::Entry::{Occupied, Vacant},
    BTreeMap,
};
use std::rc::Rc;

type ModuleInitializer = fn(&ModuleList) -> Rc<RefCell<dyn ModuleInst>>;

pub struct Registry {
    imp: Lazy<BTreeMap<ModuleInfo, ModuleInitializer>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            imp: Lazy::new(Self::create),
        }
    }

    fn create() -> BTreeMap<ModuleInfo, ModuleInitializer> {
        let mut reg = BTreeMap::new();
        Self::register::<UIPreInit>(&mut reg);
        Self::register::<UIPostInit>(&mut reg);
        Self::register::<TitleButtons>(&mut reg);
        Self::register::<Plugins>(&mut reg);
        reg
    }

    fn register<T: Module + 'static>(reg: &mut BTreeMap<ModuleInfo, ModuleInitializer>) {
        match reg.entry(T::INFO) {
            Vacant(entry) => entry.insert(|m| Rc::new(RefCell::new(T::new(m)))),
            Occupied(entry) => panic!(
                "The modules \"{}\" and \"{}\" both have a load phase {:#?} and priority {}.",
                entry.key().name,
                T::INFO.name,
                T::INFO.phase,
                T::INFO.priority
            ),
        };
    }

    pub fn iter_phase(
        &self,
        phase: LoadPhase,
    ) -> impl Iterator<Item = (&ModuleInfo, &ModuleInitializer)> + '_ {
        self.imp.iter().filter(move |e| e.0.phase == phase)
    }
}
