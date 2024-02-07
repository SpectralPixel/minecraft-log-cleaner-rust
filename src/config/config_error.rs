use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct UsageError(pub String);

impl fmt::Display for UsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_message = &self.0;
        let error_message = format!("{error_message}\nType \"[program name] -- help\" to get more info on how to use this tool.");
        write!(f, "{error_message}")
    }
}

impl Error for UsageError {}
