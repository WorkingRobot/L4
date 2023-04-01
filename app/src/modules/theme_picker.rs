use super::{Module, ModuleCtx};
use gtk::gdk;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ThemePicker {
    provider: gtk::CssProvider,
}

impl ThemePicker {
    fn set_theme(&self, theme_idx: u32) {
        return;
        let theme_name = match theme_idx {
            0 => "Sweet",
            1 => "Sweet-Dark",
            2 => "Sweet-Mars",
            3 => "Sweet-Ambar",
            4 => "Sweet-Ambar-Blue",
            _ => todo!(),
        };

        self.provider.load_from_resource(
            format!("/org/gtk/libgtk/theme/{theme_name}/gtk-dark.css").as_str(),
        );
    }
}

impl Module for ThemePicker {
    fn new(ctx: &impl ModuleCtx) -> Rc<RefCell<Self>> {
        gtk::Settings::default()
            .unwrap()
            .set_gtk_theme_name(Some("Sweet-Dark"));

        let this = Self {
            provider: gtk::CssProvider::new(),
        };

        let display = gdk::Display::default().expect("Could not get a display");
        this.provider.connect_parsing_error(|_, section, error| {
            panic!("Could not parse css data ({} at {})", error, section);
        });

        #[allow(deprecated)]
        // add_provider_for_display isn't actually deprecated, but the rest of StyleContext is
        gtk::StyleContext::add_provider_for_display(
            &display,
            &this.provider,
            gtk::STYLE_PROVIDER_PRIORITY_THEME,
        );

        this.set_theme(0);

        let this = Rc::new(RefCell::new(this));

        let dropdown = ctx.get_object::<gtk::DropDown>("theme_dropdown");
        let this_clone = this.clone();
        dropdown.connect_selected_notify(move |drop| {
            if drop.selected() != gtk::INVALID_LIST_POSITION {
                this_clone.borrow().set_theme(drop.selected());
            }
        });

        this
    }
}
