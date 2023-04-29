use std::{collections::HashMap, time::SystemTime};

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FriendConnection {
    // Given by all but nintendo
    name: Option<String>,

    // Given by steam
    id: Option<String>,

    // Given by steam
    avatar: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FriendUser {
    // Account id of the friend
    account_id: String,

    // Display name of the user (if not headless)
    display_name: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedFriend {
    #[serde(flatten)]
    base: FriendUser,

    // Any external connections the friend has
    connections: HashMap<String, FriendConnection>,

    // Given sometimes
    #[serde(default, rename = "mutual")]
    mutual_friend_count: i32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestedFriend {
    #[serde(flatten)]
    base: SuggestedFriend,
    // favorite is always false
}

// Returned in the explicit (non-summary) endpoints
#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RealFriend {
    #[serde(flatten)]
    base: RequestedFriend,
    // empty groups array
    alias: String,
    note: String,
    created: SystemTime,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FriendLimits {
    incoming: bool,
    outgoing: bool,
    accepted: bool,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFriendsSummary {
    // List of all friends returned
    friends: Vec<RealFriend>,

    incoming: Vec<RequestedFriend>,
    outgoing: Vec<RequestedFriend>,
    suggested: Vec<SuggestedFriend>,
    blocklist: Vec<FriendUser>,

    // settings aren't used anywhere, it seems like "acceptInvites" is always "public"
    limits_reached: FriendLimits,
}
