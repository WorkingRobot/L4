use super::prelude::*;
use deps::utils::{signal, SignalHolder, UsesDpi};
use gtk::prelude::*;

pub struct UIPostInit {
    _holder_realize: SignalHolder,
}

impl Module for UIPostInit {
    const META: Metadata = Metadata {
        phase: LoadPhase::UILoad,
        priority: 0,
    };

    fn new(ctx: &impl ModuleCtx) -> Self {
        let window = ctx.get_object::<gtk::ApplicationWindow>("window");
        window.set_application(Some(&ctx.get_application()));

        let manager = adw::StyleManager::default();
        manager.set_color_scheme(adw::ColorScheme::PreferDark);

        let settings = gtk::Settings::default().expect("Could not get default settings");
        settings.set_gtk_theme_name(Some("Sweet-Ambar-Blue"));

        let _holder_realize = signal!(window, connect_realize, move |win| {
            settings.set_gtk_xft_dpi(win.get_dpi().unwrap_or(96) as i32 * 1024);
        });

        Self { _holder_realize }
    }
}
