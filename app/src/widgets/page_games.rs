use super::composite_widget;
use gtk::{glib, subclass::prelude::*, CompositeTemplate};

composite_widget!(PageGames => "L4PageGames",
    @inner PageGamesInner,
    @parent gtk::Box,
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable
);

#[derive(CompositeTemplate, Default)]
#[template(resource = "/me/workingrobot/l4/templates/page_games.ui")]
pub struct PageGamesInner {}

#[gtk::template_callbacks]
impl PageGamesInner {
    fn init(&self) {}
}

impl WidgetImpl for PageGamesInner {}

impl BoxImpl for PageGamesInner {}
