use std::time::SystemTime;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthId {
    // Account id of the auth
    pub id: String,

    // Type of auth
    #[serde(rename = "type")]
    pub auth_type: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExternalAuth {
    // Id of the epic account this auth is attatched to (redundant info since you need it to get this data anyway)
    pub account_id: String,

    // Type of connection (github, google, steam, twitch, etc)
    #[serde(rename = "type")]
    pub auth_type: String,

    // Username/id of the user on the external platform (optional)
    pub external_auth_id: Option<String>,

    // Type of the auth id (optional)
    pub external_auth_id_type: Option<String>,

    // Secondary username/id of the external platform (optional)
    pub external_auth_secondary_id: Option<String>,

    // Display name of the user on the external platform
    pub external_display_name: Option<String>,

    // Avatar (id) of the user on the external platform (optional)
    pub avatar: Option<String>,

    // List of all auth ids this connection is attached to
    pub auth_ids: Vec<AuthId>,

    // When this connection was added to the epic account (only given if this is your account)
    pub date_added: Option<SystemTime>,

    // When this connection was used to login to the epic account (this doesn't look accurate) (optional)
    pub last_login: Option<SystemTime>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountExternalAuths {
    // A list of all external connections the account has
    #[serde(flatten)]
    pub external_auths: Vec<ExternalAuth>,
}
