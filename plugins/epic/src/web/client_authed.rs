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

    pub fn get_account(&self) -> impl FutureResult<GetAccount> {
        self.client
            .get(format_url!(
                Account,
                "public/account/{}",
                self.auth_data.account_id
            ))
            .decorate(self)
            .bearer_auth(self.auth_data.access_token.as_str())
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
}

impl ClientTrait for ClientAuthed {
    fn decorate_request(&self, req: RequestBuilder) -> RequestBuilder {
        req.bearer_auth(self.auth_data.access_token.as_str())
    }
}

impl Drop for ClientAuthed {
    fn drop(&mut self) {
        // kill token
        todo!()
    }
}
