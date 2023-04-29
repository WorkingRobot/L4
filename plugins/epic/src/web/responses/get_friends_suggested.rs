#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetFriendsSuggested {
    // List of all suggested friends
    #[serde(flatten)]
    suggestions: Vec<super::RealFriend>,
}
