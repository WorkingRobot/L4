use super::module::*;
use gtk::prelude::*;
use gtk::{gdk, IconTheme};

pub struct UIPreInit;

impl Module for UIPreInit {
    const META: Metadata = Metadata {
        phase: LoadPhase::Initialize,
        priority: 0,
    };

    fn new(_ctx: &impl ModuleCtx) -> Self {
        let display = gdk::Display::default().expect("Could not get a display");
        let icon_theme = IconTheme::for_display(&display);
        icon_theme.add_resource_path("/me/workingrobot/l4");
        icon_theme.add_resource_path("/com/fontawesome/icons");

        // Register types
        crate::widgets::PluginModel::static_type();

        Self {}
    }
}
