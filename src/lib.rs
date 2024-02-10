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
        println!("Engaging auto-blacklisting...");
        let mut line_counts = formatter::get_line_counts(&config, &raw_log);
        let mut auto_blacklisted_lines: Vec<String> = Vec::new();
        for line in line_counts.drain().filter(|x| x.1 > config.get_auto_blacklist_percentage()) {
            auto_blacklisted_lines.push(line.0.clone());

            // let key = line.0;
            // let val = (line.1 * 10000.).floor() / 10000.;
            // println!("{val}% - {key}")
        }
        config.add_to_blacklist(&mut auto_blacklisted_lines);
        println!("Auto-blacklisting complete!")
    }

    let filtered_log = formatter::filter_log(&config, &raw_log);

    write_output_to_file(&config, &"filtered.log", filtered_log)?;

    Ok(())
}

fn write_output_to_file(config: &Config, file_name: &str, output: String) -> Result<(), Box<dyn Error>> {
    let output_dir = config.output_directory();
    let path = format!("{output_dir}/{file_name}");

    println!("Writing file [{path}]");

    let mut file_to_write = File::create(&path)?;
    file_to_write.write(output.as_bytes())?;

    println!("Successfully wrote to [{path}]!");

    Ok(())
}
