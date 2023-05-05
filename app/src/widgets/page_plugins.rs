use super::models;
use deps::utils::composite_widget;
use gtk::{
    gio::ListStore, glib, subclass::prelude::*, CompositeTemplate, SingleSelection, TemplateChild,
};

composite_widget!(PagePlugins => "L4PagePlugins",
    @inner PagePluginsInner,
    @parent gtk::Box,
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable,
    @uses models::Plugin
);

#[derive(CompositeTemplate, Default)]
#[template(resource = "/me/workingrobot/l4/templates/page_plugins.ui")]
pub struct PagePluginsInner {
    #[template_child]
    pub(self) selection: TemplateChild<SingleSelection>,
}

impl ObjectImpl for PagePluginsInner {}

impl WidgetImpl for PagePluginsInner {}

impl BoxImpl for PagePluginsInner {}

impl PagePlugins {
    pub fn set_model(&self, model: &ListStore) {
        self.imp().selection.set_model(Some(model));
    }
}
