mod imp;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::ObjectSubclassExt;
use plugins_core::Plugin;
use std::sync::Weak;

glib::wrapper! {
    pub struct PluginModel(ObjectSubclass<imp::PluginModel>);
}

impl PluginModel {
    pub fn new(plugin: Weak<dyn Plugin>) -> Self {
        let this = Object::builder().build();
        imp::PluginModel::from_obj(&this).set_plugin(plugin);
        this
    }
}
