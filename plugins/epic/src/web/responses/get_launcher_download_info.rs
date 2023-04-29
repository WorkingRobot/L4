use serde_enum_str::Deserialize_enum_str as DeserializeEnum;

#[derive(DeserializeEnum, Debug, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum BuildStatusEnum {
    Deprecated,
    NotDeprecated,
    #[serde(other)]
    Other(String),
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BuildStatus {
    // Not given by CheckLauncherVersion
    app: Option<String>,

    status: BuildStatusEnum,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetLauncherDownloadInfo {
    #[serde(flatten)]
    base: super::GetDownloadInfo,

    // Only provided if ClientVersion is given: A list of build statuses (only 1 item)
    #[serde(rename = "buildStatuses")]
    statuses: Vec<BuildStatus>,
}
