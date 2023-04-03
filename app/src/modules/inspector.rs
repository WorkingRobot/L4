use super::{Module, ModuleCtx};
use gtk::traits::ButtonExt;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Inspector {}

impl Module for Inspector {
    fn new(ctx: &impl ModuleCtx) -> Rc<RefCell<Self>> {
        let this = Self {};

        let button = ctx.get_object::<gtk::Button>("button-inspector");
        button.connect_clicked(|_| gtk::Window::set_interactive_debugging(true));

        Rc::new(RefCell::new(this))
    }
}
