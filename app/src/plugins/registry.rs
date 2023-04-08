use super::Client;
use libloading::Library;
use plugins_core::{Plugin, PluginDeclaration};
use std::sync::{Arc, Weak};
use std::{io, path::Path};

pub struct PluginRegistry {
    client: Arc<Client>,
    plugins: Vec<PluginHandle>,
}

struct PluginHandle {
    plugin: Arc<dyn Plugin>,
    _library: Library,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Client {}),
            plugins: vec![],
        }
    }

    pub unsafe fn load<P: AsRef<Path>>(
        &mut self,
        file_path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let library = Library::new(file_path.as_ref().as_os_str())?;

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

        self.plugins.push(PluginHandle {
            plugin,
            _library: library,
        });

        Ok(())
    }

    pub fn iter_plugins(&self) -> impl Iterator<Item = Weak<dyn Plugin>> + '_ {
        self.plugins.iter().map(|p| Arc::downgrade(&p.plugin))
    }
}
