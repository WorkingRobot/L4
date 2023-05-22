use crate::config::SavedUserCreds;
use adw::{subclass::prelude::*, AboutWindow};
use deps::utils::{composite_widget, Dispatcher};
use gtk::{glib, traits::GtkWindowExt, CompositeTemplate, TemplateChild};
use once_cell::unsync::OnceCell;
use plugins_core::prelude::{
    adw::Avatar,
    gtk::{
        gdk::Texture,
        gdk_pixbuf::Pixbuf,
        gio::{Cancellable, MemoryInputStream},
        glib::Bytes,
    },
    *,
};
use reqwest::Client;

composite_widget!(Account => "EpicAccount",
    @inner AccountInner!,
    @parent adw::ActionRow,
    @extends adw::ActionRow, adw::PreferencesRow, gtk::ListBoxRow, gtk::Widget,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget
);

#[derive(CompositeTemplate, Default)]
#[template(resource = "/me/workingrobot/l4/epic/templates/account.ui")]
pub struct AccountInner {
    #[template_child]
    pub avatar: TemplateChild<Avatar>,

    pub account: OnceCell<SavedUserCreds>,
}

#[gtk::template_callbacks]
impl AccountInner {
    fn set_account(&self, account: SavedUserCreds) {
        self.obj().set_title(&account.display_name);
        self.obj().set_subtitle(&account.account_id);

        let mut avatar_dispatcher = Dispatcher::<Bytes>::new();

        let avatar = &*self.avatar;
        avatar_dispatcher.connect(
            glib::clone!(@weak avatar => @default-return false, move |bytes| {
                Pixbuf::from_stream_async(
                    &MemoryInputStream::from_bytes(&bytes),
                    None::<&Cancellable>,
                    move |result| {
                        if let Ok(pixbuf) = result {
                            let texture = Texture::for_pixbuf(&pixbuf);
                            avatar.set_custom_image(Some(&texture));
                        }
                        // On failure, ignore
                    },
                );
                true
            }),
        );

        tokio::spawn(async move {
            if let Some(avatar) = account.avatar_id.strip_prefix("ATHENACHARACTER:") {
                let data = Client::new()
                    .get(format!(
                        "https://fortnite-api.com/images/cosmetics/br/{}/smallicon.png",
                        avatar
                    ))
                    .send()
                    .await
                    .unwrap()
                    .bytes()
                    .await
                    .unwrap();
                let bytes = Bytes::from_owned(data);
                avatar_dispatcher.emit(bytes);
            }
        });
    }

    #[template_callback]
    fn on_remove_account(&self) {
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

impl ObjectImpl for AccountInner {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for AccountInner {}

impl ListBoxRowImpl for AccountInner {}

impl PreferencesRowImpl for AccountInner {}

impl ActionRowImpl for AccountInner {}

impl Account {
    pub fn new(account: SavedUserCreds) -> Self {
        let this: Account = glib::Object::builder().build();
        this.imp().set_account(account);
        this
    }
}
