use crate::{
    config::{Config, SavedUserCreds},
    ui::Settings,
    web::{
        responses::{DeviceAuth, GetAccount, OAuthTokenUser},
        Client, ClientAuthed, Credentials, Result,
    },
};
use adw::subclass::prelude::*;
use deps::utils::Dispatcher;
use fragile::Fragile;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::glib;
use once_cell::sync::Lazy;
use plugins_core::prelude::*;
use std::sync::{Arc, RwLock};

struct PluginGui {
    image_icon: Pixbuf,
    image_banner: Pixbuf,
    settings: Settings,
}

pub struct Plugin {
    client: Arc<dyn core::Client>,
    gui: Fragile<PluginGui>,
    config: Arc<RwLock<Config>>,
    auth_code_dispatcher: Dispatcher<SavedUserCreds>,
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
        let gui = self.gui.get();
        match image_type {
            ImageType::Icon => Some(gui.image_icon.clone()),
            ImageType::Banner => Some(gui.image_banner.clone()),
            _ => None,
        }
    }
}

impl core::Plugin for Plugin {
    fn new(client: Arc<impl plugins_core::Client + 'static>) -> Arc<Self>
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

        let mut this = Self {
            client,
            gui: PluginGui {
                image_icon: Pixbuf::from_resource_at_scale(
                    "/me/workingrobot/l4/epic/graphics/icon.svg",
                    256,
                    256,
                    true,
                )
                .unwrap(),
                image_banner: Pixbuf::from_resource_at_scale(
                    "/me/workingrobot/l4/epic/graphics/banner.png",
                    1920,
                    1080,
                    true,
                )
                .unwrap(),
                settings: Settings::new(),
            }
            .into(),
            config: Default::default(),
            auth_code_dispatcher: Default::default(),
        };

        let settings = &this.gui.get().settings;
        let config = &this.config;
        this.auth_code_dispatcher.connect(
            glib::clone!(@weak settings, @weak config => @default-return false, move |creds| {
                settings.imp().on_added_account(creds.clone());
                config.write().unwrap().users.push(creds);
                true
            }),
        );

        this.client
            .register_protocol(&this, "com.epicgames.fortnite")
            .unwrap();

        if let Some(saved_config) = this.client.get_storage(&this) {
            let mut config = this.config.write().unwrap();
            *config = from_value(saved_config).unwrap();
        }
        settings.imp().set_config(config.clone());

        Arc::new(this)
    }

    fn get_available_apps(&self) -> Option<Vec<Box<dyn core::App>>> {
        if self.get_user().is_some() {
            Some(vec![])
        } else {
            None
        }
    }

    fn get_user(&self) -> Option<Box<dyn core::User>> {
        todo!()
    }

    fn get_settings_widget(&self) -> adw::PreferencesGroup {
        self.gui.get().settings.clone().into()
    }

    fn on_protocol_callback(&self, data: &str) {
        if let Some(code) = Self::parse_fnauth(data) {
            tokio::spawn(Self::consume_authorization_code(
                self.auth_code_dispatcher.clone(),
                code,
            ));
        }
    }
}

impl Plugin {
    fn parse_fnauth(url: &str) -> Option<String> {
        let url = reqwest::Url::parse(url).ok()?;

        if url.scheme() != "com.epicgames.fortnite" {
            return None;
        }

        if url.domain() != Some("fnauth") {
            return None;
        }

        if url.path() != "/" {
            return None;
        }

        Some(
            url.query_pairs()
                .find_map(|pair| if pair.0 == "code" { Some(pair.1) } else { None })?
                .into_owned(),
        )
    }

    async fn create_creds_from_auth_code(code: String) -> Result<SavedUserCreds> {
        let client = Client::new()?;
        let token: OAuthTokenUser = client
            .oauth_authorization_code(Credentials::FortniteAndroid, &code)
            .await?;
        let authed_client = ClientAuthed::new(token, Credentials::FortniteAndroid)?;
        let account: GetAccount = authed_client.get_account().await?;
        let device_auth: DeviceAuth = authed_client.create_device_auth().await?;

        Ok(SavedUserCreds {
            account_id: account.id,
            display_name: account.display_name,
            avatar_id: "ATHENACHARACTER:CID_069_ATHENA_COMMANDO_F_PINKBEAR".into(),
            device_id: device_auth.device_id,
            secret: device_auth.secret.unwrap(),
        })
    }

    async fn consume_authorization_code(dispatcher: Dispatcher<SavedUserCreds>, code: String) {
        let result: Result<SavedUserCreds> = Self::create_creds_from_auth_code(code).await;
        match result {
            Ok(creds) => {
                dispatcher.emit(creds);
            }
            Err(err) => {
                println!("{err:?}");
            }
        };
    }
}

impl Drop for Plugin {
    fn drop(&mut self) {
        self.client
            .set_storage(self, to_value(&*self.config.read().unwrap()).unwrap())
    }
}

#[test]
fn fnauth() {
    assert_eq!(
        Plugin::parse_fnauth(
            "com.epicgames.fortnite://fnauth/?code=c920a24f4b87433787320b2db20b3f75"
        ),
        Some("c920a24f4b87433787320b2db20b3f75".into())
    );
}
