use crate::utils::{register_protocol, Storage};
use gtk::gdk_pixbuf::Pixbuf;
use plugins_core::prelude::*;
use std::{
    path::Path,
    sync::{LazyLock, RwLock},
};

pub struct Client {
    storage: RwLock<Storage>,
}

impl Client {
    pub fn new<P: AsRef<Path>>(storage_path: P) -> Result<Self, crate::utils::Error> {
        Ok(Self {
            storage: Storage::new(storage_path)?.into(),
        })
    }
}

impl core::Client for Client {
    fn register_protocol(&self, plugin: &dyn core::Plugin, schema: &str) -> std::io::Result<()> {
        register_protocol(schema, format!("proto-{}", plugin.id()).as_str())
    }

    fn get_storage(&self, plugin: &dyn plugins_core::Plugin) -> Option<rmpv::Value> {
        self.storage.read().unwrap().get_raw(plugin).cloned()
    }

    fn set_storage(&self, plugin: &dyn plugins_core::Plugin, data: rmpv::Value) {
        self.storage.write().unwrap().set_raw(plugin, data)
    }
}

impl core::Identity for Client {
    fn id(&self) -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    fn name(&self) -> &'static str {
        "L4"
    }

    fn description(&self) -> &'static str {
        env!("CARGO_PKG_DESCRIPTION")
    }

    fn version(&self) -> &'static Version {
        static VERSION: LazyLock<Version> =
            LazyLock::new(|| Version::parse(env!("CARGO_PKG_VERSION")).unwrap());
        &VERSION
    }

    fn authors(&self) -> &'static [&'static str] {
        static AUTHORS: LazyLock<Vec<&str>> =
            LazyLock::new(|| env!("CARGO_PKG_AUTHORS").split(':').collect());
        &AUTHORS
    }

    fn repository_url(&self) -> &'static str {
        env!("CARGO_PKG_REPOSITORY")
    }

    fn license(&self) -> &'static str {
        env!("CARGO_PKG_LICENSE")
    }

    fn image(&self, _image_type: ImageType) -> Option<Pixbuf> {
        None
    }
}
