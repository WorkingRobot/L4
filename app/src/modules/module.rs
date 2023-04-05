use gtk::glib;
use gtk::prelude::*;
use std::cmp::Ordering;

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

    fn get_application(&self) -> gtk::Application;
}

pub trait ModuleInst {}

pub trait Module: Sized {
    const INFO: ModuleInfo;

    fn new(ctx: &impl ModuleCtx) -> Self;
}

impl<T: Module> ModuleInst for T {}

#[derive(Eq)]
pub struct ModuleInfo {
    pub name: &'static str,
    pub phase: LoadPhase,
    pub priority: u16,
}

impl Ord for ModuleInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.phase
            .cmp(&other.phase)
            .then(self.priority.cmp(&other.priority))
    }
}

impl PartialOrd for ModuleInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ModuleInfo {
    fn eq(&self, other: &Self) -> bool {
        self.phase == other.phase && self.priority == other.priority
    }
}
