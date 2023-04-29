#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockedUsers {
    // List of all blocked users returned
    #[serde(flatten)]
    pub users: Vec<super::FriendUser>,
}
