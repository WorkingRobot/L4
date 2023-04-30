use super::item_model;
use gtk::glib::ParamSpecString;
use once_cell::sync::Lazy;
use plugins_core::prelude::*;
use std::sync::{Arc, Weak};

item_model!(
    Game,
    GameInner,
    "L4ModelGame",
    (game: Arc<dyn core::App>),
    |inner| {
        ParamSpecString("id") => inner.id(),
        ParamSpecString("name") => inner.name(),
        ParamSpecString("description") => inner.description(),
    }
);

struct GameInner {
    game: Weak<dyn core::App>,
}

impl GameInner {
    fn new(game: Arc<dyn core::App>) -> Self {
        Self {
            game: Arc::downgrade(&game),
        }
    }

    fn id(&self) -> Option<String> {
        self.game.upgrade().map(|p| p.id().to_string())
    }

    fn name(&self) -> Option<String> {
        self.game.upgrade().map(|p| p.name().to_string())
    }

    fn description(&self) -> Option<String> {
        self.game.upgrade().map(|p| p.description().to_string())
    }
}
