use chrono::{DateTime, Utc};
use serde::*;
use serde_enum_str::Deserialize_enum_str as DeserializeEnum;

#[derive(Deserialize, Debug)]
pub struct GetStatuspageSummary {
    pub page: Page,
    pub components: Vec<Component>,
    pub status: Status,
}

#[derive(Deserialize, Debug)]
pub struct Page {
    pub id: String,
    pub name: String,
    pub url: String,
    pub timezone: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub status: ComponentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub position: i32,
    pub group: bool,
    pub only_show_if_degraded: bool,
    #[serde(default)]
    pub components: Option<Vec<String>>,
}

#[derive(DeserializeEnum, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ComponentStatus {
    Operational,
    DegradedPerformance,
    PartialOutage,
    MajorOutage,
    #[serde(other)]
    Other(String),
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub indicator: StatusIndicator,
    pub description: String,
}

#[derive(DeserializeEnum, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StatusIndicator {
    None,
    Minor,
    Major,
    Critical,
    #[serde(other)]
    Other(String),
}
