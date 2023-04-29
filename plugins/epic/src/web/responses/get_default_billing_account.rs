#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetDefaultBillingAccount {
    // Display name of the billing account (e.g "PayPal - a***z@example.com")
    billing_account_name: String,

    // Country of the billing account
    country: String,

    // Not too sure what this is, I've only seen "BILLING"
    country_source: String,

    // Currency used for the billing acocunt (e.g. "USD")
    currency: String,
}
