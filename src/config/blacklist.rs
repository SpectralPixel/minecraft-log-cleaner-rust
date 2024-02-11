use serde::Deserialize;
use std::vec::IntoIter;

#[derive(Deserialize)]
pub struct Blacklist {
    blacklist: Vec<String>,
    auto_blacklist: bool,
    auto_blacklist_percentage: f64,
    auto_blacklist_passes: usize,
}

impl Blacklist {
    pub fn get_blacklist_iter(&self) -> IntoIter<String> {
        let blacklist_clone = self.blacklist.clone();
        blacklist_clone.into_iter()
    }

    pub fn add_to_blacklist(&mut self, new_items: &mut Vec<String>) {
        self.blacklist.append(new_items);
    }

    pub fn get_auto_blacklist(&self) -> bool {
        self.auto_blacklist
    }

    pub fn get_auto_blacklist_percentage(&self) -> f64 {
        self.auto_blacklist_percentage
    }

    pub fn get_auto_blacklist_passes(&self) -> usize {
        self.auto_blacklist_passes
    }
}
