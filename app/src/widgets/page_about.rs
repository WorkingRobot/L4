use super::composite_widget;
use gtk::{glib, subclass::prelude::*, CompositeTemplate};

composite_widget!(PageAbout => "L4PageAbout",
    @inner PageAboutInner,
    @parent gtk::Box,
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable
);

#[derive(CompositeTemplate, Default)]
#[template(resource = "/me/workingrobot/l4/templates/page_about.ui")]
pub struct PageAboutInner {}

#[gtk::template_callbacks]
impl PageAboutInner {
    fn init(&self) {}
}

impl WidgetImpl for PageAboutInner {}

impl BoxImpl for PageAboutInner {}
