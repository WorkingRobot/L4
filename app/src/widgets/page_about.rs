use gtk::glib;

mod imp {
    use gtk::glib::subclass::prelude::*;
    use gtk::glib::subclass::InitializingObject;
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/me/workingrobot/l4/templates/page_about.ui")]
    pub struct PageAbout {}

    #[glib::object_subclass]
    impl ObjectSubclass for PageAbout {
        const NAME: &'static str = "L4PageAbout";
        type Type = super::PageAbout;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl PageAbout {}

    impl ObjectImpl for PageAbout {}

    impl WidgetImpl for PageAbout {}

    impl BoxImpl for PageAbout {}
}

glib::wrapper! {
    pub struct PageAbout(ObjectSubclass<imp::PageAbout>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl PageAbout {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
