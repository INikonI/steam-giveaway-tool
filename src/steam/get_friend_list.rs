use std::fmt::Display;

use super::SteamId;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct GetFriendList {
    friends: Vec<Friend>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Friend {
    #[serde(rename = "steamid")]
    pub id: SteamId,
    pub relationship: Relationship,
}

#[derive(Deserialize)]
pub enum Relationship {
    #[serde(alias = "friend")]
    Friend,
    #[serde(alias = "ignored")]
    Ignored,
}

#[allow(dead_code)]
pub enum RelationshipFilter {
    All,
    Friend,
    Ignored,
}

impl Display for RelationshipFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::All => "all",
            Self::Friend => "friend",
            Self::Ignored => "ignored",
        })
    }
}

pub fn execute_request(
    http: &Client,
    access_token: &str,
    relationship: RelationshipFilter,
    user_id: Option<SteamId>,
) -> Result<Vec<Friend>, reqwest::Error> {
    const URL: &str = "https://api.steampowered.com/ISteamUserOAuth/GetFriendList/v1";
    http.get(URL)
        .query(&[
            ("access_token", access_token),
            ("relationship", &relationship.to_string()),
            (
                "steamid",
                &user_id.map(|id| id.to_string()).unwrap_or_default(),
            ),
        ])
        .send()
        .inspect(|r| println!("{:#?}", r))
        .inspect_err(|e| eprintln!("{:#?}", e))?
        .json::<GetFriendList>()
        .map(|res| res.friends)
}
