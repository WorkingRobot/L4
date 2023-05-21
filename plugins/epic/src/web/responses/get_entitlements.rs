use chrono::{DateTime, Utc};

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entitlement {
    // Id of the entitlement
    pub id: String,

    // Name of the entitlement
    pub entitlement_name: String,

    // Namespace of the entitlement
    pub namespace: String,

    // Catalog id of the entitlement
    pub catalog_item_id: String,

    // Account id that this entitlement is given to
    pub account_id: String,

    // Identity id that this entitlement is given to (seems equal to the account id)
    pub identity_id: String,

    // Type of entitlement (e.g. "EXECUTABLE" "ENTITLEMENT" "AUDIENCE")
    pub entitlement_type: String,

    // When the entitlement was granted
    pub grant_date: DateTime<Utc>,

    // When the entitlement started being given out (I think)
    pub start_date: Option<DateTime<Utc>>,

    // Whether the entitlement is cosumable (?)
    pub consumable: bool,

    // Status of the entitlement (I've only seen "ACTIVE")
    pub status: String,

    // Whether the entitlement is active
    pub active: bool,

    // Number of times the entitlement has been used (?)
    pub use_count: i32,

    // Not sure what this means
    pub original_use_count: Option<i32>,

    // Not sure what this means (I've seen "EPIC")
    pub platform_type: Option<String>,

    // When the entitlement was created
    pub created: DateTime<Utc>,

    // Last time the entitlement was updated
    pub updated: DateTime<Utc>,

    // Whether the entitlement was given to a group (?)
    pub group_entitlement: bool,

    // Country? Unsure what this would do here
    pub country: Option<String>,

    // Not sure, I've only seen "anonymous"
    pub operator: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetEntitlements {
    // A list of all entitlements recieved
    #[serde(flatten)]
    pub entitlements: Vec<Entitlement>,
}
