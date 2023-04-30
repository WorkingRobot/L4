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
        self.plugins
            .push(PluginHandle::new(file_path, self.client.clone())?);

        Ok(())
    }

    pub fn iter_plugins(&self) -> impl Iterator<Item = Weak<dyn Plugin>> + '_ {
        self.plugins.iter().map(|p| Arc::downgrade(&p.plugin))
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginHandle {
    unsafe fn new<P: AsRef<Path>>(
        file_path: P,
        client: Arc<Client>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
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

        if !decl.gresource.is_empty() {
            let bytes = gtk::glib::Bytes::from_static(decl.gresource);
            let resource = gtk::gio::Resource::from_data(&bytes)?;
            gtk::gio::resources_register(&resource);
        }

        let plugin = (decl.register)(client);

        Ok(Self {
            plugin,
            _library: library,
        })
    }
}
