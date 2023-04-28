use super::{
    credentials::BasicAuthable, format_url, responses::*, Credentials, FutureResult, Sendable,
};
use reqwest::Client as RClient;

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> reqwest::Result<Self> {
        Ok(Self {
            client: RClient::builder().build()?,
        })
    }

    pub fn get_page_info(&self, language: &str) -> impl FutureResult<GetPageInfo> {
        self.client
            .get(format_url!(FortniteContent, "pages/fortnite-game"))
            .query(&[("lang", language)])
            .send_into::<GetPageInfo>()
    }

    pub fn get_blog_posts(
        &self,
        locale: &str,
        posts_per_page: i32,
        offset: i32,
    ) -> impl FutureResult<GetBlogPosts> {
        self.client
            .get(format_url!(BaseFortnite, "blog/getPosts"))
            .query(&[
                ("category", ""),
                ("postsPerPage", posts_per_page.to_string().as_str()),
                ("offset", offset.to_string().as_str()),
                ("locale", locale),
                ("rootPageSlug", "blog"),
            ])
            .send_into::<GetBlogPosts>()
    }

    pub fn get_statuspage_summary(&self) -> impl FutureResult<GetStatuspageSummary> {
        self.client
            .get(format_url!(Statuspage, "summary.json"))
            .send_into::<GetStatuspageSummary>()
    }

    pub fn oauth_client_credentials(
        &self,
        credentials: Credentials,
    ) -> impl FutureResult<OAuthToken> {
        self.client
            .post(format_url!(Account, "oauth/token"))
            .credentials(credentials)
            .form(&[("grant_type", "client_credentials"), ("token_type", "eg1")])
            .send_into::<OAuthToken>()
    }

    pub fn oauth_authorization_code(
        &self,
        credentials: Credentials,
        auth_code: &str,
    ) -> impl FutureResult<OAuthTokenUser> {
        self.client
            .post(format_url!(Account, "oauth/token"))
            .credentials(credentials)
            .form(&[
                ("grant_type", "authorization_code"),
                ("token_type", "eg1"),
                ("code", auth_code),
            ])
            .send_into::<OAuthTokenUser>()
    }

    pub fn oauth_exchange_code(
        &self,
        credentials: Credentials,
        exchange_code: &str,
    ) -> impl FutureResult<OAuthTokenUser> {
        self.client
            .post(format_url!(Account, "oauth/token"))
            .credentials(credentials)
            .form(&[
                ("grant_type", "exchange_code"),
                ("token_type", "eg1"),
                ("exchange_code", exchange_code),
            ])
            .send_into::<OAuthTokenUser>()
    }

    pub fn oauth_refresh_token(
        &self,
        credentials: Credentials,
        refresh_token: &str,
    ) -> impl FutureResult<OAuthTokenUser> {
        self.client
            .post(format_url!(Account, "oauth/token"))
            .credentials(credentials)
            .form(&[
                ("grant_type", "refresh_token"),
                ("token_type", "eg1"),
                ("refresh_token", refresh_token),
            ])
            .send_into::<OAuthTokenUser>()
    }
}
