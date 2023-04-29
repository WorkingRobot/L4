use std::time::SystemTime;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAccount {
    // Account id
    pub id: String,

    // Display name
    pub display_name: String,

    // First name
    pub name: String,

    // Last name
    pub last_name: String,

    // Email
    pub email: String,

    // Whether the email has been verfied
    pub email_verified: bool,

    // Failed login attempts before the successful one
    pub failed_login_attempts: i32,

    // Last login time
    pub last_login: SystemTime,

    // Number of times the display name has changed (presumably to detect that the cached display name is different?)
    pub display_name_change_count: i32,

    // Last time the display name changed
    pub last_display_name_change: SystemTime,

    // Whether the user can change the display name
    pub can_update_display_name: bool,

    // Age group, I've only observed "UNKNOWN"
    pub age_group: String,

    // Whether we know the account holder is a minor
    pub minor_verified: bool,

    // Whether we can assume they are a minor, but we can't verify it (I think having a switch fn account enables this)
    pub minor_expected: bool,

    // Minor status, I've only seen "UNKNOWN"
    pub minor_status: String,

    // Not sure what standard this conforms to, I've only had "US"
    pub country: String,

    // Preferred language (I've only had "en")
    pub preferred_language: String,

    // Phone number
    pub phone_number: String,

    // Whether 2FA is enabled
    pub tfa_enabled: bool,

    // This isn't a "real full fledged" account. It's tied directly to a console account or something else
    pub headless: bool,
}
