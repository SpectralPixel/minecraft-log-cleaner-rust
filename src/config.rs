use serde::Deserialize;
use crate::{
    config::blacklist::Blacklist,
    formatter::{self, Formatting},
    io::Files,
};
use std::{
    fs,
    error::Error,
};

mod blacklist;

#[derive(Deserialize)]
pub struct Config {
    formatting: Formatting,
    files: Files,
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

    pub fn formatting(&self) -> &Formatting {
        &self.formatting
    }

    pub fn files(&self) -> &Files {
        &self.files
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
