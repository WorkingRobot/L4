use super::module::*;
use crate::{plugins::PluginRegistrar, widgets::PluginModel};

pub struct Plugins {
    registrar: PluginRegistrar,
}

impl Plugins {
    fn load_plugins(&mut self) {
        std::fs::read_dir(std::env::current_exe().unwrap().parent().unwrap())
            .unwrap()
            .flatten()
            .filter(|e| e.metadata().map_or(false, |m| m.is_file()))
            .filter(|e| {
                e.file_name()
                    .to_str()
                    .map_or(false, |f| f.starts_with("plugins_") && f.ends_with(".dll"))
            })
            .for_each(|e| unsafe { self.registrar.load(e.path()) }.unwrap())
    }
}

impl Module for Plugins {
    const INFO: ModuleInfo = ModuleInfo {
        name: "Plugin Registrar",
        phase: LoadPhase::UILoad,
        priority: 2,
    };

    fn new(ctx: &impl ModuleCtx) -> Self {
        let mut this = Self {
            registrar: PluginRegistrar::new(),
        };

        this.load_plugins();

        let store = ctx.get_object::<gtk::gio::ListStore>("games-plugin-store");
        for plugin in this.registrar.iter_plugins() {
            store.append(&PluginModel::new(plugin));
        }

        this
    }
}
