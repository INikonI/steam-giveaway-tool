use std::collections::HashMap;

use crate::steam::{SteamId, SteamUser};

use super::friends::Friends;

#[derive(Default)]
pub struct Winners {
    pub next_number: usize,

    pub current: Vec<SteamUser>,
    pub all_time: HashMap<SteamId, usize>, // value is number of wins

    pub saved: bool,
    pub auto_save_current: bool,
}

impl Winners {
    pub fn update_current(&mut self, friends: &Friends) {
        use rand::seq::IteratorRandom;

        let mut rng = rand::rng();
        self.current = friends
            .filtered
            .iter()
            .choose_multiple(&mut rng, self.next_number)
            .into_iter()
            .cloned()
            .collect();

        if self.auto_save_current {
            self.save_current();
        } else {
            self.saved = false;
        }
    }

    pub fn save_current(&mut self) {
        self.saved = true;
        for winner in &self.current {
            if let Some(times) = self.all_time.get_mut(&winner.id) {
                *times += 1;
            } else {
                self.all_time.insert(winner.id, 1);
            }
        }
    }
}
