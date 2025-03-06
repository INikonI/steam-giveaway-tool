use std::collections::HashMap;

use super::{SteamId, StoreItemId};
use reqwest::blocking::Client;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
struct Response {
    #[serde(flatten, deserialize_with = "deserialize_userdetails")]
    apps: HashMap<StoreItemId, Option<StoreItemUserDetails>>,
}

#[derive(Debug, Deserialize)]
struct DataWrapper {
    data: Option<StoreItemUserDetails>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FriendOwn {
    #[serde(rename = "steamid")]
    pub id: SteamId,
    pub playtime_twoweeks: u16, // in minutes
    pub playtime_total: u32,    // in minutes
}

#[derive(Debug, Clone, Deserialize)]
pub struct FriendWant {
    #[serde(rename = "steamid")]
    pub id: SteamId,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StoreItemUserDetails {
    #[serde(rename = "friendsown", default)]
    pub friends_own: Vec<FriendOwn>,
    #[serde(rename = "friendswant", default)]
    pub friends_want: Vec<FriendWant>,
}

fn deserialize_userdetails<'de, D>(
    deserializer: D,
) -> Result<HashMap<StoreItemId, Option<StoreItemUserDetails>>, D::Error>
where
    D: Deserializer<'de>,
{
    let wrapped_user_details: HashMap<StoreItemId, DataWrapper> =
        Deserialize::deserialize(deserializer)?;
    let mut user_details = HashMap::new();
    for (id, data) in wrapped_user_details.into_iter() {
        user_details.insert(id, data.data);
    }
    Ok(user_details)
}

pub fn execute_request(
    http: &Client,
    app_ids: &[StoreItemId],
) -> Result<HashMap<StoreItemId, Option<StoreItemUserDetails>>, reqwest::Error> {
    const URL: &str = "https://store.steampowered.com/api/appuserdetails";
    http.get(URL)
        .query(&[(
            "appids",
            app_ids
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(","),
        )])
        .send()?
        .json::<Response>()
        .map(|res| res.apps)
}
