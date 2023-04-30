use gtk::{glib, subclass::prelude::ObjectSubclassExt};
use plugins_core::prelude::*;
use std::sync::Arc;

mod imp {
    use gtk::glib::{self, Value};
    use gtk::{glib::ParamSpec, prelude::*};
    use gtk::{glib::ParamSpecString, subclass::prelude::*};
    use once_cell::sync::Lazy;
    use once_cell::unsync::OnceCell;
    use plugins_core::prelude::*;
    use std::sync::{Arc, Weak};

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

    #[derive(Default)]
    pub struct Game {
        inner: OnceCell<GameInner>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Game {
        const NAME: &'static str = "L4ModelGame";
        type Type = super::Game;
    }

    impl Game {
        pub fn init(&self, game: Arc<dyn core::App>) {
            _ = self.inner.set(GameInner::new(game));
        }
    }

    impl ObjectImpl for Game {
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
}

glib::wrapper! {
    pub struct Game(ObjectSubclass<imp::Game>);
}

impl Game {
    pub fn new(game: Arc<dyn core::App>) -> Self {
        let this = glib::Object::builder().build();
        imp::Game::from_obj(&this).init(game);
        this
    }
}
