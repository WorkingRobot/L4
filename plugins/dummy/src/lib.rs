use plugins_core::*;

plugins_core::export_plugin!(register);

fn register(_client: &dyn Client) -> Box<dyn plugins_core::Plugin> {
    Box::new(Plugin)
}

pub struct Plugin;

impl plugins_core::Identity for Plugin {
    fn id(&self) -> &str {
        "dummy"
    }

    fn name(&self) -> &str {
        "Dummy!"
    }

    fn description(&self) -> &str {
        "A dummy plugin that does nothing"
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

impl plugins_core::Plugin for Plugin {
    async fn get_apps(&self) -> Vec<Box<dyn App>> {
        Default::default()
    }

    async fn get_user(&self) -> Option<Box<dyn User>> {
        None
    }

    async fn open_auth_session(&self) -> Option<Box<dyn AuthSession>> {
        None
    }
}
