use super::{SteamId, SteamUser};
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GetUserSummaries {
    players: Vec<SteamUser>,
}

pub fn execute_request(
    http: &Client,
    access_token: &str,
    user_ids: &[SteamId],
) -> Result<Vec<SteamUser>, reqwest::Error> {
    const URL: &str = "https://api.steampowered.com/ISteamUserOAuth/GetUserSummaries/v1";
    http.get(URL)
        .query(&[
            ("access_token", access_token),
            (
                "steamids",
                &user_ids
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            ),
        ])
        .send()?
        .json::<GetUserSummaries>()
        .map(|res| res.players)
}
