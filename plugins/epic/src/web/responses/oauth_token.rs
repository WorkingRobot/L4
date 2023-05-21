use chrono::{DateTime, Utc};
use serde::*;
use serde_enum_str::Deserialize_enum_str as DeserializeEnum;

#[derive(Deserialize, Debug)]
pub struct OAuthToken {
    pub access_token: String,
    pub expires_in: i32,
    pub expires_at: DateTime<Utc>,
    pub token_type: TokenType,
    pub client_id: String,
    pub internal_client: bool,
    pub client_service: String,
    pub product_id: Option<String>,
    pub application_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OAuthTokenUser {
    pub access_token: String,
    pub expires_in: i32,
    pub expires_at: DateTime<Utc>,
    pub token_type: TokenType,
    pub refresh_token: String,
    pub refresh_expires: i32,
    pub refresh_expires_at: DateTime<Utc>,
    pub account_id: String,
    pub client_id: String,
    pub internal_client: bool,
    pub client_service: String,
    pub scope: Vec<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub app: String,
    pub in_app_id: String,
    pub device_id: String,
    pub product_id: Option<String>,
    pub application_id: Option<String>,
}

#[derive(DeserializeEnum, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Bearer,
    #[serde(other)]
    Other(String),
}
