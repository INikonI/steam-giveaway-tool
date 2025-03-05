use super::{SteamUser, SteamId};
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GetPlayerSummaries {
    response: Response,
}

#[derive(Debug, Deserialize)]
struct Response {
    players: Vec<SteamUser>,
}

pub fn execute_request(
    http: &Client,
    access_token: &str,
    user_ids: &[SteamId],
) -> Result<Vec<SteamUser>, reqwest::Error> {
    const URL: &str = "https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2";
    http.get(URL)
        .query(&[
            ("access_token", access_token),
            ("key", access_token),
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
        .json::<GetPlayerSummaries>()
        .map(|res| res.response.players)
}
