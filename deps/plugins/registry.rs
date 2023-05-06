use super::Client;
use plugins_core::{Plugin, PluginDeclaration};
use std::sync::Arc;

pub struct PluginRegistry {
    client: Arc<Client>,
    plugins: Vec<PluginHandle>,
}

struct PluginHandle {
    plugin: Arc<dyn Plugin>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Client {}),
            plugins: vec![],
        }
    }

    pub fn load(&mut self, decl: &'static PluginDeclaration) -> Result<(), gtk::glib::Error> {
        self.plugins
            .push(PluginHandle::new(decl, self.client.clone())?);

        Ok(())
    }

    pub fn iter_plugins(&self) -> impl Iterator<Item = Arc<dyn Plugin + 'static>> + '_ {
        self.plugins.iter().map(|p| p.plugin.clone())
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginHandle {
    fn new(
        decl: &'static PluginDeclaration,
        client: Arc<Client>,
    ) -> Result<Self, gtk::glib::Error> {
        if !decl.gresource.is_empty() {
            let bytes = gtk::glib::Bytes::from_static(decl.gresource);
            let resource = gtk::gio::Resource::from_data(&bytes)?;
            gtk::gio::resources_register(&resource);
        }

        let plugin = (decl.register)(client);

        Ok(Self { plugin })
    }
}
