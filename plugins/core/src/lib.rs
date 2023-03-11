pub use async_trait::async_trait;
pub use semver::Version;

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub trait Environment {
    fn version(&self) -> Version;
}

#[async_trait]
pub trait App {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str; // TODO: allow markdown of some kind?
    fn short_description(&self) -> &str;
    fn environments(&self) -> Vec<&str>;

    async fn get_environment(&self) -> Box<dyn Environment>;
}

pub trait User {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn discriminator(&self) -> Option<&str>;
    fn region(&self) -> Option<&str>;
}

pub trait AuthSession {
    // TODO: continue work here, convert to async generator instead?
    fn is_complete(&self) -> bool;
}

pub trait Identity {
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
    async fn get_apps(&self) -> Vec<Box<dyn App>>;
    async fn get_user(&self) -> Option<Box<dyn User>>;
    async fn open_auth_session(&self) -> Option<Box<dyn AuthSession>>;
}

pub trait Client: Identity {}

#[derive(Copy, Clone)]
pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe fn(&dyn Client) -> Box<dyn Plugin>,
}

#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::PluginDeclaration = $crate::PluginDeclaration {
            rustc_version: $crate::RUSTC_VERSION,
            core_version: $crate::CORE_VERSION,
            register: $register,
        };
    };
}
