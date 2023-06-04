#[macro_export]
macro_rules! item_model {
    ($name:tt, $inner_name:tt, $glib_name:expr, ($($arg_name:ident: $arg_type:ty),* $(,)?), |$inner:ident| {$($param_spec:ident $(::<$param_turbofish:ty>)? ($prop_name:expr) => $prop_getter:expr),* $(,)?}) => {
        mod imp {
            use super::$inner_name as Inner;
            use gtk::glib;
            use gtk::subclass::prelude::*;
            use gtk::glib::{ParamSpec, Value};
            use std::ops::Deref;
            use std::cell::OnceCell;
            use std::sync::LazyLock;
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
                    static PROPERTIES: LazyLock<Vec<ParamSpec>> = LazyLock::new(|| {
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

            impl AsRef<Inner> for $name {
                fn as_ref(&self) -> &Inner {
                    self.deref()
                }
            }

            impl Deref for $name {
                type Target = Inner;

                fn deref(&self) -> &Inner {
                    self.inner.get().unwrap()
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

#[macro_export]
macro_rules! subclassed_gobject {
    ($name:tt => $glib_name:expr, @inner $inner_name:ident, @parent $parent:ty, @extends $($extend_types:ty),*, @implements $($impl_types:ty),* $(,)?) => {
        #[glib::object_subclass]
        impl ObjectSubclass for $inner_name {
            const NAME: &'static str = $glib_name;
            type Type = $name;
            type ParentType = $parent;
        }

        glib::wrapper! {
            pub struct $name(ObjectSubclass<$inner_name>)
                @extends $($extend_types),*,
                @implements $($impl_types),*;
        }
    };
}

// @inner InnerName! implies added callbacks
#[macro_export]
macro_rules! composite_widget {
    ($name:tt => $glib_name:expr, @inner $inner_name:ident, @parent $parent:ty, @extends $($extend_types:ty),*, @implements $($impl_types:ty),*$(, @uses $($child_types:ty),* $(,)?)?) => {
        #[glib::object_subclass]
        impl ObjectSubclass for $inner_name {
            const NAME: &'static str = $glib_name;
            type Type = $name;
            type ParentType = $parent;

            fn class_init(klass: &mut Self::Class) {
                #[allow(unused_imports)]
                use gtk::prelude::StaticTypeExt;

                $($(<$child_types>::ensure_type();)*)?

                klass.bind_template();
            }

            fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
                obj.init_template();
            }
        }

        glib::wrapper! {
            pub struct $name(ObjectSubclass<$inner_name>)
                @extends $($extend_types),*,
                @implements $($impl_types),*;
        }
    };

    ($name:tt => $glib_name:expr, @inner $inner_name:ident!, @parent $parent:ty, @extends $($extend_types:ty),*, @implements $($impl_types:ty),*$(, @uses $($child_types:ty),* $(,)?)?) => {
        #[glib::object_subclass]
        impl ObjectSubclass for $inner_name {
            const NAME: &'static str = $glib_name;
            type Type = $name;
            type ParentType = $parent;

            fn class_init(klass: &mut Self::Class) {
                #[allow(unused_imports)]
                use gtk::prelude::StaticTypeExt;

                $($(<$child_types>::ensure_type();)*)?

                klass.bind_template();
                klass.bind_template_callbacks();
            }

            fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
                obj.init_template();
            }
        }

        glib::wrapper! {
            pub struct $name(ObjectSubclass<$inner_name>)
                @extends $($extend_types),*,
                @implements $($impl_types),*;
        }
    };
}

pub use {composite_widget, item_model, subclassed_gobject};
