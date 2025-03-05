use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Preferences {
    pub avatars: bool,
    pub flags_icons: bool,
    pub store_items_capsules: bool,
}

impl Default for Preferences {
    #[inline]
    fn default() -> Self {
        Self {
            avatars: true,
            flags_icons: true,
            store_items_capsules: true,
        }
    }
}
