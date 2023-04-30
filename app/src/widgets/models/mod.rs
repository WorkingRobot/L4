mod game;
mod plugin;

pub use game::Game;
pub use plugin::Plugin;

macro_rules! item_model {
    ($name:tt, $inner_name:tt, $glib_name:expr, [$($arg_name:ident: $arg_type:ty)*]) => {
        mod imp {
            use super::$inner_name as Inner;
            use once_cell::unsync::OnceCell;
            use gtk::glib;
            use gtk::subclass::prelude::ObjectSubclass;
            use super::*;

            #[derive(Default)]
            pub struct $name {
                pub(super) inner: OnceCell<Inner>,
            }

            #[glib::object_subclass]
            impl ObjectSubclass for $name {
                const NAME: &'static str = $glib_name;
                type Type = super::$name;
            }

            impl $name {
                pub(super) fn init(&self, $($arg_name: $arg_type)*) {
                    _ = self.inner.set(Inner::new($($arg_name)*));
                }
            }
        }

        gtk::glib::wrapper! {
            pub struct $name(ObjectSubclass<imp::$name>);
        }

        impl $name {
            pub fn new($($arg_name: $arg_type)*) -> Self {
                use gtk::subclass::prelude::ObjectSubclassExt;

                let this = gtk::glib::Object::builder().build();
                imp::$name::from_obj(&this).init($($arg_name)*);
                this
            }
        }
    };
}

pub(crate) use item_model;
