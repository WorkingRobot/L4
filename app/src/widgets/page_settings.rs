use super::{composite_widget, models};
use gtk::{
    gio::{ListModel, ListStore},
    glib,
    prelude::{Cast, StaticType},
    subclass::prelude::*,
    CompositeTemplate, SingleSelection, TemplateChild, TreeListModel,
};
use once_cell::unsync::OnceCell;

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
    pub(self) selection: TemplateChild<SingleSelection>,

    pub(self) plugin_model: OnceCell<ListStore>,
}

impl ObjectImpl for PageSettingsInner {}

impl WidgetImpl for PageSettingsInner {}

impl BoxImpl for PageSettingsInner {}

impl PageSettings {
    pub fn init_model(&self, plugin_model: ListStore) {
        let imp = self.imp();

        let settings_model = ListStore::new(models::Setting::static_type());
        settings_model.append(&models::Setting::new(
            "General".to_owned(),
            "fa-gear-solid-symbolic".to_owned(),
        ));
        settings_model.append(&models::Setting::new(
            "Plugins".to_owned(),
            "fa-puzzle-piece-solid-symbolic".to_owned(),
        ));

        imp.plugin_model.set(plugin_model).unwrap();
        let model = TreeListModel::new(
            settings_model.upcast::<ListModel>(),
            false,
            true,
            glib::clone!(@weak self as s => @default-panic, move |o| {
                s.on_create_model(o)
            }),
        );
        imp.selection.set_model(Some(&model));
    }

    fn on_create_model(&self, obj: &glib::Object) -> Option<ListModel> {
        let setting: &models::Setting = obj.downcast_ref::<_>()?;
        if setting.name() == "Plugins" {
            self.imp()
                .plugin_model
                .get()
                .map(|m| m.clone().upcast::<ListModel>())
        } else {
            None
        }
    }
}
