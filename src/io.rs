use std::{
    fs::File,
    error::Error,
    io::Write,
};
use crate::Config;

pub fn write_output_to_file(config: &Config, file_name: &str, output: String) -> Result<(), Box<dyn Error>> {
    let output_dir = &config.get_output_directory();
    let path = format!("{output_dir}/{file_name}");

    println!("Writing file {path}...");

    let mut file_to_write = File::create(&path)?;
    file_to_write.write(output.as_bytes())?;

    println!("Successfully wrote to {path}!");

    Ok(())
}
