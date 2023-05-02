use super::item_model;
use gtk::{glib::ParamSpecString, subclass::prelude::ObjectSubclassIsExt};
use once_cell::sync::Lazy;
use plugins_core::prelude::*;

item_model!(
    Setting,
    SettingInner,
    "L4ModelSetting",
    (name: String, icon_name: String),
    |inner| {
        ParamSpecString("name") => inner.name(),
        ParamSpecString("icon-name") => inner.icon_name(),
    }
);

struct SettingInner {
    name: String,
    icon_name: String,
}

impl SettingInner {
    fn new(name: String, icon_name: String) -> Self {
        Self { name, icon_name }
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn icon_name(&self) -> &str {
        self.icon_name.as_str()
    }
}

impl Setting {
    pub fn name(&self) -> &str {
        self.imp().inner.get().unwrap().name()
    }
}
