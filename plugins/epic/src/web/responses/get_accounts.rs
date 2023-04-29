use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    // Account id
    pub id: String,

    // Display name or username
    pub display_name: Option<String>,

    // Key is the auth type, value is the auth data
    pub external_auths: HashMap<String, super::ExternalAuth>,
    // links is an empty dictionary, only some users have it, presumably for social medias?
}

impl Account {
    pub fn display_name(&self) -> &str {
        self.display_name
            .as_ref()
            .or_else(|| {
                self.external_auths
                    .iter()
                    .find_map(|a| a.1.external_display_name.as_ref())
            })
            .unwrap_or(&self.id)
            .as_str()
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAccounts {
    // A list of all accounts requested, note that some can be missing if an id does not exist, or if you are unauthorized to view them
    #[serde(flatten)]
    pub accounts: Vec<Account>,
}
