use std::collections::HashMap;
use crate::config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Formatting {
    date_range: [usize; 2],
    time_range: [usize; 2],
    content_start_char_pos: usize,
}

impl Formatting {
    pub fn get_date_range(&self) -> (usize, usize) {
        (self.date_range[0], self.date_range[1])
    }

    pub fn get_time_range(&self) -> (usize, usize) {
        (self.time_range[0], self.time_range[1])
    }

    pub fn get_content_start_pos(&self) -> usize {
        self.content_start_char_pos
    }
}

pub fn get_line_counts(config: &Config, raw_log: &String) -> HashMap<String, f64> {
    let mut line_counts: HashMap<String, f64> = HashMap::new();
    let trimmed_log = trim_line_starts(config.formatting().get_content_start_pos(), raw_log);

    let raw_log_line_count = trimmed_log.lines().count() as f64;
    let inverse_total_line_count: f64 = 1f64 / raw_log_line_count;
    let single_line_percentage = inverse_total_line_count * 100.;

    for line in trimmed_log.lines() {
        let line = String::from(line);
        match line_counts.get_mut(&line) {
            Some(count) => {
                *count += single_line_percentage;
            },
            None => {
                line_counts.insert(line, single_line_percentage);
            },
        }
    }

    line_counts
}

pub fn filter_log(config: &Config, raw_log: &String) -> String {
    println!("Filtering blacklisted lines out of log...");

    let mut filtered_log = String::new();

    for line in raw_log.lines().filter(
        |line| {
            let mut contains_illegal = false;
            let line = line.to_lowercase();
            for item in config.blacklist().get_blacklist_iter() {
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

    println!("Log filtered!");

    filtered_log
}

fn trim_line_starts(content_start: usize, text: &String) -> String {
    let trimmed_text = text.lines().flat_map(
        |line| {
            line[content_start..line.len()].chars().chain("\n".chars())
        }
    ).collect();

    trimmed_text
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_garbage() {
        let initial_string = String::from("2023-11-20 02:00:20 [INFO] Saving chunks");
        let content_start_char_pos: usize = 20;
        let final_string = trim_line_starts(content_start_char_pos, &initial_string);
        let expected_string = String::from("[INFO] Saving chunks\n");

        assert_eq!(final_string, expected_string);
    }

    #[test]
    fn remove_garbage_multiline() {
        let initial_string = String::from(
"2023-11-20 02:00:20 [INFO] CONSOLE: Forcing save..
2023-11-20 02:00:20 [INFO] CONSOLE: Save complete.
2023-11-20 02:00:20 [INFO] CONSOLE: Stopping the server..
2023-11-20 02:00:20 [INFO] Stopping server
2023-11-20 02:00:20 [INFO] Saving chunks"
        );
        let content_start_char_pos: usize = 20;
        let final_string = trim_line_starts(content_start_char_pos, &initial_string);
        let expected_string = String::from(
"[INFO] CONSOLE: Forcing save..
[INFO] CONSOLE: Save complete.
[INFO] CONSOLE: Stopping the server..
[INFO] Stopping server
[INFO] Saving chunks\n"
        );

        assert_eq!(final_string, expected_string);
    }
}
