use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CatalogKeyImage {
    // Type of key image
    #[serde(rename = "type")]
    pub image_type: String,

    // URL of key image
    pub url: String,

    // MD5 of the image at the url
    pub md5: String,

    // Width of the image
    pub width: i32,

    // Height of the image
    pub height: i32,

    // Size of the image (file size)
    pub size: i32,

    // Date of when the image was uploaded
    pub uploaded_date: DateTime<Utc>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CatalogCategory {
    // Path of the category? This is the only field so idfk
    pub path: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CatalogCustomAttribute {
    // Type of the value (I've only seen STRING, so I think value can stay as just a string)
    #[serde(rename = "type")]
    pub attr_type: String,

    // Value of the attribute
    pub value: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CatalogReleaseInfo {
    // Id of the release
    pub id: String,

    // App id of the release
    pub app_id: String,

    // compatibleApps is a list here, but it's empty and optional (I saw see it for fortnite)

    // Platforms the release is available on
    pub platform: Vec<String>,

    // Date when the release was added
    pub date_added: Option<DateTime<Utc>>,

    // These 2 fields were seen in poodle/twinmotion, but both were empty
    pub release_note: Option<String>,

    pub version_title: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CatalogDLCItem {
    // Id of the DLC
    pub id: String,

    // Namespace of the DLC
    pub namespace: String,

    // Whether the DLC can't appear in search
    pub unsearchable: bool,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CatalogItem {
    // Id of the catalog item (redundant in both the url request and the key in the map)
    pub id: String,

    // Title of the item
    pub title: String,

    // Description of the item
    pub description: String,

    // Description, but lengthy
    pub long_description: Option<String>,

    // Technical requirements of the app/item
    pub technical_details: Option<String>,

    // Key images of the item
    pub key_images: Vec<CatalogKeyImage>,

    // Categories of the item
    pub categories: Vec<CatalogCategory>,

    // Namespace of the catalog item (redundant since it's in the url request)
    pub namespace: String,

    // Status of the catalog (I've seen "ACTIVE", not sure about others)
    pub status: String,

    // Date created
    pub creation_date: DateTime<Utc>,

    // Last time the item was modified
    pub last_modified_date: DateTime<Utc>,

    // Custom attributes, e.g. can run offline, presence id, cloud save folders, UAC, etc.
    pub custom_attributes: Option<HashMap<String, CatalogCustomAttribute>>,

    // Entitlement name of the item (same as id usually)
    pub entitlement_name: String,

    // Entitlement type of the item (e.g. "EXECUTABLE")
    pub entitlement_type: String,

    // Item type? not sure what this is, I've seen "DURABLE"
    pub item_type: String,

    // Release info of the item
    pub release_info: Vec<CatalogReleaseInfo>,

    // Developer of the item
    pub developer: String,

    // Id of the developer
    pub developer_id: String,

    // EULA ids for the item
    pub eula_ids: Vec<String>,

    // Whether it's end of support for the item
    pub end_of_support: bool,

    // DLCs for the item
    pub dlc_item_list: Option<Vec<CatalogDLCItem>>,

    // Self refundable
    pub self_refundable: Option<bool>,

    // ageGatings dictionary/map/object here, but it's empty (only in fortnite)

    // Saw this in twinmotion/poodle, it's set to false. i wonder what qualifies as "secure"
    pub requires_secure_account: Option<bool>,

    // Unsearchable (like twinmotion education edition)
    pub unsearchable: bool,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCatalogItems {
    // A list of all queried items
    #[serde(flatten)]
    pub items: HashMap<String, CatalogItem>,
}
