#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFriendsSuggested {
    // List of all suggested friends
    #[serde(flatten)]
    pub suggestions: Vec<super::RealFriend>,
}
