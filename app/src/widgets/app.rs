use super::AppWindow;
use adw::subclass::prelude::*;
use deps::utils::{subclassed_gobject, UsesDpi};
use gtk::{
    gdk,
    gio::{self, prelude::*, ApplicationFlags},
    glib,
    traits::{GtkWindowExt, WidgetExt},
    IconTheme,
};
use std::cell::OnceCell;

subclassed_gobject!(App => "L4App",
    @inner AppInner,
    @parent adw::Application,
    @extends adw::Application, gtk::Application, gio::Application,
    @implements gio::ActionGroup, gio::ActionMap
);

#[derive(Default)]
pub struct AppInner {
    window: OnceCell<AppWindow>,
}

impl AppInner {
    fn register_resources() -> Result<(), glib::Error> {
        gio::resources_register_include!("Sweet.gresource")?;
        gio::resources_register_include!("Sweet-Ambar.gresource")?;
        gio::resources_register_include!("Sweet-Ambar-Blue.gresource")?;
        gio::resources_register_include!("Sweet-Dark.gresource")?;
        gio::resources_register_include!("Sweet-Mars.gresource")?;
        gio::resources_register_include!("FontAwesome.gresource")?;
        gio::resources_register_include!("L4.gresource")?;
        Ok(())
    }

    fn register_icons() -> Option<()> {
        let display = gdk::Display::default()?;
        let icon_theme = IconTheme::for_display(&display);
        icon_theme.add_resource_path("/me/workingrobot/l4");
        icon_theme.add_resource_path("/com/fontawesome/icons");
        Some(())
    }

    fn register_theme(settings: &gtk::Settings) {
        let manager = adw::StyleManager::default();
        manager.set_color_scheme(adw::ColorScheme::PreferDark);

        settings.set_gtk_theme_name(Some("Sweet-Ambar-Blue"));
    }
}

impl ObjectImpl for AppInner {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl ApplicationImpl for AppInner {
    fn startup(&self) {
        Self::register_resources().expect("Failed to register resources");

        self.parent_startup();

        Self::register_icons().expect("Failed to register icons");

        let settings = gtk::Settings::default().expect("Failed to get default settings");
        Self::register_theme(&settings);

        let window = AppWindow::new(self.obj().upcast_ref::<gtk::Application>());
        window.connect_realize(move |window| {
            settings.set_gtk_xft_dpi(window.get_dpi().unwrap_or(96) as i32 * 1024)
        });
        self.window.set(window).unwrap();
    }

    fn activate(&self) {
        self.parent_activate();

        if let Some(window) = self.window.get() {
            window.minimize();
            window.present();
        }
    }

    fn command_line(&self, command_line: &gio::ApplicationCommandLine) -> glib::ExitCode {
        self.parent_command_line(command_line);
        self.activate();

        if command_line.arguments().len() != 1 {
            if let Some(window) = self.window.get() {
                if let Err(error) = window.imp().on_protocol_callback(command_line) {
                    println!("Command line handling error: {error:?}");
                    return 1.into();
                }
            }
        }

        0.into()
    }
}

impl GtkApplicationImpl for AppInner {}

impl AdwApplicationImpl for AppInner {}

impl App {
    pub fn new(application_id: &str, flags: ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }
}
