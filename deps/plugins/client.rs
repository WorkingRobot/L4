use plugins_core::{Client as ClientTrait, Identity, Version};

pub struct Client;

impl ClientTrait for Client {}

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
