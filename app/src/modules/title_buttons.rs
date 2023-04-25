use super::module::*;
use deps::utils::{signal, SignalHolder};
use gtk::glib;
use gtk::prelude::*;
use gtk::traits::ButtonExt;

pub struct TitleButtons {
    _holder_settings: SignalHolder,
    _holder_about: SignalHolder,
    _holder_debug: SignalHolder,
    _holder_back: SignalHolder,
    _holder_stack_switch: SignalHolder,
}

impl Module for TitleButtons {
    const META: Metadata = Metadata {
        phase: LoadPhase::UILoad,
        priority: 1,
    };

    fn new(ctx: &impl ModuleCtx) -> Self {
        let stack = ctx.get_object::<gtk::Stack>("main-stack");

        let button = ctx.get_object::<gtk::Button>("button-back");
        let _holder_back = signal!(
            button,
            connect_clicked,
            glib::clone!(@weak stack => move |_| {
                stack.set_visible_child_name("games");
            })
        );
        let _holder_stack_switch = signal!(
            stack,
            connect_visible_child_name_notify,
            glib::clone!(@weak button => move |stack| {
                button.set_visible(stack.visible_child_name().unwrap_or_default().as_str() != "games");
            })
        );

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

        Self {
            _holder_settings,
            _holder_about,
            _holder_debug,
            _holder_back,
            _holder_stack_switch,
        }
    }
}
