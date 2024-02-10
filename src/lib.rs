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

    if config.get_auto_blacklist() {
        todo!(); // MAKE SURE TO FILTER OUT ACTIVITY THAT IS 100% KNOWN TO BE PLAYER ACTIVITY BEFORE SORTING LINES. ONLY SYSTEM MESSAGES SHOULD BE SORTED OUT
        let mut line_counts = formatter::get_line_counts(&config, &raw_log);
        let mut auto_blacklisted_lines: Vec<String> = Vec::new();
        for line in line_counts.drain().filter(|x| x.1 > config.get_auto_blacklist_percentage()) {
            auto_blacklisted_lines.push(line.0.clone());

            // let key = line.0;
            // let val = (line.1 * 10000.).floor() / 10000.;
            // println!("{val}% - {key}")
        }
        config.add_to_blacklist(&mut auto_blacklisted_lines);
    }
    let filtered_log = filter_log(&config, &raw_log);

    write_output_to_file(&config, &"filtered.log", filtered_log)?;

    Ok(())
}

fn filter_log(config: &Config, raw_log: &String) -> String {
    let mut filtered_log = String::new();

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
        filtered_log.push_str(&line);
    }

    filtered_log
}

fn write_output_to_file(config: &Config, file_name: &str, output: String) -> Result<(), Box<dyn Error>> {
    let output_dir = config.output_directory();
    let path = format!("{output_dir}/{file_name}");
    let mut file_to_write = File::create(path)?;
    file_to_write.write(output.as_bytes())?;
    Ok(())
}
