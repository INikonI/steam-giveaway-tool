mod exclude_won_before;
mod has_app;
mod include_app_in_wishlist;
mod region;

pub use exclude_won_before::*;
pub use has_app::*;
pub use include_app_in_wishlist::*;
pub use region::*;

use super::friends::Friends;

#[derive(Default)]
pub struct Filters {
    pub regions_and_countries: RegionsAndCountriesFilter,
    pub has_store_items: Vec<HasAppFilter>,

    pub include_who_has_app_in_wishlist: bool,
    pub exclude_who_won_before: bool,
}

impl Filters {
    pub fn reset(&mut self, friends: &Friends) {
        self.has_store_items = Default::default();
        self.include_who_has_app_in_wishlist = false;
        self.exclude_who_won_before = false;
        self.reset_regions_and_countries(friends);
    }

    #[inline]
    pub fn reset_regions_and_countries(&mut self, friends: &Friends) {
        self.regions_and_countries = Default::default();
        self.regions_and_countries.available_countries = friends.regions.clone();
    }
}
