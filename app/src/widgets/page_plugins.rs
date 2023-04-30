use gtk::glib;

mod imp {
    use std::cell::RefCell;

    use deps::plugins::PluginRegistry;
    use gtk::prelude::StaticTypeExt;
    use gtk::subclass::prelude::*;
    use gtk::{gio::ListStore, glib::subclass::InitializingObject};
    use gtk::{glib, CompositeTemplate};

    use crate::widgets::models;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/me/workingrobot/l4/templates/page_plugins.ui")]
    pub struct PagePlugins {
        registry: RefCell<PluginRegistry>,
        #[template_child]
        store: TemplateChild<ListStore>,
    }

    impl PagePlugins {
        pub fn init(&self) {
            self.load_plugins();

            for plugin in self.registry.borrow().iter_plugins() {
                self.store.append(&models::Plugin::new(plugin));
            }
        }

        fn load_plugins(&self) {
            let mut registry_mut = self.registry.borrow_mut();
            std::fs::read_dir(std::env::current_exe().unwrap().parent().unwrap())
                .unwrap()
                .flatten()
                .filter(|e| e.metadata().map_or(false, |m| m.is_file()))
                .filter(|e| {
                    e.file_name()
                        .to_str()
                        .map_or(false, |f| f.starts_with("plugins_") && f.ends_with(".dll"))
                })
                .for_each(|e| unsafe { registry_mut.load(e.path()) }.unwrap())
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PagePlugins {
        const NAME: &'static str = "L4PagePlugins";
        type Type = super::PagePlugins;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            models::Plugin::ensure_type();

            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl PagePlugins {}

    impl ObjectImpl for PagePlugins {
        fn constructed(&self) {
            self.init();

            self.parent_constructed();
        }
    }

    impl WidgetImpl for PagePlugins {}

    impl BoxImpl for PagePlugins {}
}

glib::wrapper! {
    pub struct PagePlugins(ObjectSubclass<imp::PagePlugins>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl PagePlugins {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
