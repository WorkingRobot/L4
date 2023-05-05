use deps::utils::item_model;
use gtk::{
    gdk::{Paintable, Texture},
    glib::{ParamSpecObject, ParamSpecString},
};
use once_cell::sync::Lazy;
use plugins_core::prelude::*;
use std::sync::{Arc, Weak};

item_model!(
    Plugin,
    PluginInner,
    "L4ModelPlugin",
    (game: Arc<dyn core::Plugin>),
    |inner| {
        ParamSpecString("id") => inner.id(),
        ParamSpecString("name") => inner.name(),
        ParamSpecString("description") => inner.description(),
        ParamSpecString("version") => inner.version().map(|v| v.to_string()),
        ParamSpecObject::<Paintable>("icon-paintable") => inner.icon_paintable()
    }
);

pub struct PluginInner {
    plugin: Weak<dyn core::Plugin>,
    icon_paintable: Paintable,
}

impl PluginInner {
    fn new(plugin: Arc<dyn core::Plugin>) -> Self {
        Self {
            plugin: Arc::downgrade(&plugin),
            icon_paintable: Texture::for_pixbuf(&plugin.image_with_fallback(ImageType::Icon))
                .into(),
        }
    }

    pub fn plugin(&self) -> Option<Arc<dyn core::Plugin>> {
        self.plugin.upgrade()
    }

    fn id(&self) -> Option<String> {
        self.plugin.upgrade().map(|p| p.id().to_string())
    }

    fn name(&self) -> Option<String> {
        self.plugin.upgrade().map(|p| p.name().to_string())
    }

    fn description(&self) -> Option<String> {
        self.plugin.upgrade().map(|p| p.description().to_string())
    }

    fn version(&self) -> Option<Version> {
        self.plugin.upgrade().map(|p| p.version())
    }

    fn icon_paintable(&self) -> &Paintable {
        &self.icon_paintable
    }
}
