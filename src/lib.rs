use std::{
    error::Error, 
    fs::File,
    io::Write,
};
use config::Config;

pub mod config;
pub mod formatter;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut config = Config::build()?;
    let raw_log = config.get_raw_log()?;

    if config.blacklist().get_auto_blacklist() {
        todo!(); // MAKE SURE TO FILTER OUT ACTIVITY THAT IS 100% KNOWN TO BE PLAYER ACTIVITY BEFORE SORTING LINES. ONLY SYSTEM MESSAGES SHOULD BE SORTED OUT
        config.auto_blacklist(&raw_log);
    }

    let filtered_log = formatter::filter_log(&config, &raw_log);

    write_output_to_file(&config, &"filtered.log", filtered_log)?;

    Ok(())
}

fn write_output_to_file(config: &Config, file_name: &str, output: String) -> Result<(), Box<dyn Error>> {
    let output_dir = config.output_directory();
    let path = format!("{output_dir}/{file_name}");

    println!("Writing file {path}...");

    let mut file_to_write = File::create(&path)?;
    file_to_write.write(output.as_bytes())?;

    println!("Successfully wrote to {path}!");

    Ok(())
}
