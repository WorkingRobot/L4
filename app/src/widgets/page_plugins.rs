use super::composite_widget;
use super::models;
use deps::plugins::PluginRegistry;
use gtk::{gio::ListStore, glib, subclass::prelude::*, CompositeTemplate, TemplateChild};
use std::cell::RefCell;

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
    store: TemplateChild<ListStore>,
    registry: RefCell<PluginRegistry>,
}

#[gtk::template_callbacks]
impl PagePluginsInner {
    pub fn init(&self) {
        self.load_plugins();

        for plugin in self.registry.borrow().iter_plugins() {
            self.store.append(&models::Plugin::new(plugin));
        }
    }

    fn load_plugins(&self) {
        let mut registry_mut = self.registry.borrow_mut();
        std::fs::read_dir(std::env::current_exe().unwrap().parent().unwrap())
            .unwrap()
            .flatten()
            .filter(|e| e.metadata().map_or(false, |m| m.is_file()))
            .filter(|e| {
                e.file_name()
                    .to_str()
                    .map_or(false, |f| f.starts_with("plugins_") && f.ends_with(".dll"))
            })
            .for_each(|e| unsafe { registry_mut.load(e.path()) }.unwrap())
    }
}

impl WidgetImpl for PagePluginsInner {}

impl BoxImpl for PagePluginsInner {}
