use std::error::Error;
use std::fs;
use std::vec::IntoIter;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    date_range: [usize; 2],
    time_range: [usize; 2],
    content_start_char_pos: usize,
    raw_log_path: String,
    output_directory: String,
    blacklist: Vec<String>,
    auto_blacklist: bool,
    auto_blacklist_percentage: f64,
}

impl Config {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        println!("Loading configuration file... (Config.toml)");
        let config = fs::read_to_string("Config.toml")?;
        let config: Config = toml::from_str(&config)?;
        println!("\"Config.toml\" loaded!");
        Ok(config)
    }

    pub fn get_date_range(&self) -> (usize, usize) {
        (self.date_range[0], self.date_range[1])
    }

    pub fn get_time_range(&self) -> (usize, usize) {
        (self.time_range[0], self.time_range[1])
    }

    pub fn get_content_start_pos(&self) -> usize {
        self.content_start_char_pos
    }

    pub fn get_raw_log(&self) -> Result<String, Box<dyn Error>> {
        let path = &self.raw_log_path;
        println!("Reading raw log file...  ({path})");
        let raw_log = fs::read_to_string(path)?;
        println!("Loaded contents of raw log file!");
        Ok(raw_log)
    }

    pub fn output_directory(&self) -> &String {
        &self.output_directory
    }

    pub fn get_blacklist(&self) -> IntoIter<String> {
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
}
