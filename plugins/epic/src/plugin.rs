use crate::web::ClientAuthed;

use super::ui;
use super::User;
use fragile::Fragile;
use gtk::gdk_pixbuf::Pixbuf;
use plugins_core::prelude::*;
use std::sync::Arc;

pub struct Plugin {
    client: Arc<dyn core::Client>,
    user: Option<User>,
    web_client: Option<ClientAuthed>,
    image_icon: Fragile<Pixbuf>,
    image_banner: Fragile<Pixbuf>,
}

impl Plugin {
    pub fn new(client: Arc<dyn core::Client>) -> Self {
        Self {
            client,
            user: None,
            web_client: None,
            image_icon: Pixbuf::from_resource_at_scale(
                "/me/workingrobot/l4/epic/graphics/icon.svg",
                256,
                256,
                true,
            )
            .unwrap()
            .into(),
            image_banner: Pixbuf::from_resource_at_scale(
                "/me/workingrobot/l4/epic/graphics/banner.png",
                1920,
                1080,
                true,
            )
            .unwrap()
            .into(),
        }
    }
}

impl core::Identity for Plugin {
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

    fn image(&self, image_type: ImageType) -> Option<Pixbuf> {
        match image_type {
            ImageType::Icon => Some(self.image_icon.get().clone()),
            ImageType::Banner => Some(self.image_banner.get().clone()),
            _ => None,
        }
    }
}

#[async_trait]
impl core::Plugin for Plugin {
    fn client(&self) -> &dyn core::Client {
        self.client.as_ref()
    }

    async fn get_available_apps(&self) -> Option<Vec<Box<dyn core::App>>> {
        if self.get_user().await.is_some() {
            Some(vec![])
        } else {
            None
        }
    }

    async fn get_user(&self) -> Option<Box<dyn core::User>> {
        unimplemented!()
    }

    fn get_settings_widget(&self) -> adw::PreferencesGroup {
        ui::Settings::new().into()
    }
}
