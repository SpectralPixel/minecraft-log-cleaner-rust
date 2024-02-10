use std::{
    error::Error,
    fs::File,
    io::Write
};

pub mod config;

pub fn run() -> Result<(), Box<dyn Error>> {
    let config = config::Config::build()?;
    let raw_log = config.read_raw_log()?;
    let mut filtered_output = String::new();

    for line in raw_log.lines().filter(
        |line| {
            let mut contains_illegal = false;
            let line = line.to_lowercase();
            for item in config.get_blacklist() {
                if line.contains(&item.to_lowercase()) {
                    contains_illegal = true;
                    break;
                }
            }
            !contains_illegal // filter for items that don't contain a blacklisted phrase
        }
    ) {
        let line = format!("{line}\n");
        filtered_output.push_str(&line);
    }

    write_output_to_file(config.get_filtered_log_path(), filtered_output)?;

    Ok(())
}

fn write_output_to_file(path: &String, output: String) -> Result<(), Box<dyn Error>> {
    let mut file_to_write = File::create(path)?;
    file_to_write.write(output.as_bytes())?;
    Ok(())
}
