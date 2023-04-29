use std::{collections::HashMap, time::SystemTime};

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Item {
    template_id: String,
    attributes: serde_json::Value,
    quantity: i32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Stats {
    attributes: serde_json::Value,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Profile {
    #[serde(rename = "_id")]
    id: String,
    created: SystemTime,
    updated: SystemTime,
    rvn: i32,
    wipe_number: i32,
    account_id: String,
    profile_id: String,
    version: String,
    items: HashMap<String, Item>,
    stats: Stats,
    command_revision: i32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ProfileChange {
    change_type: String,
    profile: Profile,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct QueryProfile {
    profile_revision: i32,
    profile_id: String,
    profile_changes_base_revision: i32,
    profile_changes: Vec<ProfileChange>,
    profile_command_revision: i32,
    server_time: SystemTime,

    // A multiUpdate array can also exist here which is basically QueryProfile but without ServerTime or ResponseVersion (seen in campaign)
    response_version: i32,
}
