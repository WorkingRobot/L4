use std::time::SystemTime;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DeviceInfo {
    // Manufacturer? e.g: Google
    #[serde(rename = "type")]
    device_type: Option<String>,

    // Phone model e.g: Pixel 2
    model: Option<String>,

    // OS version e.g: 10
    os: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DeviceEventData {
    // City, Country (probably gets this from a URL similar to https://www.epicgames.com/id/api/location)
    location: String,

    // Ipv4 address, haven't seen/tried this with ipv6
    ip_address: String,

    // Time of event occurrence
    date_time: SystemTime,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DeviceAuth {
    // Unique id of device, created at time of auth creation
    device_id: String,

    // Account id of device holder
    account_id: String,

    // Secret is only provided when creating the auth
    secret: Option<String>,

    // If a user agent was given during the creation, it'll be here
    user_agent: Option<String>,

    // Provided with X-Epic-Device-Info header on creation, e.g. {"type":"Google","model":"Pixel 2","os":"10"}
    // One of the fields must be not empty/null in order to set it
    // If a field is null or does not exist, it isn't set, but if it's an empty string (as long as another field is set), it is
    device_info: Option<DeviceInfo>,

    // When the auth was created
    created: DeviceEventData,

    // When the auth was last used
    last_access: Option<DeviceEventData>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetDeviceAuths {
    #[serde(flatten)]
    auths: Vec<DeviceAuth>,
}
