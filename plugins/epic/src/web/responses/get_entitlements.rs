use std::time::SystemTime;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Entitlement {
    // Id of the entitlement
    id: String,

    // Name of the entitlement
    entitlement_name: String,

    // Namespace of the entitlement
    namespace: String,

    // Catalog id of the entitlement
    catalog_item_id: String,

    // Account id that this entitlement is given to
    account_id: String,

    // Identity id that this entitlement is given to (seems equal to the account id)
    identity_id: String,

    // Type of entitlement (e.g. "EXECUTABLE" "ENTITLEMENT" "AUDIENCE")
    entitlement_type: String,

    // When the entitlement was granted
    grant_date: SystemTime,

    // When the entitlement started being given out (I think)
    start_date: Option<SystemTime>,

    // Whether the entitlement is cosumable (?)
    consumable: bool,

    // Status of the entitlement (I've only seen "ACTIVE")
    status: String,

    // Whether the entitlement is active
    active: bool,

    // Number of times the entitlement has been used (?)
    use_count: i32,

    // Not sure what this means
    original_use_count: Option<i32>,

    // Not sure what this means (I've seen "EPIC")
    platform_type: Option<String>,

    // When the entitlement was created
    created: SystemTime,

    // Last time the entitlement was updated
    updated: SystemTime,

    // Whether the entitlement was given to a group (?)
    group_entitlement: bool,

    // Country? Unsure what this would do here
    country: Option<String>,

    // Not sure, I've only seen "anonymous"
    operator: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetEntitlements {
    // A list of all entitlements recieved
    #[serde(flatten)]
    entitlements: Vec<Entitlement>,
}
