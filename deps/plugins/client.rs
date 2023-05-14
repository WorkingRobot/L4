use gtk::gdk_pixbuf::Pixbuf;
use once_cell::sync::Lazy;
use plugins_core::prelude::*;

pub struct Client;

impl core::Client for Client {}

impl core::Identity for Client {
    fn id(&self) -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    fn name(&self) -> &'static str {
        "L4"
    }

    fn description(&self) -> &'static str {
        env!("CARGO_PKG_DESCRIPTION")
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

    fn image(&self, _image_type: ImageType) -> Option<Pixbuf> {
        None
    }
}
