use super::Account;
use crate::config::{Config, SavedUserCreds};
use adw::{subclass::prelude::*, AboutWindow, ExpanderRow};
use deps::utils::composite_widget;
use gtk::{glib, traits::GtkWindowExt, CompositeTemplate, TemplateChild};
use plugins_core::prelude::*;
use std::{
    cell::OnceCell,
    sync::{Arc, RwLock},
};

composite_widget!(Settings => "EpicSettings",
    @inner SettingsInner!,
    @parent adw::PreferencesGroup,
    @extends adw::PreferencesGroup, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
);

#[derive(CompositeTemplate, Default)]
#[template(resource = "/me/workingrobot/l4/epic/templates/settings.ui")]
pub struct SettingsInner {
    #[template_child(id = "account-row")]
    account_row: TemplateChild<ExpanderRow>,

    config: OnceCell<Arc<RwLock<Config>>>,
}

#[gtk::template_callbacks]
impl SettingsInner {
    pub fn on_added_account(&self, user: SavedUserCreds) {
        self.account_row.add_row(&Account::new(user.clone()));
    }

    pub fn set_config(&self, config: Arc<RwLock<Config>>) {
        self.config.set(config).ok().unwrap();
        let config = self.config.get().unwrap().read().unwrap();

        for user in &config.accounts.0 {
            self.account_row.add_row(&Account::new(user.clone()));
        }
        //if let Some(selected_user) = &config.selected_user {
        // todo
        //}
    }

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

impl ObjectImpl for SettingsInner {}

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
