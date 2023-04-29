use std::{collections::HashMap, path::Path};

use rand::seq::SliceRandom;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryParameter {
    name: String,
    value: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    uri: String,
    query_params: Vec<QueryParameter>,
    headers: Vec<QueryParameter>,
}

impl Manifest {
    pub fn cloud_dir(&self) -> &str {
        Path::new(&self.uri)
            .parent()
            .map(|p| p.to_str())
            .flatten()
            .unwrap()
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    // Name of the app
    app_name: String,

    // Label of the app
    label_name: String,

    // Version of the app
    build_version: String,

    // SHA1 of manifest file
    hash: String,

    // Any metadata for the element (optional)
    // Here are some possible keys:
    // androidSigningFingerprintSHA1
    // androidPackageVersionCode
    // androidSigningFingerprintSHA256
    // androidPackageName
    // status
    metadata: Option<HashMap<String, String>>,

    // Manifest URLs
    manifests: Vec<Manifest>,
}

impl Element {
    pub fn pick_manifest(&self) -> Option<&Manifest> {
        self.manifests.choose(&mut rand::thread_rng())
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetDownloadInfo {
    // A list of all assets available to the user
    elements: Vec<Element>,
}

impl GetDownloadInfo {
    pub fn get_element(&self, app_name: &str) -> Option<&Element> {
        self.elements.iter().find(|e| e.app_name == app_name)
    }
}
