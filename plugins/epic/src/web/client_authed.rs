use chrono::Utc;
use reqwest::RequestBuilder;

use super::{
    format_url, responses::*, ClientTrait, Credentials, Decoratable, FutureResult, Sendable,
};

pub struct ClientAuthed {
    client: reqwest::Client,
    auth_data: OAuthTokenUser,
    credentials: Credentials,
}

impl ClientAuthed {
    pub fn new(auth_data: OAuthTokenUser, credentials: Credentials) -> reqwest::Result<Self> {
        Ok(Self {
            client: reqwest::Client::builder().build()?,
            auth_data,
            credentials,
        })
    }

    // Account service

    fn kill_token(&self) -> impl FutureResult<()> {
        self.client
            .delete(format_url!(
                Account,
                "oauth/sessions/kill/{}",
                self.auth_data.access_token
            ))
            .decorate(self)
            .send_into_empty()
    }

    pub fn get_account(&self) -> impl FutureResult<GetAccount> {
        self.client
            .get(format_url!(
                Account,
                "public/account/{}",
                self.auth_data.account_id
            ))
            .decorate(self)
            .send_into::<GetAccount>()
    }

    pub fn get_account_external_auths(&self) -> impl FutureResult<GetAccountExternalAuths> {
        self.client
            .get(format_url!(
                Account,
                "public/account/{}/externalAuths",
                self.auth_data.account_id
            ))
            .decorate(self)
            .send_into::<GetAccountExternalAuths>()
    }

    // NOTE: You can only request a maximum of 100 accounts at a time!
    // Fortnite uses 50 at a time, so it's recommended to do the same
    pub fn get_accounts(&self, account_ids: &[&str]) -> impl FutureResult<GetAccounts> {
        self.client
            .get(format_url!(Account, "public/account"))
            .decorate(self)
            .query(
                &account_ids
                    .iter()
                    .map(|id| ("accountId", id))
                    .collect::<Vec<_>>(),
            )
            .send_into::<GetAccounts>()
    }

    pub fn get_account_by_id(&self, account_id: &str) -> impl FutureResult<Account> {
        self.client
            .get(format_url!(Account, "public/account/{}", account_id))
            .decorate(self)
            .send_into::<Account>()
    }

    pub fn get_account_by_display_name(&self, display_name: &str) -> impl FutureResult<Account> {
        self.client
            .get(format_url!(
                Account,
                "public/account/displayName/{}",
                display_name
            ))
            .decorate(self)
            .send_into::<Account>()
    }

    // Doesn't work anymore allegedly?
    pub fn get_account_by_email(&self, email: &str) -> impl FutureResult<Account> {
        self.client
            .get(format_url!(Account, "public/account/email/{}", email))
            .decorate(self)
            .send_into::<Account>()
    }

    pub fn get_device_auths(&self) -> impl FutureResult<GetDeviceAuths> {
        self.client
            .get(format_url!(
                Account,
                "public/account/{}/deviceAuth",
                self.auth_data.account_id
            ))
            .decorate(self)
            .send_into::<GetDeviceAuths>()
    }

    // You can optionally add a X-Epic-Device-Info header with JSON {"type": "Google","model":"Pixel 3","os":"10"}
    pub fn create_device_auth(&self) -> impl FutureResult<DeviceAuth> {
        self.client
            .post(format_url!(
                Account,
                "public/account/{}/deviceAuth",
                self.auth_data.account_id
            ))
            .decorate(self)
            .header("Content-Type", "application/json")
            .body("{}")
            .send_into::<DeviceAuth>()
    }

    pub fn get_exchange_code(&self) -> impl FutureResult<GetExchangeCode> {
        self.client
            .get(format_url!(Account, "oauth/exchange"))
            .decorate(self)
            .send_into::<GetExchangeCode>()
    }

    // Launcher service

    pub fn get_default_billing_account(&self) -> impl FutureResult<GetDefaultBillingAccount> {
        self.client
            .get(format_url!(
                Account,
                "public/account/{}/billingaccounts/default",
                self.auth_data.account_id
            ))
            .decorate(self)
            .send_into::<GetDefaultBillingAccount>()
    }

    // URL I'm unsure of: GET https://launcher-public-service-prod06.ol.epicgames.com/launcher/api/public/accounts/{AccountId}/wallet?currencyCode=USD
    // It just returns 204, not sure what it does exactly

    // Default label is Production (if omitted from the request), but the launcher always calls with Live
    // All platforms are technically valid, but they'll return an empty list
    // Some that I know for sure that work are "Windows", "Mac", "IOS"
    pub fn get_assets(&self, platform: &str, label: &str) -> impl FutureResult<GetAssets> {
        self.client
            .get(format_url!(Launcher, "public/assets/{platform}"))
            .decorate(self)
            .query(&[("label", label)])
            .send_into::<GetAssets>()
    }

    // Version is in format like "11.0.1-14907503+++Portal+Release-Live-Windows"
    pub fn check_launcher_version(&self, current_version: &str) -> impl FutureResult<BuildStatus> {
        self.client
            .get(format_url!(
                Launcher,
                "public/assets/info/launcher/{current_version}"
            ))
            .decorate(self)
            .send_into::<BuildStatus>()
    }

    // Label used to be Live-DurrBurger up until 10.18.11, now it's Live-EternalKnight
    pub fn get_launcher_download_info(
        &self,
        platform: &str,
        label: &str,
        client_version: Option<&str>,
        machine_id: Option<&str>,
    ) -> impl FutureResult<GetLauncherDownloadInfo> {
        self.client
            .get(format_url!(
                Launcher,
                "public/assets/v2/platform/{platform}/launcher"
            ))
            .decorate(self)
            .header("Content-Type", "application/json")
            .query(&[
                ("label", label),
                ("clientVersion", client_version.unwrap_or("")),
                ("machineId", machine_id.unwrap_or("")),
            ])
            .send_into::<GetLauncherDownloadInfo>()
    }

    pub fn get_download_info(
        &self,
        platform: &str,
        label: &str,
        catalog_item_id: &str,
        app_name: &str,
    ) -> impl FutureResult<GetDownloadInfo> {
        self.client
            .post(format_url!(
                Launcher,
                "public/assets/v2/platform/{platform}/catalogItem/{catalog_item_id}/app/{app_name}/label/{label}"
            ))
            .decorate(self)
            .header("Content-Type", "application/json")
            .body("{}")
            .send_into::<GetDownloadInfo>()
    }

    // Catalog service

    // You can actually get by without auth (e.g. open in browser) if you set start to 0 and count to 100
    // By ommitting start/count, it only returns 10 elements
    pub fn get_currencies(&self, start: i32, count: i32) -> impl FutureResult<GetCurrencies> {
        self.client
            .get(format_url!(Catalog, "shared/currencies"))
            .decorate(self)
            .query(&[("start", start), ("count", count)])
            .send_into::<GetCurrencies>()
    }

    // I have never seen those 2 booleans actually add something to the response
    pub fn get_catalog_items(
        &self,
        namespace: &str,
        items: &[&str],
        country: &str,
        locale: &str,
        include_dlc_details: bool,
        include_main_game_details: bool,
    ) -> impl FutureResult<GetCatalogItems> {
        self.client
            .get(format_url!(
                Catalog,
                "shared/namespace/{namespace}/bulk/items"
            ))
            .decorate(self)
            .query(
                &[
                    (
                        "includeDLCDetails",
                        include_dlc_details.to_string().as_str(),
                    ),
                    (
                        "includeMainGameDetails",
                        include_main_game_details.to_string().as_str(),
                    ),
                    ("country", country),
                    ("locale", locale),
                ]
                .into_iter()
                .chain(items.iter().map(|id| ("id", *id)))
                .collect::<Vec<_>>(),
            )
            .send_into::<GetCatalogItems>()
    }

    // Entitlement service

    // Max count = 5000
    pub fn get_entitlements(&self, start: i32, count: i32) -> impl FutureResult<GetEntitlements> {
        self.client
            .get(format_url!(
                Entitlement,
                "account/{}/entitlements",
                self.auth_data.account_id
            ))
            .decorate(self)
            .query(&[("start", start), ("count", count)])
            .send_into::<GetEntitlements>()
    }

    // Friends service

    pub fn get_friends_summary(&self) -> impl FutureResult<GetFriendsSummary> {
        self.client
            .get(format_url!(
                Friends,
                "v1/{}/summary",
                self.auth_data.account_id
            ))
            .decorate(self)
            .query(&[("displayNames", true)])
            .send_into::<GetFriendsSummary>()
    }

    pub fn get_friends(&self) -> impl FutureResult<GetFriends> {
        self.client
            .get(format_url!(
                Friends,
                "v1/{}/friends",
                self.auth_data.account_id
            ))
            .decorate(self)
            .query(&[("displayNames", true)])
            .send_into::<GetFriends>()
    }

    pub fn get_friends_inbound_requests(&self) -> impl FutureResult<GetFriendsRequested> {
        self.client
            .get(format_url!(
                Friends,
                "v1/{}/incoming",
                self.auth_data.account_id
            ))
            .decorate(self)
            .query(&[("displayNames", true)])
            .send_into::<GetFriendsRequested>()
    }

    pub fn get_friends_outbound_requests(&self) -> impl FutureResult<GetFriendsRequested> {
        self.client
            .get(format_url!(
                Friends,
                "v1/{}/outgoing",
                self.auth_data.account_id
            ))
            .decorate(self)
            .query(&[("displayNames", true)])
            .send_into::<GetFriendsRequested>()
    }

    pub fn get_friends_suggested(&self) -> impl FutureResult<GetFriendsSuggested> {
        self.client
            .get(format_url!(
                Friends,
                "v1/{}/suggested",
                self.auth_data.account_id
            ))
            .decorate(self)
            .query(&[("displayNames", true)])
            .send_into::<GetFriendsSuggested>()
    }

    pub fn get_blocked_users(&self) -> impl FutureResult<GetBlockedUsers> {
        self.client
            .get(format_url!(
                Friends,
                "v1/{}/blocklist",
                self.auth_data.account_id
            ))
            .decorate(self)
            .query(&[("displayNames", true)])
            .send_into::<GetBlockedUsers>()
    }

    pub fn get_friend(&self, account_id: &str) -> impl FutureResult<RealFriend> {
        self.client
            .get(format_url!(
                Friends,
                "v1/{}/friends/{account_id}",
                self.auth_data.account_id
            ))
            .decorate(self)
            .query(&[("displayNames", true)])
            .send_into::<RealFriend>()
    }

    pub async fn add_friend(&self, account_id: &str) -> impl FutureResult<()> {
        self.client
            .post(format_url!(
                Friends,
                "v1/{}/friends/{account_id}",
                self.auth_data.account_id
            ))
            .decorate(self)
            .send_into_empty()
    }

    pub fn remove_friend(&self, account_id: &str) -> impl FutureResult<()> {
        self.client
            .delete(format_url!(
                Friends,
                "v1/{}/friends/{account_id}",
                self.auth_data.account_id
            ))
            .decorate(self)
            .send_into_empty()
    }

    pub fn set_friend_alias(&self, account_id: &str, nickname: &str) -> impl FutureResult<()> {
        self.client
            .put(format_url!(
                Friends,
                "v1/{}/friends/{account_id}/alias",
                self.auth_data.account_id
            ))
            .decorate(self)
            .header("Content-Type", "text/plain")
            .body(nickname.to_owned())
            .send_into_empty()
    }

    pub fn clear_friend_alias(&self, account_id: &str) -> impl FutureResult<()> {
        self.client
            .delete(format_url!(
                Friends,
                "v1/{}/friends/{account_id}/alias",
                self.auth_data.account_id
            ))
            .decorate(self)
            .send_into_empty()
    }

    pub fn set_friend_note(&self, account_id: &str, note: &str) -> impl FutureResult<()> {
        self.client
            .put(format_url!(
                Friends,
                "v1/{}/friends/{account_id}/note",
                self.auth_data.account_id
            ))
            .decorate(self)
            .header("Content-Type", "text/plain")
            .body(note.to_owned())
            .send_into_empty()
    }

    pub fn clear_friend_note(&self, account_id: &str) -> impl FutureResult<()> {
        self.client
            .delete(format_url!(
                Friends,
                "v1/{}/friends/{account_id}/note",
                self.auth_data.account_id
            ))
            .decorate(self)
            .send_into_empty()
    }

    pub fn block_user(&self, account_id: &str) -> impl FutureResult<()> {
        self.client
            .post(format_url!(
                Friends,
                "v1/{}/blocklist/{account_id}",
                self.auth_data.account_id
            ))
            .decorate(self)
            .send_into_empty()
    }

    pub fn unblock_user(&self, account_id: &str) -> impl FutureResult<()> {
        self.client
            .delete(format_url!(
                Friends,
                "v1/{}/blocklist/{account_id}",
                self.auth_data.account_id
            ))
            .decorate(self)
            .send_into_empty()
    }

    // Lightswitch

    pub fn get_lightswitch_status(
        &self,
        app_name: &str,
    ) -> impl FutureResult<LightswitchServiceStatus> {
        self.client
            .get(format_url!(Lightswitch, "service/{app_name}/status"))
            .decorate(self)
            .send_into::<LightswitchServiceStatus>()
    }

    pub fn get_lightswitch_status_multiple(
        &self,
        app_names: &[&str],
    ) -> impl FutureResult<GetLightswitchStatus> {
        self.client
            .get(format_url!(Lightswitch, "service/bulk/status"))
            .decorate(self)
            .query(
                &app_names
                    .iter()
                    .map(|n| ("serviceId", n))
                    .collect::<Vec<_>>(),
            )
            .send_into::<GetLightswitchStatus>()
    }

    // MCP Requests

    // ProfileIds:
    // athena = br
    // collections = s14 fish collections
    // common_core = purchases and banners(?)
    // creative = islands and stuff
    // common_public = no idea, hasn't been updated in 2 years
    // metadata = stw storm shield stuff
    // campaign = stw
    // collection_book_schematics0 = stw collection book
    // collection_book_people0 = stw collection book
    // theater0 = stw world inventory
    // outpost0 = stw storm shield storage
    // Use revision = -1 to get the latest
    pub fn query_profile(&self, _profile_id: &str, _revision: i32) {
        unimplemented!()
    }

    // Avatar Service

    // GET https://avatar-service-prod.identity.live.on.epicgames.com/v1/avatar/fortnite/ids?accountIds=accountId1,accoundId2,accountId3,...

    // Profile Service

    // PUT https://global-profile-service.game-social.epicgames.com/profiles
    // JSON: {"namespace":"Fortnite","accountIds":["accoundId1","accountId2"]}

    // Web API

    pub fn authorize_eosclient(
        &self,
        client_id: &str,
        scopes: &[&str],
        csrf_token: &GetCsrfToken,
    ) -> impl FutureResult<()> {
        self.client
            .post(format_url!(BaseEpicGames, "client/{}/authorize", client_id))
            .decorate(self)
            .header("X-XSRF-Token", csrf_token.xsrf_token.as_str())
            .json(&[("scopes", scopes)])
            .header(
                "Cookie",
                [
                    ("EPIC_BEARER_TOKEN", self.auth_data.access_token.as_str()),
                    ("EPIC_SESSION_AP", csrf_token.epic_session_ap.as_str()),
                ]
                .iter()
                .map(|(name, value)| format!("{}={}", name, value))
                .collect::<Vec<_>>()
                .join("; "),
            )
            .send_into_empty()
    }

    // Launcher GraphQL API

    pub fn update_presence_status(
        &self,
        namespace: &str,
        connection_id: &str,
        status: &str,
    ) -> impl FutureResult<()> {
        self.client
            .get(format_url!(LauncherGql, ""))
            .decorate(self)
            .header("User-Agent", "EpicGamesLauncher")
            .json(&[("operationName", serde_json::Value::from("updateStatus")), ("variables", serde_json::Value::from(serde_json::Map::from_iter([("namespace".to_string(), serde_json::Value::from(namespace)), ("connectionId".to_string(), serde_json::Value::from(connection_id)), ("status".to_string(), serde_json::Value::from(status))]))), ("query", serde_json::Value::from("mutation updateStatus($namespace: String!, $connectionId: String!, $status: String!) {\n  PresenceV2 {\n    updateStatus(namespace: $namespace, connectionId: $connectionId, status: $status) {\n      success\n      __typename\n    }\n    __typename\n  }\n}\n"))])
            .send_into_empty()
    }
}

impl ClientTrait for ClientAuthed {
    fn decorate_request(&self, req: RequestBuilder) -> RequestBuilder {
        req.bearer_auth(self.auth_data.access_token.as_str())
    }
}

impl Drop for ClientAuthed {
    fn drop(&mut self) {
        if self.auth_data.expires_at > Utc::now() {
            tokio::task::spawn(self.kill_token());
        }
    }
}
