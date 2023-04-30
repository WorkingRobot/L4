use gtk::glib;

mod imp {
    use gtk::glib::subclass::prelude::*;
    use gtk::glib::subclass::InitializingObject;
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/me/workingrobot/l4/templates/page_settings.ui")]
    pub struct PageSettings {}

    #[glib::object_subclass]
    impl ObjectSubclass for PageSettings {
        const NAME: &'static str = "L4PageSettings";
        type Type = super::PageSettings;
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
    impl PageSettings {}

    impl ObjectImpl for PageSettings {}

    impl WidgetImpl for PageSettings {}

    impl BoxImpl for PageSettings {}
}

glib::wrapper! {
    pub struct PageSettings(ObjectSubclass<imp::PageSettings>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl PageSettings {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
