#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetExchangeCode {
    expires_in_seconds: i32,
    code: String,
    creating_client_id: String,
}
