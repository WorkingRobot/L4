mod imp;

use std::sync::Arc;

use gtk::glib;
use gtk::subclass::prelude::{ObjectSubclassExt, ObjectSubclassType};
use plugins_core::Plugin;

glib::wrapper! {
    pub struct PluginModel(ObjectSubclass<imp::PluginModel>);
}

impl PluginModel {
    pub fn new(plugin: Arc<dyn Plugin>) -> Self {
        let this = glib::Object::builder().build();
        imp::PluginModel::from_obj(&this).set_plugin(plugin);
        this
    }
}
