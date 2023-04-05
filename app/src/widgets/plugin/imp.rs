use glib::ParamSpecString;
use glib::{ParamSpec, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;
use once_cell::unsync::OnceCell;
use plugins_core::Plugin;
use std::sync::Weak;

pub struct PluginModel {
    plugin: OnceCell<Weak<dyn Plugin>>,
}

impl PluginModel {
    pub fn set_plugin(&self, plugin: Weak<dyn Plugin>) {
        self.plugin.set(plugin).ok().unwrap();
    }
}

#[glib::object_subclass]
impl ObjectSubclass for PluginModel {
    const NAME: &'static str = "L4PluginModel";
    type Type = super::PluginModel;

    fn new() -> Self {
        Self {
            plugin: Default::default(),
        }
    }
}

impl ObjectImpl for PluginModel {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("id").read_only().build(),
                ParamSpecString::builder("name").read_only().build(),
                ParamSpecString::builder("description").read_only().build(),
                ParamSpecString::builder("version").read_only().build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        let plugin = self.plugin.get().and_then(|w| w.upgrade());
        if let Some(plugin) = plugin {
            return match pspec.name() {
                "id" => plugin.id().to_value(),
                "name" => plugin.name().to_value(),
                "description" => plugin.description().to_value(),
                "version" => plugin.version().to_string().to_value(),
                _ => unimplemented!(),
            };
        }
        None::<&str>.to_value()
    }
}
