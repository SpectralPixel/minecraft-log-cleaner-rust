use minecraft_log_cleaner_rust;
use std::process;

fn main() {
    println!("----- MINECRAFT LOG CLEANER -----");
    if let Err(e) = minecraft_log_cleaner_rust::run() {
        let exit_code = 1;
        eprintln!("\n----- ERROR ENCOUNTERED! -----");
        eprintln!("{e}");
        println!("exiting process with code {exit_code}...");
        process::exit(exit_code);
    }
}
