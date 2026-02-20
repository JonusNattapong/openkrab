//! Logs command - View and tail gateway logs

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn get_log_path() -> PathBuf {
    let home = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join("krabkrab").join("logs")
}

pub fn logs_tail_command(lines: Option<usize>, _follow: bool, json: bool) -> String {
    let log_dir = get_log_path();
    let log_file = log_dir.join("gateway.log");

    if !log_file.exists() {
        return format!(
            "Log file not found: {}\nRun 'krabkrab gateway start' to create logs.",
            log_file.display()
        );
    }

    let n = lines.unwrap_or(100);

    match File::open(&log_file) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let all_lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
            let start = if all_lines.len() > n {
                all_lines.len() - n
            } else {
                0
            };

            let output: Vec<String> = all_lines[start..]
                .iter()
                .map(|l| {
                    if json {
                        l.clone()
                    } else {
                        // Simple colored output
                        if l.contains("ERROR") || l.contains("error") {
                            format!("\x1b[31m{}\x1b[0m", l)
                        } else if l.contains("WARN") || l.contains("warn") {
                            format!("\x1b[33m{}\x1b[0m", l)
                        } else if l.contains("INFO") || l.contains("info") {
                            l.clone()
                        } else if l.contains("DEBUG") || l.contains("debug") {
                            format!("\x1b[90m{}\x1b[0m", l)
                        } else {
                            l.clone()
                        }
                    }
                })
                .collect();

            output.join("\n")
        }
        Err(e) => format!("Failed to open log file: {}", e),
    }
}

pub fn logs_follow_command() -> String {
    "Use 'krabkrab logs --follow' to tail logs (not yet implemented)".to_string()
}
