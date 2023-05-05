#![feature(async_fn_in_trait)]
#![feature(trait_alias)]
#![allow(incomplete_features)]

use async_trait::async_trait;
use gtk::{
    self,
    gdk_pixbuf::{Colorspace, Pixbuf},
};
use semver::Version;
use std::path::Path;
use std::sync::Arc;
pub mod prelude;

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

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
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn version(&self) -> Version;
    fn authors(&self) -> Vec<&str>;
    fn repository_url(&self) -> &str;
    fn license(&self) -> &str;
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
    fn client(&self) -> &dyn Client;

    async fn get_user(&self) -> Option<Box<dyn User>>;
    fn get_settings_widget(&self) -> adw::PreferencesGroup;

    async fn get_available_apps(&self) -> Option<Vec<Box<dyn App>>>;
}

pub trait Client: Identity {}

#[derive(Copy, Clone)]
pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub gresource: &'static [u8],
    pub register: unsafe fn(client: Arc<dyn Client>) -> Arc<dyn Plugin>,
}

#[macro_export]
macro_rules! export_plugin {
    ($register:ty, $gresource_path:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::PluginDeclaration = $crate::PluginDeclaration {
            rustc_version: $crate::RUSTC_VERSION,
            core_version: $crate::CORE_VERSION,
            gresource: include_bytes!(concat!(env!("OUT_DIR"), "/", $gresource_path)),
            register: |client| Arc::new(plugin::Plugin::new(client)),
        };
    };
}
