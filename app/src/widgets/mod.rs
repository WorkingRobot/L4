mod app_window;
pub mod models;
mod page_about;
mod page_games;
mod page_plugins;
mod page_settings;

pub use app_window::AppWindow;
pub use page_about::PageAbout;
pub use page_games::PageGames;
pub use page_plugins::PagePlugins;
pub use page_settings::PageSettings;

// @inner InnerName! implies added callbacks
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

pub(crate) use composite_widget;
