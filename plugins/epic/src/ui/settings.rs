use adw::{subclass::prelude::*, AboutWindow};
use deps::utils::composite_widget;
use gtk::{glib, traits::GtkWindowExt, CompositeTemplate, StringList, TemplateChild};
use plugins_core::prelude::*;

composite_widget!(Settings => "EpicSettings",
    @inner SettingsInner!,
    @parent adw::PreferencesGroup,
    @extends adw::PreferencesGroup, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
);

#[derive(CompositeTemplate, Default)]
#[template(resource = "/me/workingrobot/l4/epic/templates/settings.ui")]
pub struct SettingsInner {
    #[template_child(id = "account-list")]
    account_list: TemplateChild<StringList>,
}

#[gtk::template_callbacks]
impl SettingsInner {
    #[template_callback]
    fn on_open_about(&self) {
        AboutWindow::builder()
            .application_name("Epic Games")
            .application_icon("icon")
            .modal(true)
            .version("0.1.0")
            .copyright("copyrighted something")
            .website("https://four.gl")
            .issue_url("https://github.com/WorkingRobot/L4/issues")
            .license_type(gtk::License::Custom)
            .developer_name("Asriel Camora")
            .build()
            .present();
    }

    #[template_callback]
    fn on_add_account(&self) {
        self.account_list.append("Asriel_Dev2");
    }
}

impl ObjectImpl for SettingsInner {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for SettingsInner {}

impl PreferencesGroupImpl for SettingsInner {}

impl Settings {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
