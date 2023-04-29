#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LightswitchLauncherInfoDTO {
    // App name of service (used in request)
    pub app_name: String,

    // Catalog item id of service
    pub catalog_item_id: String,

    // Namespace of service
    pub namespace: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LightswitchServiceStatus {
    // Given from request (more or less)
    pub service_instance_id: String,

    // Status of service (UP or DOWN)
    pub status: String,

    // Message to give (Not really shown to users, I doubt they'd want to see "Yo Fortnite's up")
    pub message: String,

    // Never seen this not be null
    pub maintenance_uri: String,

    // Never seen a real use for this since I've only seen it used on Fortnite, and their one id here leads to
    // https://raw.githubusercontent.com/EpicData-info/items-tracker/master/database/items/a7f138b2e51945ffbfdacc1af0541053.json
    pub override_catalog_ids: Vec<String>,

    // Only isn't empty when logged in (can show "PLAY" or "DOWNLOAD", haven't seen others)
    pub allowed_actions: Vec<String>,

    // If you're banned
    pub banned: bool,

    // Has stuff for launcher, no idea tbh (not sure if this is optional, but I don't want to risk it)
    pub launcher_info_dto: Option<LightswitchLauncherInfoDTO>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLightswitchStatus {
    // List of all services requested
    #[serde(flatten)]
    pub services: Vec<LightswitchServiceStatus>,
}
