#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    // Type of currency (only seen "REAL")
    #[serde(rename = "type")]
    pub currency_type: String,

    // Code of the currency (e.g. "USD", "EUR", "GBP")
    pub code: String,

    // Symbol of the currency (e.g. "$")
    pub symbol: String,

    // Description of the currency (sometimes explains it, other times looks like a placeholder)
    pub description: String,

    // Number of decimals in the currency (e.g. .00 = 2 in USD for cents)
    pub decimals: i32,

    // Unsure, only seen 0 here
    pub trunc_length: i32,

    // Not too sure what it's used for, but all strings are like "[0,number]"
    pub price_ranges: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    // Number of total elements requested
    pub count: i32,

    // Start index of elements requested
    pub start: i32,

    // Total elements in the endpoi32
    pub total: i32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrencies {
    // Listed currencies
    pub elements: Vec<Currency>,

    // Page info depending on query params
    pub paging: PageInfo,
}
