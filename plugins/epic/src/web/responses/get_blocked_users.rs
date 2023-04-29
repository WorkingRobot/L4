#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetBlockedUsers {
    // List of all blocked users returned
    #[serde(flatten)]
    users: Vec<super::FriendUser>,
}
