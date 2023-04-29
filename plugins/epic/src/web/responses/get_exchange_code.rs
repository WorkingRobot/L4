#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetExchangeCode {
    pub expires_in_seconds: i32,
    pub code: String,
    pub creating_client_id: String,
}
