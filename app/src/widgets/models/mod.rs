mod game;
mod plugin;

pub use game::Game;
pub use plugin::Plugin;

macro_rules! item_model {
    ($name:tt, $inner_name:tt, $glib_name:expr, ($($arg_name:ident: $arg_type:ty),* $(,)?), |$inner:ident| {$($param_spec:ident $(::<$param_turbofish:ty>)? ($prop_name:expr) => $prop_getter:expr),* $(,)?}) => {
        mod imp {
            use super::$inner_name as Inner;
            use once_cell::unsync::OnceCell;
            use gtk::glib;
            use gtk::subclass::prelude::*;
            use gtk::glib::{ParamSpec, Value};
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

            impl ObjectImpl for $name {
                fn properties() -> &'static [ParamSpec] {
                    static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                        vec![
                            $($param_spec::builder$(::<$param_turbofish>)?($prop_name).read_only().build(),)*
                        ]
                    });
                    PROPERTIES.as_ref()
                }

                fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
                    let $inner = self.inner.get().unwrap();
                    match pspec.name() {
                        $($prop_name => $prop_getter.to_value(),)*
                        _ => unimplemented!(),
                    }
                }
            }

            impl $name {
                pub(super) fn init(&self, $($arg_name: $arg_type),*) {
                    _ = self.inner.set(Inner::new($($arg_name),*));
                }
            }
        }

        gtk::glib::wrapper! {
            pub struct $name(ObjectSubclass<imp::$name>);
        }

        impl $name {
            pub fn new($($arg_name: $arg_type),*) -> Self {
                use gtk::subclass::prelude::ObjectSubclassExt;

                let this = gtk::glib::Object::builder().build();
                imp::$name::from_obj(&this).init($($arg_name),*);
                this
            }
        }
    };
}

pub(crate) use item_model;
