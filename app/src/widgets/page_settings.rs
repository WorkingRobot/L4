use super::composite_widget;
use gtk::{glib, subclass::prelude::*, CompositeTemplate, SingleSelection, TemplateChild};

composite_widget!(PageSettings => "L4PageSettings",
    @inner PageSettingsInner,
    @parent gtk::Box,
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable
);

#[derive(CompositeTemplate, Default)]
#[template(resource = "/me/workingrobot/l4/templates/page_settings.ui")]
pub struct PageSettingsInner {
    #[template_child]
    selection: TemplateChild<SingleSelection>,
}

#[gtk::template_callbacks]
impl PageSettingsInner {
    fn init(&self) {}
}

impl WidgetImpl for PageSettingsInner {}

impl BoxImpl for PageSettingsInner {}
