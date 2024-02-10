use std::error::Error;
use std::fs;
use std::vec::IntoIter;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    raw_log_path: String,
    filtered_log_path: String,
    blacklist: Vec<String>,
}

impl Config {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let config = fs::read_to_string("Config.toml")?;
        let config: Config = toml::from_str(&config)?;
        Ok(config)
    }

    pub fn read_raw_log(&self) -> Result<String, Box<dyn Error>> {
        let raw_log = fs::read_to_string(&self.raw_log_path)?;
        Ok(raw_log)
    }

    pub fn get_filtered_log_path(&self) -> &String {
        &self.filtered_log_path
    }

    pub fn get_blacklist(&self) -> IntoIter<String> {
        let blacklist_clone = self.blacklist.clone();
        blacklist_clone.into_iter()
    }
}
