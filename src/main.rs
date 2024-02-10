use minecraft_log_cleaner_rust;
use std::process;

fn main() {
    if let Err(e) = minecraft_log_cleaner_rust::run() {
        eprintln!("ERROR: {e}");
        process::exit(1);
    }
}

// run the program: cargo run -- test_files/server.log test_files/settings.toml > test_files/cleaned.log
