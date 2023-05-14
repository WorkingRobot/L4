use adw::{prelude::*, subclass::prelude::*, AboutWindow, ComboRow, PreferencesPage};
use deps::utils::composite_widget;
use gtk::{
    gio::{resources_enumerate_children, ListStore, ResourceLookupFlags},
    glib,
    traits::GtkWindowExt,
    CompositeTemplate, StringList, TemplateChild,
};
use once_cell::unsync::OnceCell;
use std::cell::RefCell;

use super::models;

composite_widget!(SettingsWindow => "L4SettingsWindow",
    @inner SettingsWindowInner!,
    @parent adw::PreferencesWindow,
    @extends adw::PreferencesWindow, adw::Window, gtk::Window, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager
);

pub struct SettingsWindowData {
    plugin_store: ListStore,
}

impl SettingsWindowData {
    fn new(_outer: &SettingsWindowInner, plugin_store: ListStore) -> Self {
        let this = Self { plugin_store };
        for plugin in &this.plugin_store {
            let plugin = plugin
                .unwrap()
                .downcast_ref::<models::Plugin>()
                .unwrap()
                .imp()
                .plugin()
                .unwrap();
            _outer.plugins_page.add(&plugin.get_settings_widget());
        }
        this
    }
}

#[derive(CompositeTemplate, Default)]
#[template(resource = "/me/workingrobot/l4/templates/settings_window.ui")]
pub struct SettingsWindowInner {
    #[template_child(id = "plugins-page")]
    plugins_page: TemplateChild<PreferencesPage>,

    #[template_child(id = "theme-list")]
    theme_list: TemplateChild<StringList>,

    #[template_child(id = "theme-combo")]
    theme_combo: TemplateChild<ComboRow>,

    data: RefCell<OnceCell<SettingsWindowData>>,
}

#[gtk::template_callbacks]
impl SettingsWindowInner {
    #[template_callback]
    fn on_open_about(&self) {
        AboutWindow::builder()
            .application_name("L4")
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
}

impl ObjectImpl for SettingsWindowInner {
    fn constructed(&self) {
        self.parent_constructed();

        let mut themes =
            resources_enumerate_children("/org/gtk/libgtk/theme", ResourceLookupFlags::NONE)
                .unwrap()
                .iter()
                .filter_map(|t| t.strip_suffix('/').map(str::to_owned))
                .collect::<Vec<_>>();
        themes.sort();
        self.theme_list
            .splice(0, 0, &themes.iter().map(String::as_str).collect::<Vec<_>>());

        gtk::Settings::default()
            .unwrap()
            .bind_property("gtk-theme-name", &self.theme_combo.get(), "selected")
            .transform_to(glib::clone!(@strong themes => move |_, s: &str| {
                themes
                    .iter()
                    .position(|t| s == t)
                    .map(|p| (p as u32).to_value())
            }))
            .transform_from(glib::clone!(@strong themes => move |_, s: u32| Some(themes[s as usize].to_value())))
            .bidirectional()
            .sync_create()
            .build();
    }
}

impl WidgetImpl for SettingsWindowInner {}

impl WindowImpl for SettingsWindowInner {}

impl AdwWindowImpl for SettingsWindowInner {}

impl PreferencesWindowImpl for SettingsWindowInner {}

impl SettingsWindow {
    pub fn new(plugin_model: ListStore) -> Self {
        let window = glib::Object::builder::<Self>().build();
        let imp = window.imp();
        _ = imp
            .data
            .borrow()
            .set(SettingsWindowData::new(imp, plugin_model));
        window
    }
}
