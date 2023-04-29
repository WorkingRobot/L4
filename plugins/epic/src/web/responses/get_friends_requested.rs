#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFriendsRequested {
    // List of all requested friends returned (for incoming and outgoing)
    #[serde(flatten)]
    pub requests: Vec<super::RealFriend>,
}
