use std::error::Error;

pub use config::Config;

mod config;
mod formatter;
mod io;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut config = Config::build()?;
    let raw_log = config.get_raw_log()?;

    if config.get_auto_blacklist() {
        todo!(); // MAKE SURE TO FILTER OUT ACTIVITY THAT IS 100% KNOWN TO BE PLAYER ACTIVITY BEFORE SORTING LINES. ONLY SYSTEM MESSAGES SHOULD BE SORTED OUT
        config.auto_blacklist(&raw_log);
    }

    let filtered_log = formatter::filter_log(&config, &raw_log);

    io::write_output_to_file(&config, &"filtered.log", filtered_log)?;

    Ok(())
}
