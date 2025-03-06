mod client;
mod get_current_user_country;
mod get_friend_list;
mod get_user_summaries;
mod store_item;
mod store_item_user_details;
mod store_search;
mod user;

use std::sync::LazyLock;

use chrono::{DateTime, Datelike, Utc};
pub use client::*;
pub use get_friend_list::*;
pub use store_item::*;
pub use store_item_user_details::*;
pub use user::*;

pub static STEAM_FOUNDATION_DATE: LazyLock<DateTime<Utc>> = LazyLock::new(|| {
    DateTime::default()
        .with_year(2003)
        .unwrap()
        .with_month(9)
        .unwrap()
        .with_day(12)
        .unwrap()
});
