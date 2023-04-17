#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

pub use async_trait::async_trait;
//use generator::Generator;
pub use semver::Version;
use std::path::Path;
use std::sync::Arc;

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

pub enum AuthStep {
    Complete(),
    Fatal(),
    Screen(),
}

pub type AuthSession<'a> = &'a i32; //Generator<'a, AuthStep, AuthStep>;

pub trait Identity: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn version(&self) -> Version;
    fn authors(&self) -> Vec<&str>;
    fn repository_url(&self) -> &str;
    fn license(&self) -> &str;
}

#[async_trait]
pub trait Plugin: Identity {
    fn client(&self) -> &dyn Client;

    async fn get_user(&self) -> Option<Box<dyn User>>;
    async fn open_auth_session(&self) -> Option<AuthSession>;

    async fn get_apps(&self) -> Option<Vec<Box<dyn App>>>;
}

pub trait Client: Identity {}

#[derive(Copy, Clone)]
pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe fn(client: Arc<dyn Client>) -> Arc<dyn Plugin>,
}

#[macro_export]
macro_rules! export_plugin {
    ($register:ty) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::PluginDeclaration = $crate::PluginDeclaration {
            rustc_version: $crate::RUSTC_VERSION,
            core_version: $crate::CORE_VERSION,
            register: |client| Arc::new(plugin::Plugin::new(client)),
        };
    };
}
