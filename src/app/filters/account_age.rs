use chrono::Utc;

use crate::steam::SteamUser;

use super::Filters;

pub fn apply_account_age_filter(friends: &mut Vec<SteamUser>, filters: &Filters) {
    if filters.account_age <= 0 && !filters.exclude_unknown_age {
        return;
    }

    let now = Utc::now();
    friends.retain(|friend| {
        friend
            .created_at
            .map(|created_at| now.years_since(created_at).unwrap() >= filters.account_age)
            .unwrap_or(!filters.exclude_unknown_age)
    });
}
