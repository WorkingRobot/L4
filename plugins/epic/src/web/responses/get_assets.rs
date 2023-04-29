#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetMetadata {
    // Not too sure what this is, looks like a guid like the other values
    pub installation_pool_id: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    // Name of the app
    pub app_name: String,

    // Label of the app
    pub label_name: String,

    // Version of the app
    pub build_version: String,

    // Catalog id of the app
    pub catalog_item_id: String,

    // Namespace of the app
    pub namespace: String,

    // Any metadata for the app (optional)
    pub metadata: Option<AssetMetadata>,

    // Asset id?
    pub asset_id: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAssets {
    // A list of all assets available to the user
    #[serde(flatten)]
    pub assets: Vec<Asset>,
}
