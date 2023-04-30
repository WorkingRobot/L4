use glib::ParamSpecString;
use glib::{ParamSpec, Value};
use gtk::{
    gdk::Paintable, gdk::Texture, glib, glib::ParamSpecObject, prelude::*, subclass::prelude::*,
};
use once_cell::sync::Lazy;
use once_cell::unsync::OnceCell;
use plugins_core::prelude::*;
use std::sync::Weak;

pub struct PluginModel {
    plugin: OnceCell<Weak<dyn core::Plugin>>,
    icon_paintable: OnceCell<Paintable>,
}

impl PluginModel {
    pub fn set_plugin(&self, plugin: Weak<dyn core::Plugin>) {
        self.plugin.set(plugin.clone()).ok().unwrap();
        let plugin = plugin.upgrade().unwrap();
        self.icon_paintable
            .set(Texture::for_pixbuf(&plugin.image_with_fallback(ImageType::Icon)).into())
            .unwrap();
    }
}

#[glib::object_subclass]
impl ObjectSubclass for PluginModel {
    const NAME: &'static str = "L4PluginModel";
    type Type = super::PluginModel;

    fn new() -> Self {
        Self {
            plugin: Default::default(),
            icon_paintable: Default::default(),
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
                ParamSpecObject::builder::<Paintable>("icon-paintable")
                    .read_only()
                    .build(),
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
                "icon-paintable" => self.icon_paintable.get().unwrap().to_value(),
                _ => unimplemented!(),
            };
        }
        None::<&str>.to_value()
    }
}
