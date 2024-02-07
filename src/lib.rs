use std::env;
use std::process;
use std::error::Error;

pub mod config;

pub fn run() -> Result<(), Box<dyn Error>> {
    // check if "help" argument is present
    let help_args: Vec<String> = env::args().filter(|x| x == "help").collect();
    if help_args.len() != 0 {
        display_help();
    }

    let args: Vec<String> = env::args().collect();
    let config = config::Config::build(args)?;

    for line in config.raw_log().lines() {
        println!("{line}");
    }

    Ok(())
}

pub fn write_file(_output: Vec<String>, _output_file: String) {
    todo!();
}

fn display_help() {
    println!("Usage: [program name] -- [raw_log] [config (optional)] > [output_file (optional)]");
    println!("raw_log    : The path to the file you want to filter.");
    println!("config     : The path to the file that contains your own custom configuration for the cleaner.");
    println!("output_file: The path to the file where you want the output to be stored.");
    process::exit(1);
}
