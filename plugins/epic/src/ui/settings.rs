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
    pub account_list: TemplateChild<StringList>,
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
        let launcher = gtk::UriLauncher::new("https://www.epicgames.com/id/embedded/login?client_id=3f69e56c7649492c8cc29f1af08a8a12&response_type=code&display=embedded&prompt=login");
        launcher.launch(None::<&gtk::Window>, None::<&gtk::gio::Cancellable>, |_| {});
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
