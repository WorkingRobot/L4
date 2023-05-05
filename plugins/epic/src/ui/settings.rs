use adw::{subclass::prelude::*, AboutWindow, ComboRow, PreferencesPage};
use deps::utils::composite_widget;
use gtk::{
    gio::{resources_enumerate_children, ResourceLookupFlags},
    glib,
    prelude::{ObjectExt, ToValue},
    traits::GtkWindowExt,
    CompositeTemplate, StringList, TemplateChild,
};
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
    #[template_child(id = "plugins-page")]
    plugins_page: TemplateChild<PreferencesPage>,

    #[template_child(id = "theme-list")]
    theme_list: TemplateChild<StringList>,

    #[template_child(id = "theme-combo")]
    theme_combo: TemplateChild<ComboRow>,
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
}

impl ObjectImpl for SettingsInner {
    fn constructed(&self) {
        self.parent_constructed();

        let mut themes =
            resources_enumerate_children("/org/gtk/libgtk/theme", ResourceLookupFlags::NONE)
                .unwrap()
                .iter()
                .filter_map(|t| t.strip_suffix("/").map(str::to_owned))
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

impl WidgetImpl for SettingsInner {}

impl PreferencesGroupImpl for SettingsInner {}

impl Settings {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
