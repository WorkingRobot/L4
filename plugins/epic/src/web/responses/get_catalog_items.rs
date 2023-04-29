use std::{collections::HashMap, time::SystemTime};

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CatalogKeyImage {
    // Type of key image
    #[serde(rename = "type")]
    image_type: String,

    // URL of key image
    url: String,

    // MD5 of the image at the url
    md5: String,

    // Width of the image
    width: i32,

    // Height of the image
    height: i32,

    // Size of the image (file size)
    size: i32,

    // Date of when the image was uploaded
    uploaded_date: SystemTime,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CatalogCategory {
    // Path of the category? This is the only field so idfk
    path: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CatalogCustomAttribute {
    // Type of the value (I've only seen STRING, so I think value can stay as just a string)
    #[serde(rename = "type")]
    attr_type: String,

    // Value of the attribute
    value: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CatalogReleaseInfo {
    // Id of the release
    id: String,

    // App id of the release
    app_id: String,

    // compatibleApps is a list here, but it's empty and optional (I saw see it for fortnite)

    // Platforms the release is available on
    platform: Vec<String>,

    // Date when the release was added
    date_added: Option<SystemTime>,

    // These 2 fields were seen in poodle/twinmotion, but both were empty
    release_note: Option<String>,

    version_title: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CatalogDLCItem {
    // Id of the DLC
    id: String,

    // Namespace of the DLC
    namespace: String,

    // Whether the DLC can't appear in search
    unsearchable: bool,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CatalogItem {
    // Id of the catalog item (redundant in both the url request and the key in the map)
    id: String,

    // Title of the item
    title: String,

    // Description of the item
    description: String,

    // Description, but lengthy
    long_description: Option<String>,

    // Technical requirements of the app/item
    technical_details: Option<String>,

    // Key images of the item
    key_images: Vec<CatalogKeyImage>,

    // Categories of the item
    categories: Vec<CatalogCategory>,

    // Namespace of the catalog item (redundant since it's in the url request)
    namespace: String,

    // Status of the catalog (I've seen "ACTIVE", not sure about others)
    status: String,

    // Date created
    creation_date: SystemTime,

    // Last time the item was modified
    last_modified_date: SystemTime,

    // Custom attributes, e.g. can run offline, presence id, cloud save folders, UAC, etc.
    custom_attributes: Option<HashMap<String, CatalogCustomAttribute>>,

    // Entitlement name of the item (same as id usually)
    entitlement_name: String,

    // Entitlement type of the item (e.g. "EXECUTABLE")
    entitlement_type: String,

    // Item type? not sure what this is, I've seen "DURABLE"
    item_type: String,

    // Release info of the item
    release_info: Vec<CatalogReleaseInfo>,

    // Developer of the item
    developer: String,

    // Id of the developer
    developer_id: String,

    // EULA ids for the item
    eula_ids: Vec<String>,

    // Whether it's end of support for the item
    end_of_support: bool,

    // DLCs for the item
    dlc_item_list: Option<Vec<CatalogDLCItem>>,

    // Self refundable
    self_refundable: Option<bool>,

    // ageGatings dictionary/map/object here, but it's empty (only in fortnite)

    // Saw this in twinmotion/poodle, it's set to false. i wonder what qualifies as "secure"
    requires_secure_account: Option<bool>,

    // Unsearchable (like twinmotion education edition)
    unsearchable: bool,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetCatalogItems {
    // A list of all queried items
    #[serde(flatten)]
    items: HashMap<String, CatalogItem>,
}
