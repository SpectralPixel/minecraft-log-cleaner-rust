use minecraft_log_cleaner_rust;
use std::process;

fn main() {
    if let Err(e) = minecraft_log_cleaner_rust::run() {
        eprintln!("ERROR: {e}");
        process::exit(1);
    }
}
