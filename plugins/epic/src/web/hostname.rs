#[derive(Copy, Clone)]
pub enum Hostname {
    BaseFortnite,
    BaseEpicGames,
    Account,
    Launcher,
    Catalog,
    Entitlement,
    Friends,
    Lightswitch,
    Xmpp,
    EosStomp,
    LauncherStomp,
    Eos,
    FortniteContent,
    LauncherGql,
    Statuspage,
    UnrealEngineCdn1,
    UnrealEngineCdn2,
}

impl Hostname {
    pub const fn into_url(self) -> &'static str {
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
            Hostname::Xmpp => "wss://xmpp-service-prod.ol.epicgames.com//",
            Hostname::EosStomp => "wss://connect.epicgames.dev/",
            Hostname::LauncherStomp => "wss://connect.ol.epicgames.com/",
            Hostname::Eos => "https://api.epicgames.dev/epic/",
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

#[macro_export]
macro_rules! format_url {
    ($hostname:tt, $($arg:tt)*) => {{
        &[$crate::web::hostname::Hostname::$hostname.into_url(), format!($($arg)*).as_str()].concat()
    }};
}

pub use format_url;
