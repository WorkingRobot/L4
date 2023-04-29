#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Currency {
    // Type of currency (only seen "REAL")
    #[serde(rename = "type")]
    currency_type: String,

    // Code of the currency (e.g. "USD", "EUR", "GBP")
    code: String,

    // Symbol of the currency (e.g. "$")
    symbol: String,

    // Description of the currency (sometimes explains it, other times looks like a placeholder)
    description: String,

    // Number of decimals in the currency (e.g. .00 = 2 in USD for cents)
    decimals: i32,

    // Unsure, only seen 0 here
    trunc_length: i32,

    // Not too sure what it's used for, but all strings are like "[0,number]"
    price_ranges: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PageInfo {
    // Number of total elements requested
    count: i32,

    // Start index of elements requested
    start: i32,

    // Total elements in the endpoi32
    total: i32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetCurrencies {
    // Listed currencies
    elements: Vec<Currency>,

    // Page info depending on query params
    paging: PageInfo,
}
