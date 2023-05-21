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
            client: Arc::new(Client::new("plugin-store.mp").unwrap()),
            plugins: vec![],
        }
    }

    pub fn load<T: Plugin + 'static>(&mut self) -> Result<(), gtk::glib::Error> {
        let plugin = T::new(self.client.clone());

        self.plugins.push(Arc::new(plugin));

        Ok(())
    }

    pub fn iter_plugins(&self) -> impl Iterator<Item = Arc<dyn Plugin + 'static>> + '_ {
        self.plugins.iter().cloned()
    }

    pub fn on_protocol_callback(&self, plugin_id: &str, data: &str) {
        if let Some(plugin) = self.plugins.iter().find(|p| p.id() == plugin_id) {
            plugin.on_protocol_callback(data);
        }
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
