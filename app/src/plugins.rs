use libloading::Library;
use plugins_core::{Identity, Plugin, PluginDeclaration, Version};
use std::ffi::OsStr;
use std::sync::Arc;
use std::{env, io};

pub struct PluginRegistrar {
    client: Arc<Client>,
    plugins: Vec<(Box<dyn Plugin>, Library)>,
}

impl PluginRegistrar {
    pub fn new(client: Client) -> Self {
        Self {
            client: Arc::new(client),
            plugins: vec![],
        }
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

        let plugin = (decl.register)(self.client.clone());

        self.plugins.push((plugin, library));

        Ok(())
    }
}

pub struct Client;

impl Identity for Client {
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

impl plugins_core::Client for Client {}
