use std::collections::HashMap;
use conv::ConvUtil;
use crate::config::Config;

pub fn get_line_counts(config: &Config, raw_log: &String) -> HashMap<String, f64> {
    let mut line_counts: HashMap<String, f64> = HashMap::new();
    let trimmed_log = trim_line_starts(config.get_content_start_pos(), raw_log);

    let raw_log_line_count: f64 = trimmed_log.lines().count().value_as().unwrap();
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
