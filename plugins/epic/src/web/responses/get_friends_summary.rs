use std::{collections::HashMap, time::SystemTime};

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FriendConnection {
    // Given by all but nintendo
    pub name: Option<String>,

    // Given by steam
    pub id: Option<String>,

    // Given by steam
    pub avatar: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FriendUser {
    // Account id of the friend
    pub account_id: String,

    // Display name of the user (if not headless)
    pub display_name: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedFriend {
    #[serde(flatten)]
    pub base: FriendUser,

    // Any external connections the friend has
    pub connections: HashMap<String, FriendConnection>,

    // Given sometimes
    #[serde(default, rename = "mutual")]
    pub mutual_friend_count: i32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestedFriend {
    #[serde(flatten)]
    pub base: SuggestedFriend,
    // favorite is always false
}

// Returned in the explicit (non-summary) endpoints
#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RealFriend {
    #[serde(flatten)]
    pub base: RequestedFriend,
    // empty groups array
    pub alias: String,
    pub note: String,
    pub created: SystemTime,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FriendLimits {
    pub incoming: bool,
    pub outgoing: bool,
    pub accepted: bool,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFriendsSummary {
    // List of all friends returned
    pub friends: Vec<RealFriend>,

    pub incoming: Vec<RequestedFriend>,
    pub outgoing: Vec<RequestedFriend>,
    pub suggested: Vec<SuggestedFriend>,
    pub blocklist: Vec<FriendUser>,

    // settings aren't used anywhere, it seems like "acceptInvites" is always "public"
    pub limits_reached: FriendLimits,
}
