#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[allow(dead_code)]
mod archive;
mod modules;
mod plugins;
mod utils;
mod widgets;

use archive::*;
use gtk::prelude::*;
use gtk::{gio, glib};
use widgets::Application;

static APP_ID: &str = "me.workingrobot.l4";

fn main() -> glib::ExitCode {
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
    glib::set_program_name(Some("L4"));

    let mut archive = ArchiveMut::new("yo.ar").unwrap();
    let mut stream = archive.stream_mut(4).unwrap();
    let mut iter = stream.iter_bytes_mut(63..4030).unwrap();

    while let Some(_slice) = iter.next() {}

    Application::from_application_id(APP_ID).run()
}
