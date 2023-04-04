use super::{Module, ModuleCtx};
use crate::utils::{signal, SignalHolder, UsesDpi};
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Init {
    _holder_realize: SignalHolder,
}

impl Module for Init {
    fn new(ctx: &impl ModuleCtx) -> Rc<RefCell<Self>> {
        let window = ctx.get_object::<gtk::ApplicationWindow>("window");
        window.set_application(Some(&ctx.get_application()));

        let manager = adw::StyleManager::default();
        manager.set_color_scheme(adw::ColorScheme::PreferDark);

        let settings = gtk::Settings::default().expect("Could not get default settings");
        settings.set_gtk_theme_name(Some("Sweet-Ambar-Blue"));

        let _holder_realize = signal!(window, connect_realize, move |win| {
            settings.set_gtk_xft_dpi(win.get_dpi().unwrap_or(96) as i32 * 1024);
        });

        Rc::new(RefCell::new(Self { _holder_realize }))
    }
}
