use std::{
    collections::HashSet,
    sync::{Arc, RwLock, mpsc::Sender},
    thread,
    time::Duration,
};

use std::thread::sleep;

use crate::steam::{RelationshipFilter, SteamApiClient, SteamUser, StoreItemUserDetails};

use super::{
    Msg, Winners, apply_account_age_filter, apply_has_app_filters, apply_region_filters,
    exclude_who_won_before, filters::Filters, include_who_has_store_item_in_wishlist,
};

#[derive(Default)]
pub struct Friends {
    pub all: Vec<SteamUser>,
    pub filtered: Vec<SteamUser>,
    pub regions: Vec<String>,

    pub is_loading: bool,
    pub loading_progress: f32,
}

impl Friends {
    pub fn update(&mut self, steam: Arc<RwLock<SteamApiClient>>, sender: Sender<Msg>) {
        self.is_loading = true;
        thread::spawn(move || {
            let sender = sender;
            let friends_and_regions = fetch_friends(steam, &sender);
            sender
                .send(Msg::UpdateFriends(friends_and_regions))
                .expect("Message should be sended");
        });
    }

    pub fn update_filtered(
        &mut self,
        filters: &Filters,
        winners: &Winners,
        app_user_details: Option<&StoreItemUserDetails>,
    ) {
        if self.all.is_empty() {
            return;
        }

        let mut filtered_friends = self.all.clone();

        apply_account_age_filter(&mut filtered_friends, filters);
        if filters.exclude_who_won_before {
            exclude_who_won_before(&mut filtered_friends, winners);
        }
        apply_region_filters(&mut filtered_friends, &filters.regions_and_countries);
        if filters.include_who_has_app_in_wishlist {
            include_who_has_store_item_in_wishlist(&mut filtered_friends, app_user_details);
        }
        apply_has_app_filters(&mut filtered_friends, &filters.has_store_items);

        self.filtered = filtered_friends;
    }
}

pub struct FriendsAndTheirRegions(pub Vec<SteamUser>, pub Vec<String>);

pub fn fetch_friends(
    steam: Arc<RwLock<SteamApiClient>>,
    sender: &Sender<Msg>,
) -> FriendsAndTheirRegions {
    let mut unique_regions = HashSet::new();

    let friends = {
        let steam = steam.read().unwrap();
        let mut list = vec![];

        let friends = steam
            .get_friend_list(RelationshipFilter::Friend, None)
            .unwrap();
        let chunks = friends.chunks(100);
        let chunks_count = chunks.len();

        for (n, chunk) in chunks.enumerate() {
            if let Ok(data) =
                steam.get_user_summaries(&chunk.iter().map(|f| f.id).collect::<Vec<_>>())
            {
                list.extend(data);
            }
            sender
                .send(Msg::UpdateFriendsLoadProgress(
                    (n + 1) as f32 / chunks_count as f32,
                ))
                .expect("Message should be sended");
        }

        list
    };

    for friend in &friends {
        if let Some(ref country_code) = friend.country_code {
            unique_regions.insert(country_code.to_owned());
        }
    }

    let mut regions: Vec<_> = unique_regions.into_iter().collect();
    regions.sort();

    FriendsAndTheirRegions(friends, regions)
}
