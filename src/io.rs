use serde::Deserialize;
use std::{
    fs::{
        self,
        File,
    },
    error::Error,
    io::Write,
};

#[derive(Deserialize)]
pub struct Files {
    raw_log_path: String,
    output_directory: String,
}

impl Files {
    pub fn get_raw_log(&self) -> Result<String, Box<dyn Error>> {
        let path = &self.raw_log_path;
        println!("Reading raw log file...  ({path})");
        let raw_log = fs::read_to_string(path)?;
        println!("Loaded contents of raw log file!");
        Ok(raw_log)
    }

    pub fn write_output_to_file(&self, file_name: &str, output: String) -> Result<(), Box<dyn Error>> {
        let output_dir = &self.output_directory;
        let path = format!("{output_dir}/{file_name}");
    
        println!("Writing file {path}...");
    
        let mut file_to_write = File::create(&path)?;
        file_to_write.write(output.as_bytes())?;
    
        println!("Successfully wrote to {path}!");
    
        Ok(())
    }
}
