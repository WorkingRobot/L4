#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod widgets;

use deps::utils::UsesDpi;
use gtk::prelude::*;
use gtk::{gdk, gio, glib, IconTheme};
use widgets::AppWindow;

static APP_ID: &str = "me.workingrobot.l4";

#[tokio::main]
async fn main() -> glib::ExitCode {
    if true {
        #[cfg(debug_assertions)]
        // Cairo is enabled for faster launch times
        std::env::set_var("GSK_RENDERER", "cairo");
    }

    gio::resources_register_include!("Sweet.gresource").expect("Failed to register theme");
    gio::resources_register_include!("Sweet-Ambar.gresource").expect("Failed to register theme");
    gio::resources_register_include!("Sweet-Ambar-Blue.gresource")
        .expect("Failed to register theme");
    gio::resources_register_include!("Sweet-Dark.gresource").expect("Failed to register theme");
    gio::resources_register_include!("Sweet-Mars.gresource").expect("Failed to register theme");
    gio::resources_register_include!("FontAwesome.gresource").expect("Failed to register theme");
    gio::resources_register_include!("L4.gresource").expect("Failed to register app resources");

    gtk::init().expect("Failed to initialize GTK");
    adw::init().expect("Failed to initialize LibAdwaita");

    glib::set_application_name("L4");

    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        let display = gdk::Display::default().expect("Could not get a display");
        let icon_theme = IconTheme::for_display(&display);
        icon_theme.add_resource_path("/me/workingrobot/l4");
        icon_theme.add_resource_path("/com/fontawesome/icons");

        let window = AppWindow::new(app.upcast_ref::<gtk::Application>());

        let manager = adw::StyleManager::default();
        manager.set_color_scheme(adw::ColorScheme::PreferDark);

        let settings = gtk::Settings::default().expect("Could not get default settings");
        settings.set_gtk_theme_name(Some("Sweet-Ambar-Blue"));

        window.connect_realize(move |win| {
            settings.set_gtk_xft_dpi(win.get_dpi().unwrap_or(96) as i32 * 1024);
        });

        window.minimize();
        window.present();
    });

    app.run()
}
