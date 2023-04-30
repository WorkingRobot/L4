use super::item_model;
use gtk::{
    gdk::{Paintable, Texture},
    glib::{ParamSpec, ParamSpecObject, ParamSpecString, Value},
    subclass::prelude::*,
};
use once_cell::sync::Lazy;
use plugins_core::prelude::*;
use std::sync::{Arc, Weak};

item_model!(
    Plugin,
    PluginInner,
    "L4ModelPlugin",
    [game: Arc<dyn core::Plugin>]
);

struct PluginInner {
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

impl ObjectImpl for imp::Plugin {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("id").read_only().build(),
                ParamSpecString::builder("name").read_only().build(),
                ParamSpecString::builder("description").read_only().build(),
                ParamSpecString::builder("version").read_only().build(),
                ParamSpecObject::builder::<Paintable>("icon-paintable")
                    .read_only()
                    .build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        let inner = self.inner.get().unwrap();
        match pspec.name() {
            "id" => inner.id().to_value(),
            "name" => inner.name().to_value(),
            "description" => inner.description().to_value(),
            "version" => inner.version().unwrap().to_string().to_value(),
            "icon-paintable" => inner.icon_paintable().to_value(),
            _ => None::<&str>.to_value(),
        }
    }
}
