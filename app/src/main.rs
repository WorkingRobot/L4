use libloading::Library;
use plugins_core::{Client, Identity, Plugin, PluginDeclaration, Version};
use std::ffi::OsStr;
use std::{env, io};

#[derive(Default)]
pub struct PluginRegistrar {
    plugins: Vec<(Box<dyn Plugin>, Library)>,
}

impl PluginRegistrar {
    pub fn new() -> Self {
        Self::default()
    }

    pub unsafe fn load<P: AsRef<OsStr>>(
        &mut self,
        filename: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let library = Library::new(filename)?;

        let decl = library
            .get::<*mut PluginDeclaration>(b"plugin_declaration\0")?
            .read();

        if decl.rustc_version != plugins_core::RUSTC_VERSION
            || decl.core_version != plugins_core::CORE_VERSION
        {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "ABI version mismatch",
            )));
        }

        let plugin = (decl.register)(self);

        self.plugins.push((plugin, library));

        Ok(())
    }
}

impl Identity for PluginRegistrar {
    fn id(&self) -> &str {
        env!("CARGO_PKG_NAME")
    }

    fn name(&self) -> &str {
        "L4"
    }

    fn description(&self) -> &str {
        env!("CARGO_PKG_DESCRIPTION")
    }

    fn version(&self) -> Version {
        Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
    }

    fn authors(&self) -> Vec<&str> {
        env!("CARGO_PKG_AUTHORS").split(':').collect()
    }

    fn repository_url(&self) -> &str {
        env!("CARGO_PKG_REPOSITORY")
    }

    fn license(&self) -> &str {
        env!("CARGO_PKG_LICENSE")
    }
}

impl Client for PluginRegistrar {}

fn main() {}
