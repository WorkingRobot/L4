macro_rules! add_module {
    ($mod_name: tt) => {
        mod $mod_name;
        pub use $mod_name::*;
    };
}

add_module!(get_account_external_auths);
add_module!(get_account);
add_module!(get_accounts);
add_module!(get_assets);
add_module!(get_blocked_users);
add_module!(get_blog_posts);
add_module!(get_catalog_items);
add_module!(get_csrf_token);
add_module!(get_currencies);
add_module!(get_default_billing_account);
add_module!(get_device_auths);
add_module!(get_download_info);
add_module!(get_entitlements);
add_module!(get_exchange_code);
add_module!(get_external_source_settings);
add_module!(get_friends_requested);
add_module!(get_friends_suggested);
add_module!(get_friends_summary);
add_module!(get_friends);
add_module!(get_launcher_download_info);
add_module!(get_lightswitch_status);
add_module!(get_statuspage_summary);
add_module!(oauth_token);
add_module!(query_profile);
