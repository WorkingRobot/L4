use super::item_model;
use gtk::{
    glib::{ParamSpec, ParamSpecString, Value},
    subclass::prelude::*,
};
use once_cell::sync::Lazy;
use plugins_core::prelude::*;
use std::sync::{Arc, Weak};

item_model!(Game, GameInner, "L4ModelGame", [game: Arc<dyn core::App>]);

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

impl ObjectImpl for imp::Game {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("id").read_only().build(),
                ParamSpecString::builder("name").read_only().build(),
                ParamSpecString::builder("description").read_only().build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        let inner = self.inner.get().unwrap();
        match pspec.name() {
            "id" => inner.id().unwrap().to_value(),
            "name" => inner.name().unwrap().to_value(),
            "description" => inner.description().unwrap().to_value(),
            _ => unimplemented!(),
        }
    }
}
