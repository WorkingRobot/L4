use deps::utils::Encrypted;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavedUserCreds {
    pub account_id: String,
    pub display_name: String,
    pub avatar_id: String,
    pub device_id: String,
    pub secret: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    pub accounts: Encrypted<Vec<SavedUserCreds>>,
    pub selected_account_id: Option<String>,
}
