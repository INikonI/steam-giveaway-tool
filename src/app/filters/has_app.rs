use std::hash::Hash;

use crate::steam::{SteamStoreItem, SteamUser};

#[derive(Debug, Default)]
pub struct HasAppFilter {
    pub app: Option<SteamStoreItem>,
    // achievement_percent: u8,
    pub playtime_twoweeks: u16, // in hours
    pub playtime_total: u32,    // in hours

    pub is_loading: bool,
}

impl PartialEq for HasAppFilter {
    fn eq(&self, other: &Self) -> bool {
        self.app
            .as_ref()
            .map(|app| {
                other
                    .app
                    .as_ref()
                    .map(|other_app| app.id == other_app.id)
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }
}

impl Eq for HasAppFilter {}

impl Hash for HasAppFilter {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.app
            .as_ref()
            .map(|app| app.id)
            .unwrap_or_default()
            .hash(state);
    }
}

pub fn apply_has_app_filters(friends: &mut Vec<SteamUser>, filters: &[HasAppFilter]) {
    if filters.is_empty() {
        return;
    }

    let filters = filters
        .iter()
        .filter(|f| {
            !f.is_loading
                && f.app
                    .as_ref()
                    .map(|app| app.user_details.is_some())
                    .unwrap_or_default()
        })
        .collect::<Vec<&HasAppFilter>>();

    if filters.is_empty() {
        return;
    }

    friends.retain(|friend| {
        for filter in &filters {
            let app_user_details = filter.app.as_ref().unwrap().user_details.as_ref().unwrap();
            let friend_own_option = app_user_details
                .friends_own
                .iter()
                .find(|fo| fo.id == friend.id);
            if let Some(friend_own) = friend_own_option {
                if friend_own.playtime_total < filter.playtime_total * 60
                    || friend_own.playtime_twoweeks < filter.playtime_twoweeks * 60
                {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    });
}
