#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetFriends {
    // List of all friends returned
    #[serde(flatten)]
    friends: Vec<super::RealFriend>,
}
