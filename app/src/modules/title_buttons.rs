use super::{Module, ModuleCtx};
use gtk::glib;
use gtk::traits::ButtonExt;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TitleButtons {}

impl Module for TitleButtons {
    fn new(ctx: &impl ModuleCtx) -> Rc<RefCell<Self>> {
        let this = Self {};

        let stack = ctx.get_object::<gtk::Stack>("main-stack");

        let button = ctx.get_object::<gtk::Button>("button-settings");
        button.connect_clicked(glib::clone!(@weak stack => @default-panic, move |_| {
            stack.set_visible_child_name("settings");
        }));

        let button = ctx.get_object::<gtk::Button>("button-about");
        button.connect_clicked(glib::clone!(@weak stack => @default-panic, move |_| {
            stack.set_visible_child_name("about");
        }));

        let button = ctx.get_object::<gtk::Button>("button-inspector");
        button.connect_clicked(|_| gtk::Window::set_interactive_debugging(true));

        Rc::new(RefCell::new(this))
    }
}
