use gtk::glib;

mod imp {
    use gtk::glib::subclass::InitializingObject;
    use gtk::prelude::StaticTypeExt;
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate};

    use crate::widgets::models;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/me/workingrobot/l4/templates/page_games.ui")]
    pub struct PageGames {}

    #[glib::object_subclass]
    impl ObjectSubclass for PageGames {
        const NAME: &'static str = "L4PageGames";
        type Type = super::PageGames;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            models::Game::ensure_type();

            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl PageGames {}

    impl ObjectImpl for PageGames {}

    impl WidgetImpl for PageGames {}

    impl BoxImpl for PageGames {}
}

glib::wrapper! {
    pub struct PageGames(ObjectSubclass<imp::PageGames>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl PageGames {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
