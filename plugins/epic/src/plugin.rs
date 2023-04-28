use crate::web::ClientAuthed;

use super::User;
use plugins_core::{async_trait, App, AuthSession, Client, Version};
use std::sync::Arc;

pub struct Plugin {
    client: Arc<dyn Client>,
    user: Option<User>,
    web_client: Option<ClientAuthed>,
}

impl Plugin {
    pub fn new(client: Arc<dyn Client>) -> Self {
        Self {
            client,
            user: None,
            web_client: None,
        }
    }
}

impl plugins_core::Identity for Plugin {
    fn id(&self) -> &str {
        "epic"
    }

    fn name(&self) -> &str {
        "Epic Games"
    }

    fn description(&self) -> &str {
        "Epic Games Store Integration"
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

#[async_trait]
impl plugins_core::Plugin for Plugin {
    fn client(&self) -> &dyn Client {
        self.client.as_ref()
    }

    async fn get_available_apps(&self) -> Option<Vec<Box<dyn App>>> {
        if self.get_user().await.is_some() {
            Some(vec![])
        } else {
            None
        }
    }

    async fn get_user(&self) -> Option<Box<dyn plugins_core::User>> {
        unimplemented!()
    }

    async fn open_auth_session(&self) -> Option<AuthSession> {
        unimplemented!()
    }
}
