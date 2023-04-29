#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFriends {
    // List of all friends returned
    #[serde(flatten)]
    pub friends: Vec<super::RealFriend>,
}
