use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{Arc, LazyLock};

use crate::app::Msg;

use super::get_friend_list::{self, Friend, RelationshipFilter};
use super::{
    SteamId, SteamStoreItem, SteamUser, StoreItemId, StoreItemUserDetails,
    get_current_user_country, get_user_summaries, store_item_user_details, store_search,
};
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use regex::Regex;
use reqwest::{Url, blocking::Client, cookie::Jar};
use serde::Deserialize;

pub struct SteamApiClient {
    http: Client,
    cookies: Arc<Jar>,

    pub access_token: AccessToken,

    pub current_user: Option<SteamUser>,
}

pub struct AccessToken {
    pub info: Result<TokenInfo, TokenError>,
    pub token: String,
}

impl AccessToken {
    #[inline]
    pub fn is_expired(&self) -> bool {
        if let Ok(ref token_info) = self.info {
            if Utc::now() >= token_info.expires_on {
                return true;
            }
        }
        false
    }
}

impl Default for AccessToken {
    #[inline]
    fn default() -> Self {
        Self {
            info: Err(TokenError::EmptyString),
            token: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenInfo {
    #[serde(rename = "sub")]
    pub user_id: SteamId,
    #[serde(rename = "exp", with = "ts_seconds")]
    pub expires_on: DateTime<Utc>,
}

#[derive(Debug, Default)]
pub enum TokenError {
    #[default]
    EmptyString,
    ParseFailed,
    Expired,
}

impl SteamApiClient {
    pub fn new() -> Self {
        let cookie_store = Arc::new(Jar::default());
        Self {
            http: Client::builder()
                .cookie_provider(cookie_store.clone())
                .build()
                .unwrap(),
            cookies: cookie_store,

            access_token: Default::default(),

            current_user: None,
        }
    }

    pub fn set_access_token(&mut self, token: &str) {
        let token = token.trim();

        self.access_token.info = parse_access_token(token);
        self.access_token.token = token.to_owned();

        if let Ok(ref token_info) = self.access_token.info {
            let steam_urls = &[
                Url::parse("https://steampowered.com").unwrap(),
                Url::parse("https://api.steampowered.com").unwrap(),
                Url::parse("https://store.steampowered.com").unwrap(),
            ];

            for url in steam_urls {
                self.cookies.add_cookie_str(
                    &format!("steamLoginSecure={}%7C%7C{}", token_info.user_id, token),
                    url,
                );
            }
        }
    }

    pub fn update_current_user(&mut self, sender: Sender<Msg>) {
        if let Ok(ref token_info) = self.access_token.info {
            if let Ok(mut user) = self
                .get_user_summaries(&[token_info.user_id])
                .map(|users| users.into_iter().next().unwrap())
            {
                user.country_code = Some(
                    self.get_current_user_country()
                        .expect("Current user country should be getted"),
                );
                let _ = sender.send(Msg::UpdateCurrentUser(user));
            }
        }
    }

    fn get_current_user_country(&self) -> Result<String, reqwest::Error> {
        get_current_user_country::execute_request(
            &self.http,
            &self.access_token.token,
            self.access_token.info.as_ref().unwrap().user_id,
        )
    }

    pub fn get_friend_list(
        &self,
        relationship: RelationshipFilter,
        user_id: Option<SteamId>,
    ) -> Result<Vec<Friend>, reqwest::Error> {
        get_friend_list::execute_request(
            &self.http,
            &self.access_token.token,
            relationship,
            user_id,
        )
    }

    pub fn get_user_summaries(
        &self,
        user_ids: &[SteamId],
    ) -> Result<Vec<SteamUser>, reqwest::Error> {
        get_user_summaries::execute_request(&self.http, &self.access_token.token, user_ids)
    }

    pub fn app_user_details(
        &self,
        app_ids: &[StoreItemId],
    ) -> Result<HashMap<StoreItemId, Option<StoreItemUserDetails>>, reqwest::Error> {
        store_item_user_details::execute_request(&self.http, app_ids)
    }

    pub fn store_search(
        &self,
        term: &str,
        country_code: Option<&str>,
    ) -> Result<Vec<SteamStoreItem>, reqwest::Error> {
        store_search::execute_request(&self.http, term, country_code)
    }
}

fn parse_access_token(token: &str) -> Result<TokenInfo, TokenError> {
    if token.is_empty() {
        return Err(TokenError::EmptyString);
    }

    static JWT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
        r"^(?P<header>[A-Za-z0-9_-]+)\.(?P<payload>[A-Za-z0-9_-]+)\.(?P<signature>[A-Za-z0-9_-]+)$")
        .unwrap()
    });

    let jwt = JWT_PATTERN.captures(token).ok_or(TokenError::ParseFailed)?;
    let payload = jwt.name("payload").ok_or(TokenError::ParseFailed)?.as_str();
    let decoded_payload = BASE64_URL_SAFE_NO_PAD
        .decode(payload)
        .map_err(|_| TokenError::ParseFailed)?;
    serde_json::from_slice(&decoded_payload).map_err(|_| TokenError::ParseFailed)
}
