#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetExternalSourceSettings {
    // Whether to not ask the user to link their account (only seen false)
    pub do_not_show_linking_proposal: bool,
}
