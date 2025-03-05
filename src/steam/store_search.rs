use reqwest::blocking::Client;
use serde::Deserialize;

use super::store_item::SteamStoreItem;

#[derive(Debug, Deserialize)]
struct StoreSearch {
    // pub total: usize,
    items: Vec<SteamStoreItem>,
}

pub fn execute_request(
    http: &Client,
    term: &str,
    country_code: Option<&str>,
) -> Result<Vec<SteamStoreItem>, reqwest::Error> {
    const URL: &str = "https://store.steampowered.com/api/storesearch";
    http.get(URL)
        .query(&[
            ("term", term),
            ("l", "english"),
            ("cc", country_code.unwrap_or("us")),
        ])
        .send()?
        .json::<StoreSearch>()
        .map(|res| res.items)
}
