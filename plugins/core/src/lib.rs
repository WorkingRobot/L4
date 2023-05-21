#![feature(async_fn_in_trait)]
#![feature(trait_alias)]
#![allow(incomplete_features)]

use async_trait::async_trait;
use gtk::{
    self,
    gdk_pixbuf::{Colorspace, Pixbuf},
};
use semver::Version;
use static_assertions::assert_obj_safe;
use std::{path::Path, sync::Arc};
pub mod prelude;

pub trait InstalledApp {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn version(&self) -> Version;
    fn environment(&self) -> &str;
    fn install_location(&self) -> &Path;
}

pub trait App {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn environments(&self) -> Vec<&str>;
}

pub trait User {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn discriminator(&self) -> Option<&str>;
    fn region(&self) -> Option<&str>;
}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum ImageType {
    Icon,   // Aspect: 1:1, Size: 256x256
    Banner, // Aspect 16:9, Size: 1920x1080
}

pub trait Identity: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn version(&self) -> &'static Version;
    fn authors(&self) -> &'static [&'static str];
    fn repository_url(&self) -> &'static str;
    fn license(&self) -> &'static str;
    fn image(&self, image_type: ImageType) -> Option<Pixbuf>;

    fn image_with_fallback(&self, image_type: ImageType) -> Pixbuf {
        self.image(image_type)
            .or_else(|| {
                let image_size = match image_type {
                    ImageType::Icon => (256, 256),
                    ImageType::Banner => (1920, 1080),
                };
                Pixbuf::new(Colorspace::Rgb, true, 32, image_size.0, image_size.1)
            })
            .unwrap()
    }
}

#[async_trait]
pub trait Plugin: Identity {
    fn new(client: Arc<impl Client + 'static>) -> Self
    where
        Self: Sized;

    async fn get_user(&self) -> Option<Box<dyn User>>;
    fn get_settings_widget(&self) -> adw::PreferencesGroup;
    fn on_protocol_callback(&self, data: &str);

    async fn get_available_apps(&self) -> Option<Vec<Box<dyn App>>>;
}

pub trait Client: Identity {
    fn register_protocol(&self, plugin: &dyn Plugin, schema: &str) -> std::io::Result<()>;

    fn get_storage(&self, plugin: &dyn Plugin) -> Option<rmpv::Value>;

    fn set_storage(&self, plugin: &dyn Plugin, data: rmpv::Value);
}

assert_obj_safe!(Identity, Plugin, Client, User, App, InstalledApp);
