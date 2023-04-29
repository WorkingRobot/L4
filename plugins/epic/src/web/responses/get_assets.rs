#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AssetMetadata {
    // Not too sure what this is, looks like a guid like the other values
    installation_pool_id: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Asset {
    // Name of the app
    app_name: String,

    // Label of the app
    label_name: String,

    // Version of the app
    build_version: String,

    // Catalog id of the app
    catalog_item_id: String,

    // Namespace of the app
    namespace: String,

    // Any metadata for the app (optional)
    metadata: Option<AssetMetadata>,

    // Asset id?
    asset_id: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetAssets {
    // A list of all assets available to the user
    #[serde(flatten)]
    assets: Vec<Asset>,
}
