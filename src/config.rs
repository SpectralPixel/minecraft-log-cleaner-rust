use std::error::Error;
use std::fs;
use serde::Deserialize;
use self::blacklist::Blacklist;
use crate::formatter;

mod blacklist;

#[derive(Deserialize)]
pub struct Config {
    date_range: [usize; 2],
    time_range: [usize; 2],
    content_start_char_pos: usize,
    raw_log_path: String,
    output_directory: String,
    blacklist: Blacklist,
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

    pub fn blacklist(&self) -> &Blacklist {
        &self.blacklist
    }

    pub fn auto_blacklist(&mut self, provided_log: &String) {
        let auto_blacklist_passes = self.blacklist.get_auto_blacklist_passes();
        println!("Engaging auto-blacklisting... ({auto_blacklist_passes} passes)");
        for pass in 1..=auto_blacklist_passes {
            let filtered_log = formatter::filter_log(&self, &provided_log);
            let mut line_counts = formatter::get_line_counts(&self, &filtered_log);
            let mut auto_blacklisted_lines: Vec<String> = Vec::new();
            for line in line_counts.drain().filter(|x| x.1 > self.blacklist.get_auto_blacklist_percentage()) {
                auto_blacklisted_lines.push(line.0.clone());
                let key = line.0;
                let val = (line.1 * 10000.).floor() / 10000.;
                println!("Blacklisted (pass {pass}): {key} | {val}%")
            }
            self.blacklist.add_to_blacklist(&mut auto_blacklisted_lines);
        }
        println!("Auto-blacklisting complete!")
    }
}
