use reqwest::blocking::Client;
use serde::Deserialize;

use super::SteamId;

#[derive(Debug, Deserialize)]
struct GetUserCountry {
    response: Response,
}

#[derive(Debug, Deserialize)]
struct Response {
    country: String,
}

pub fn execute_request(
    http: &Client,
    access_token: &str,
    user_id: SteamId,
) -> Result<String, reqwest::Error> {
    const URL: &str = "https://api.steampowered.com/IUserAccountService/GetUserCountry/v1";
    http.post(URL)
        .form(&[
            ("access_token", access_token),
            ("steamid", &user_id.to_string()),
        ])
        .send()
        .inspect(|r| println!("{:#?}", r))
        .inspect_err(|e| eprintln!("{:#?}", e))?
        .json::<GetUserCountry>()
        .map(|res| res.response.country)
}
