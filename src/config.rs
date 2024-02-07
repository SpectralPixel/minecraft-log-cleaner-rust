use std::error::Error;
use std::fs;
use config_error::UsageError;

pub mod config_error;

pub struct Config {
    raw_log: String,
    _config_file: fs::File, // use this to load config, remove the file from here later as it will no longer be needed after initialization
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let raw_log: String = match args.get(1) {
            Some(v) => {
                let filename = v.clone();
                let contents = fs::read_to_string(filename)?;
                contents
            },
            None => return Err(Box::new(UsageError(String::from("Failure reading argument 1."))))
        };
        let config_file_name: String = match args.get(2) {
            Some(v) => v.clone(),
            None => return Err(Box::new(UsageError(String::from("Failure reading argument 2."))))
        };

        Ok(Config {
            raw_log,
            _config_file: fs::File::open(config_file_name)?,
        })
    }

    pub fn raw_log(&self) -> &String {
        &self.raw_log
    }
}
