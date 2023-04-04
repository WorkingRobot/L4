use super::{Module, ModuleCtx};
use crate::utils::{signal, SignalHolder};
use gtk::glib;
use gtk::prelude::*;
use gtk::traits::ButtonExt;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TitleButtons {
    _holder_settings: SignalHolder,
    _holder_about: SignalHolder,
    _holder_debug: SignalHolder,
}

impl Module for TitleButtons {
    fn new(ctx: &impl ModuleCtx) -> Rc<RefCell<Self>> {
        let stack = ctx.get_object::<gtk::Stack>("main-stack");

        let button = ctx.get_object::<gtk::Button>("button-settings");
        let _holder_settings = signal!(
            button,
            connect_clicked,
            glib::clone!(@weak stack => move |_| {
                stack.set_visible_child_name("settings");
            })
        );

        let button = ctx.get_object::<gtk::Button>("button-about");
        let _holder_about = signal!(
            button,
            connect_clicked,
            glib::clone!(@weak stack => move |_| {
                stack.set_visible_child_name("about");
            })
        );

        let button = ctx.get_object::<gtk::Button>("button-inspector");
        let _holder_debug = signal!(button, connect_clicked, |_| {
            gtk::Window::set_interactive_debugging(true)
        });

        Rc::new(RefCell::new(Self {
            _holder_settings,
            _holder_about,
            _holder_debug,
        }))
    }
}
