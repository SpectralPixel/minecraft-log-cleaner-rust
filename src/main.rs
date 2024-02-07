use minecraft_log_cleaner_rust;
use std::process;

fn main() {
    if let Err(e) = minecraft_log_cleaner_rust::run() {
        eprintln!("ERROR: {e}");
        process::exit(1);
    }
}

// run the program: cargo run -- test_files/raw.txt test_files/filters.json > test_files/output.txt
