use crate::steam::{SteamUser, StoreItemUserDetails};

#[inline]
pub fn include_who_has_store_item_in_wishlist(
    friends: &mut Vec<SteamUser>,
    store_item_user_details: Option<&StoreItemUserDetails>,
) {
    if let Some(store_item_user_details) = store_item_user_details {
        friends.retain(|friend| {
            store_item_user_details
                .friends_want
                .iter()
                .any(|f| f.id == friend.id)
        });
    }
}
