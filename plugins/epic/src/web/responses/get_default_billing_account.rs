#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetDefaultBillingAccount {
    // Display name of the billing account (e.g "PayPal - a***z@example.com")
    pub billing_account_name: String,

    // Country of the billing account
    pub country: String,

    // Not too sure what this is, I've only seen "BILLING"
    pub country_source: String,

    // Currency used for the billing acocunt (e.g. "USD")
    pub currency: String,
}
