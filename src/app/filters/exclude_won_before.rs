use crate::{app::Winners, steam::SteamUser};

#[inline]
pub fn exclude_who_won_before(friends: &mut Vec<SteamUser>, winners: &Winners) {
    friends.retain(|friend| !winners.all_time.contains_key(&friend.id));
}
