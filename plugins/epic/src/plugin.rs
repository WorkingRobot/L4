use super::ui;
use crate::{user::User, web::ClientAuthed};
use fragile::Fragile;
use gtk::gdk_pixbuf::Pixbuf;
use once_cell::sync::Lazy;
use plugins_core::prelude::*;
use std::sync::Arc;

pub struct Plugin {
    client: Arc<dyn core::Client>,
    user: Option<User>,
    web_client: Option<ClientAuthed>,
    image_icon: Fragile<Pixbuf>,
    image_banner: Fragile<Pixbuf>,
}

impl core::Identity for Plugin {
    fn id(&self) -> &'static str {
        "epic"
    }

    fn name(&self) -> &'static str {
        "Epic Games"
    }

    fn description(&self) -> &'static str {
        "Epic Games Store Integration"
    }

    fn version(&self) -> &'static Version {
        static VERSION: Lazy<Version> =
            Lazy::new(|| Version::parse(env!("CARGO_PKG_VERSION")).unwrap());
        &VERSION
    }

    fn authors(&self) -> &'static [&'static str] {
        static AUTHORS: Lazy<Vec<&str>> =
            Lazy::new(|| env!("CARGO_PKG_AUTHORS").split(':').collect());
        &AUTHORS
    }

    fn repository_url(&self) -> &'static str {
        env!("CARGO_PKG_REPOSITORY")
    }

    fn license(&self) -> &'static str {
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
    fn new(client: Arc<impl plugins_core::Client + 'static>) -> Self
    where
        Self: Sized,
    {
        {
            let bytes = gtk::glib::Bytes::from_static(include_bytes!(concat!(
                env!("OUT_DIR"),
                "/epic.gresource"
            )));
            let resource = gtk::gio::Resource::from_data(&bytes).unwrap();
            gtk::gio::resources_register(&resource);
        }

        let this = Self {
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
        };

        this.client
            .register_protocol(&this, "com.epicgames.fortnite")
            .unwrap();

        this
    }

    async fn get_available_apps(&self) -> Option<Vec<Box<dyn core::App>>> {
        if self.get_user().await.is_some() {
            Some(vec![])
        } else {
            None
        }
    }

    async fn get_user(&self) -> Option<Box<dyn core::User>> {
        todo!()
    }

    fn get_settings_widget(&self) -> adw::PreferencesGroup {
        ui::Settings::new().into()
    }

    fn on_protocol_callback(&self, _data: &str) {
        println!("protocol callback: {_data}");
        todo!()
    }
}
