use super::Client;
use plugins_core::Plugin;
use std::sync::Arc;

pub struct PluginRegistry {
    client: Arc<Client>,
    plugins: Vec<Arc<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Client {}),
            plugins: vec![],
        }
    }

    pub fn load<T: Plugin + 'static>(&mut self) -> Result<(), gtk::glib::Error> {
        let plugin = T::new(self.client.clone());

        self.plugins.push(Arc::new(plugin));

        Ok(())
    }

    pub fn iter_plugins(&self) -> impl Iterator<Item = Arc<dyn Plugin + 'static>> + '_ {
        self.plugins.iter().map(|p| p.clone())
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
