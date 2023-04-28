#[derive(Copy, Clone)]
pub enum Credentials {
    LauncherAppClient2,
    EpicGamesClientService,
}

impl Credentials {
    pub const fn id(self) -> &'static str {
        match self {
            Credentials::LauncherAppClient2 => "34a02cf8f4414e29b15921876da36f9a",
            Credentials::EpicGamesClientService => "7a40f8cdafd346219a4a0a15522b8ed7",
        }
    }

    pub const fn secret(self) -> &'static str {
        match self {
            Credentials::LauncherAppClient2 => "daafbccc737745039dffe53d94fc76cf",
            Credentials::EpicGamesClientService => "a94578c3-3a79-4441-ad22-a4ef6c9380a1",
        }
    }
}

pub trait BasicAuthable: Sized {
    fn credentials(self, credentials: Credentials) -> Self;
}

impl BasicAuthable for reqwest::RequestBuilder {
    fn credentials(self, credentials: Credentials) -> Self {
        self.basic_auth(credentials.id(), Some(credentials.secret()))
    }
}
