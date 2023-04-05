use super::module::*;
use gtk::prelude::*;
use gtk::{gdk, IconTheme};

pub struct UIPreInit {}

impl Module for UIPreInit {
    const INFO: ModuleInfo = ModuleInfo {
        name: "UI PreInit",
        phase: LoadPhase::Initialize,
        priority: 0,
    };

    fn new(_ctx: &impl ModuleCtx) -> Self {
        let display = gdk::Display::default().expect("Could not get a display");
        let icon_theme = IconTheme::for_display(&display);
        icon_theme.add_resource_path("/me/workingrobot/l4");
        icon_theme.add_resource_path("/com/fontawesome/icons");

        println!("-- searches");
        for search_path in icon_theme.search_path() {
            println!("{}", search_path.display());
        }
        println!("-- resources");
        for search_path in icon_theme.resource_path() {
            println!("{search_path}");
        }
        println!("-- end");

        // Register types
        crate::widgets::PluginModel::static_type();

        Self {}
    }
}
