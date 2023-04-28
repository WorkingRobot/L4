mod get_page_info;
pub use get_page_info::GetPageInfo;

mod get_blog_posts;
pub use get_blog_posts::GetBlogPosts;

mod get_statuspage_summary;
pub use get_statuspage_summary::GetStatuspageSummary;

mod oauth_token;
pub use oauth_token::{OAuthToken, OAuthTokenUser};

mod get_account;
pub use get_account::GetAccount;

pub struct GetAccountExternalAuths;
#[derive(serde::Deserialize, Debug)]
pub struct GetAccounts;
#[derive(serde::Deserialize, Debug)]
pub struct Account;
#[derive(serde::Deserialize, Debug)]
pub struct GetDeviceAuths;
#[derive(serde::Deserialize, Debug)]
pub struct DeviceAuth;
#[derive(serde::Deserialize, Debug)]
pub struct GetExchangeCode;
