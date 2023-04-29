use std::{collections::HashMap, time::SystemTime};

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub template_id: String,
    pub attributes: serde_json::Value,
    pub quantity: i32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub attributes: serde_json::Value,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    #[serde(rename = "_id")]
    pub id: String,
    pub created: SystemTime,
    pub updated: SystemTime,
    pub rvn: i32,
    pub wipe_number: i32,
    pub account_id: String,
    pub profile_id: String,
    pub version: String,
    pub items: HashMap<String, Item>,
    pub stats: Stats,
    pub command_revision: i32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileChange {
    pub change_type: String,
    pub profile: Profile,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryProfile {
    pub profile_revision: i32,
    pub profile_id: String,
    pub profile_changes_base_revision: i32,
    pub profile_changes: Vec<ProfileChange>,
    pub profile_command_revision: i32,
    pub server_time: SystemTime,

    // A multiUpdate array can also exist here which is basically QueryProfile but without ServerTime or ResponseVersion (seen in campaign)
    pub response_version: i32,
}
