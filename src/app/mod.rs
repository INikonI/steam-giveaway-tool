mod filters;
mod friends;
mod preferences;
mod storage_key;
mod winners;

use egui_notify::Toasts;
pub use filters::*;
pub use friends::*;
pub use preferences::*;
use reqwest::{
    blocking::Client,
    header::{ACCEPT, USER_AGENT},
};
pub use winners::*;

use eframe::{CreationContext, Frame, egui::Context};
use egui_extras::install_image_loaders;
use std::{
    sync::{
        Arc, RwLock,
        mpsc::{Receiver, Sender},
    },
    thread,
};

use crate::{
    steam::{
        SteamApiClient, SteamStoreItem, SteamUser, StoreItemId, StoreItemUserDetails, TokenError,
    },
    ui::{
        SearchSelect, main_window, settings_window, style_override, update_window, winners_window,
    },
};

pub enum Msg {
    AccessTokenSetted,
    NewVersionAvailable,
    UpdateFoundedFriends,
    UpdateUserDetailsOfAppForGiveaway(StoreItemUserDetails),
    UpdateUserDetailsOfHasAppFilter(StoreItemId, StoreItemUserDetails),
    UpdateCurrentUser(SteamUser),
    UpdateFriends(FriendsAndTheirRegions),
    UpdateFriendsLoadProgress(f32),
}

pub struct App {
    pub toasts: Toasts,

    pub steam: Arc<RwLock<SteamApiClient>>,
    pub steam_access_token_buffer: String,

    pub friends: Friends,
    pub friends_search_name: String,
    pub friends_search_results: Vec<SteamUser>,

    pub store_item_for_giveaway: Option<SteamStoreItem>,
    pub app_for_giveaway_user_details_is_loading: bool,

    pub search_select: SearchSelect,

    pub winners: Winners,

    pub filters: Filters,

    pub preferences: Preferences,

    pub sender: Sender<Msg>,
    pub receiver: Receiver<Msg>,

    pub main_current_page: usize,

    pub show_settings_window: bool,
    pub show_winners_window: bool,
    pub show_update_window: bool,
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        install_image_loaders(&cc.egui_ctx);

        let mut steam = SteamApiClient::new();
        let mut steam_access_token = String::new();
        let mut winners = Winners::default();
        let mut preferences = Preferences::default();

        let (sender, receiver) = std::sync::mpsc::channel();

        if let Some(storage) = cc.storage {
            if let Some(token) = storage.get_string(storage_key::ACCESS_TOKEN) {
                steam.set_access_token(&token);
                steam_access_token = token.to_owned();
                let _ = sender.send(Msg::AccessTokenSetted);
            }

            if let Some(raw_str) = storage.get_string(storage_key::ALL_TIME_WINNERS) {
                if let Ok(all_time_winners) = serde_json::from_str(&raw_str) {
                    winners.all_time = all_time_winners;
                }
            }

            if let Some(raw_str) = storage.get_string(storage_key::AUTO_SAVE_ALL_TIME_WINNERS) {
                if let Ok(auto_save) = serde_json::from_str(&raw_str) {
                    winners.auto_save_current = auto_save;
                }
            }

            if let Some(raw_str) = storage.get_string(storage_key::PREFERENCES) {
                if let Ok(prefs) = serde_json::from_str(&raw_str) {
                    preferences = prefs;
                }
            }
        }

        cc.egui_ctx.style_mut(|style| {
            style_override(style);
        });

        let toasts = Toasts::new();

        thread::spawn(check_for_updates(sender.clone()));

        Self {
            toasts,

            steam: Arc::new(steam.into()),
            steam_access_token_buffer: steam_access_token,

            friends: Default::default(),
            friends_search_name: Default::default(),
            friends_search_results: Default::default(),

            store_item_for_giveaway: Default::default(),
            app_for_giveaway_user_details_is_loading: false,

            search_select: SearchSelect::new(),

            winners,

            filters: Default::default(),

            preferences,

            sender,
            receiver,

            main_current_page: 1,
            show_settings_window: false,
            show_winners_window: false,

            show_update_window: false,
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        storage.set_string(
            storage_key::ACCESS_TOKEN,
            self.steam.read().unwrap().access_token.token.clone(),
        );
        storage.set_string(
            storage_key::ALL_TIME_WINNERS,
            serde_json::to_string(&self.winners.all_time).unwrap(),
        );
        storage.set_string(
            storage_key::AUTO_SAVE_ALL_TIME_WINNERS,
            serde_json::to_string(&self.winners.auto_save_current).unwrap(),
        );
        storage.set_string(
            storage_key::PREFERENCES,
            serde_json::to_string(&self.preferences).unwrap(),
        );
    }

    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if let Ok(msg) = self.receiver.try_recv() {
            match msg {
                Msg::AccessTokenSetted => {
                    let steam = self.steam.read().unwrap();
                    if let Ok(ref token_info) = steam.access_token.info {
                        if steam
                            .current_user
                            .as_ref()
                            .map(|current_user| current_user.id != token_info.user_id)
                            .unwrap_or(true)
                        {
                            let steam = self.steam.clone();
                            let sender = self.sender.clone();
                            thread::spawn(move || {
                                steam.write().unwrap().update_current_user(sender);
                            });
                        }
                    }
                }
                Msg::NewVersionAvailable => {
                    self.show_update_window = true;
                }
                Msg::UpdateUserDetailsOfAppForGiveaway(app_user_details) => {
                    self.app_for_giveaway_user_details_is_loading = false;
                    if let Some(ref mut app) = self.store_item_for_giveaway {
                        app.user_details = Some(app_user_details);
                    }
                }
                Msg::UpdateUserDetailsOfHasAppFilter(app_id, app_user_details) => {
                    if let Some(has_app_filter) =
                        self.filters.has_store_items.iter_mut().find(|filter| {
                            filter
                                .app
                                .as_ref()
                                .map(|app| app.id == app_id)
                                .unwrap_or_default()
                        })
                    {
                        if let Some(app) = has_app_filter.app.as_mut() {
                            app.user_details = Some(app_user_details);
                        }
                        has_app_filter.is_loading = false;
                    }
                }
                Msg::UpdateFoundedFriends => {
                    if self.friends_search_name.is_empty() {
                        self.friends_search_results = Default::default();
                    } else {
                        self.friends_search_results = self
                            .friends
                            .all
                            .iter()
                            .filter(|friend| {
                                friend
                                    .name
                                    .to_lowercase()
                                    .contains(&self.friends_search_name.to_lowercase())
                            })
                            .cloned()
                            .collect();
                    }
                }
                Msg::UpdateCurrentUser(user) => {
                    self.steam.write().unwrap().current_user = Some(user);
                    self.friends.update(self.steam.clone(), self.sender.clone());
                }
                Msg::UpdateFriends(FriendsAndTheirRegions(friends, regions)) => {
                    self.friends.is_loading = false;
                    self.friends.loading_progress = 0.;
                    self.friends.all = friends;
                    self.friends.regions = regions.clone();

                    self.friends_search_name = Default::default();
                    self.friends_search_results = Default::default();

                    self.filters.reset_regions_and_countries(&self.friends);
                    self.filters.has_store_items = Default::default();

                    self.toasts.success("Friends list loaded!");
                }
                Msg::UpdateFriendsLoadProgress(progress) => {
                    self.friends.loading_progress = progress;
                }
            }
        }

        if self.steam.read().unwrap().access_token.is_expired() {
            self.steam.write().unwrap().access_token.info = Err(TokenError::Expired);
        }

        main_window(self, ctx);
        winners_window(self, ctx);
        settings_window(self, ctx);
        update_window(self, ctx);

        self.toasts.show(ctx);
    }
}

#[inline]
fn check_for_updates(sender: Sender<Msg>) -> impl FnOnce() {
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct GithubRelease {
        pub tag_name: String,
    }

    move || {
        let client = Client::new();
        if let Ok(response) = client
            .get("https://api.github.com/repos/INikonI/steam-giveaway-tool/releases/latest")
            .header(ACCEPT, "application/vnd.github.v3+json")
            .header(USER_AGENT, env!("CARGO_PKG_NAME"))
            .send()
        {
            if let Ok(release) = response.json::<GithubRelease>() {
                if env!("CARGO_PKG_VERSION") != &release.tag_name[1..] {
                    let _ = sender.send(Msg::NewVersionAvailable);
                }
            }
        }
    }
}
