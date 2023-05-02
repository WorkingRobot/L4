use super::composite_widget;
use gtk::{gio::ListStore, glib, subclass::prelude::*, CompositeTemplate, SingleSelection};

composite_widget!(PageGames => "L4PageGames",
    @inner PageGamesInner,
    @parent gtk::Box,
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable
);

#[derive(CompositeTemplate, Default)]
#[template(resource = "/me/workingrobot/l4/templates/page_games.ui")]
pub struct PageGamesInner {
    #[template_child]
    pub(self) selection: TemplateChild<SingleSelection>,
}

impl ObjectImpl for PageGamesInner {}

impl WidgetImpl for PageGamesInner {}

impl BoxImpl for PageGamesInner {}

impl PageGames {
    pub fn set_model(&self, model: &ListStore) {
        self.imp().selection.set_model(Some(model));
    }
}
