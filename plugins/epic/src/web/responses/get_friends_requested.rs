#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetFriendsRequested {
    // List of all requested friends returned (for incoming and outgoing)
    #[serde(flatten)]
    requests: Vec<super::RealFriend>,
}
