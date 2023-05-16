#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod widgets;

use gtk::prelude::*;
use gtk::{gio::ApplicationFlags, glib};
use widgets::App;

static APP_ID: &str = "me.workingrobot.l4";

#[tokio::main]
async fn main() -> glib::ExitCode {
    #[cfg(debug_assertions)]
    // Cairo is enabled for faster launch times
    std::env::set_var("GSK_RENDERER", "cairo");

    glib::set_application_name("L4");

    let app = App::new(APP_ID, ApplicationFlags::HANDLES_COMMAND_LINE);
    app.run()
}
