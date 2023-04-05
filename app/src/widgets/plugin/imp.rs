use glib::ParamSpecString;
use glib::{ParamSpec, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;
use plugins_core::{async_trait, App, AuthSession, Client, Identity, Plugin, User};
use std::cell::RefCell;
use std::sync::Arc;

struct FakePlugin;

impl Identity for FakePlugin {
    fn id(&self) -> &str {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn description(&self) -> &str {
        todo!()
    }

    fn version(&self) -> plugins_core::Version {
        todo!()
    }

    fn authors(&self) -> Vec<&str> {
        todo!()
    }

    fn repository_url(&self) -> &str {
        todo!()
    }

    fn license(&self) -> &str {
        todo!()
    }
}

#[async_trait]
impl Plugin for FakePlugin {
    fn client(&self) -> &dyn Client {
        unimplemented!()
    }

    async fn get_apps(&self) -> Vec<Box<dyn App>> {
        unimplemented!()
    }

    async fn get_user(&self) -> Option<Box<dyn User>> {
        unimplemented!()
    }

    async fn open_auth_session(&self) -> Option<Box<dyn AuthSession>> {
        unimplemented!()
    }
}

pub struct PluginModel {
    plugin: RefCell<Arc<dyn Plugin>>,
}

impl PluginModel {
    pub fn set_plugin(&self, plugin: Arc<dyn Plugin>) {
        *self.plugin.borrow_mut() = plugin;
    }
}

#[glib::object_subclass]
impl ObjectSubclass for PluginModel {
    const NAME: &'static str = "L4PluginModel";
    type Type = super::PluginModel;

    fn new() -> Self {
        Self {
            plugin: RefCell::new(Arc::new(FakePlugin {})),
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
        let plugin = self.plugin.borrow();
        return match pspec.name() {
            "id" => plugin.id().to_value(),
            "name" => plugin.name().to_value(),
            "description" => plugin.description().to_value(),
            "version" => plugin.version().to_string().to_value(),
            _ => unimplemented!(),
        };
    }
}
