use super::responses::*;
use reqwest::{Client as RClient, RequestBuilder};
use serde::de::DeserializeOwned;
use std::future::Future;

pub struct Error {}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self {}
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait FutureResult<T> = Future<Output = Result<T>>;

#[derive(Copy, Clone)]
pub enum Credentials {
    LauncherAppClient2,
    EpicGamesClientService,
}

impl Credentials {
    const fn id(self) -> &'static str {
        match self {
            Credentials::LauncherAppClient2 => "34a02cf8f4414e29b15921876da36f9a",
            Credentials::EpicGamesClientService => "7a40f8cdafd346219a4a0a15522b8ed7",
        }
    }

    const fn secret(self) -> &'static str {
        match self {
            Credentials::LauncherAppClient2 => "daafbccc737745039dffe53d94fc76cf",
            Credentials::EpicGamesClientService => "a94578c3-3a79-4441-ad22-a4ef6c9380a1",
        }
    }
}

trait BasicAuthable: Sized {
    fn basic_auth_credentials(self, credentials: Credentials) -> Self;
}

impl BasicAuthable for RequestBuilder {
    fn basic_auth_credentials(self, credentials: Credentials) -> Self {
        self.basic_auth(credentials.id(), Some(credentials.secret()))
    }
}

pub struct Client {
    r_client: RClient,
}

#[derive(Copy, Clone)]
enum Hostname {
    BaseFortnite,
    BaseEpicGames,
    Account,
    Launcher,
    Catalog,
    Entitlement,
    Friends,
    Lightswitch,
    XMPP,
    EOSStomp,
    LauncherStomp,
    EOS,
    FortniteContent,
    LauncherGql,
    Statuspage,
    UnrealEngineCdn1,
    UnrealEngineCdn2,
}

impl Hostname {
    const fn as_url(self) -> &'static str {
        match self {
            Hostname::BaseEpicGames => "https://www.epicgames.com/id/api/",
            Hostname::BaseFortnite => "https://www.epicgames.com/fortnite/api/",
            Hostname::Account => {
                "https://account-public-service-prod.ol.epicgames.com/account/api/"
            }
            Hostname::Launcher => {
                "https://launcher-public-service-prod.ol.epicgames.com/launcher/api/"
            }
            Hostname::Catalog => {
                "https://catalog-public-service-prod.ol.epicgames.com/catalog/api/"
            }
            Hostname::Entitlement => {
                "https://entitlement-public-service-prod.ol.epicgames.com/entitlement/api/"
            }
            Hostname::Friends => {
                "https://friends-public-service-prod.ol.epicgames.com/friends/api/"
            }
            Hostname::Lightswitch => {
                "https://lightswitch-public-service-prod.ol.epicgames.com/lightswitch/api/"
            }
            Hostname::XMPP => "wss://xmpp-service-prod.ol.epicgames.com//",
            Hostname::EOSStomp => "wss://connect.epicgames.dev/",
            Hostname::LauncherStomp => "wss://connect.ol.epicgames.com/",
            Hostname::EOS => "https://api.epicgames.dev/epic/",
            Hostname::FortniteContent => {
                "https://fortnitecontent-website-prod07.ol.epicgames.com/content/api/"
            }
            Hostname::LauncherGql => "https://store-launcher.epicgames.com/graphql",
            Hostname::Statuspage => "https://status.epicgames.com/api/v2/",
            Hostname::UnrealEngineCdn1 => "https://cdn1.unrealengine.com/",
            Hostname::UnrealEngineCdn2 => "https://cdn2.unrealengine.com/",
        }
    }
}

fn format_url(host: Hostname, endpoint: &'static str) -> String {
    format!("{}{}", host.as_url(), endpoint)
}

async fn send_request<T: DeserializeOwned>(req: RequestBuilder) -> Result<T> {
    Ok(req.send().await?.json::<T>().await?)
}

impl Client {
    pub fn new() -> reqwest::Result<Self> {
        Ok(Self {
            r_client: RClient::builder().build()?,
        })
    }

    pub fn get_page_info(&self, language: &str) -> impl FutureResult<GetPageInfo> {
        send_request::<GetPageInfo>(
            self.r_client
                .get(format_url(Hostname::FortniteContent, "pages/fortnite-game"))
                .query(&[("lang", language)]),
        )
    }

    pub fn get_blog_posts(&self, locale: &str, posts_per_page: i32, offset: i32) -> Result<()> {
        Ok(())
    }

    pub fn get_statuspage_summary(&self) -> impl FutureResult<GetStatuspageSummary> {
        send_request::<GetStatuspageSummary>(
            self.r_client
                .get(format_url(Hostname::Statuspage, "summary.json")),
        )
    }

    pub fn oauth_client_credentials(
        &self,
        credentials: Credentials,
    ) -> impl FutureResult<OAuthToken> {
        send_request::<OAuthToken>(
            self.r_client
                .post(format_url(Hostname::Account, "oauth/token"))
                .basic_auth_credentials(credentials)
                .form(&[("grant_type", "client_credentials"), ("token_type", "eg1")]),
        )
    }

    pub fn oauth_authorization_code(
        &self,
        credentials: Credentials,
        auth_code: &str,
    ) -> impl FutureResult<OAuthTokenUser> {
        send_request::<OAuthTokenUser>(
            self.r_client
                .post(format_url(Hostname::Account, "oauth/token"))
                .basic_auth_credentials(credentials)
                .form(&[
                    ("grant_type", "authorization_code"),
                    ("token_type", "eg1"),
                    ("code", auth_code),
                ]),
        )
    }

    pub fn oauth_exchange_code(
        &self,
        credentials: Credentials,
        exchange_code: &str,
    ) -> impl FutureResult<OAuthTokenUser> {
        send_request::<OAuthTokenUser>(
            self.r_client
                .post(format_url(Hostname::Account, "oauth/token"))
                .basic_auth_credentials(credentials)
                .form(&[
                    ("grant_type", "exchange_code"),
                    ("token_type", "eg1"),
                    ("exchange_code", exchange_code),
                ]),
        )
    }

    pub fn oauth_refresh_token(
        &self,
        credentials: Credentials,
        refresh_token: &str,
    ) -> impl FutureResult<OAuthTokenUser> {
        send_request::<OAuthTokenUser>(
            self.r_client
                .post(format_url(Hostname::Account, "oauth/token"))
                .basic_auth_credentials(credentials)
                .form(&[
                    ("grant_type", "refresh_token"),
                    ("token_type", "eg1"),
                    ("refresh_token", refresh_token),
                ]),
        )
    }
}
