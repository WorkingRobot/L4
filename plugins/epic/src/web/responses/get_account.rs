use std::time::SystemTime;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAccount {
    // Account id
    id: String,

    // Display name
    display_name: String,

    // First name
    name: String,

    // Last name
    last_name: String,

    // Email
    email: String,

    // Whether the email has been verfied
    email_verified: bool,

    // Failed login attempts before the successful one
    failed_login_attempts: i32,

    // Last login time
    last_login: SystemTime,

    // Number of times the display name has changed (presumably to detect that the cached display name is different?)
    display_name_change_count: i32,

    // Last time the display name changed
    last_display_name_change: SystemTime,

    // Whether the user can change the display name
    can_update_display_name: bool,

    // Age group, I've only observed "UNKNOWN"
    age_group: String,

    // Whether we know the account holder is a minor
    minor_verified: bool,

    // Whether we can assume they are a minor, but we can't verify it (I think having a switch fn account enables this)
    minor_expected: bool,

    // Minor status, I've only seen "UNKNOWN"
    minor_status: String,

    // Not sure what standard this conforms to, I've only had "US"
    country: String,

    // Preferred language (I've only had "en")
    preferred_language: String,

    // Phone number
    phone_number: String,

    // Whether 2FA is enabled
    tfa_enabled: bool,

    // This isn't a "real full fledged" account. It's tied directly to a console account or something else
    headless: bool,
}
