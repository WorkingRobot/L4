use serde_enum_str::Deserialize_enum_str as DeserializeEnum;

#[derive(DeserializeEnum, Debug, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BuildStatusType {
    Deprecated,
    NotDeprecated,
    #[serde(other)]
    Other(String),
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BuildStatus {
    // Not given by CheckLauncherVersion
    pub app: Option<String>,

    pub status: BuildStatusType,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLauncherDownloadInfo {
    #[serde(flatten)]
    pub base: super::GetDownloadInfo,

    // Only provided if ClientVersion is given: A list of build statuses (only 1 item)
    #[serde(rename = "buildStatuses")]
    pub statuses: Vec<BuildStatus>,
}
