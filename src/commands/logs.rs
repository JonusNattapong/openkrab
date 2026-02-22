//! Logs command - View and tail gateway logs

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn get_log_path() -> PathBuf {
    let home = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join("krabkrab").join("logs")
}

pub fn logs_tail_command(lines: Option<usize>, follow: bool, json: bool) -> String {
    let log_dir = get_log_path();
    let log_file = log_dir.join("gateway.log");

    if !log_file.exists() {
        return format!(
            "Log file not found: {}\nRun 'krabkrab gateway start' to create logs.",
            log_file.display()
        );
    }

    let n = lines.unwrap_or(100);

    let format_line = |l: &str| -> String {
        if json {
            l.to_string()
        } else {
            if l.contains("ERROR") || l.contains("error") {
                format!("\x1b[31m{}\x1b[0m", l)
            } else if l.contains("WARN") || l.contains("warn") {
                format!("\x1b[33m{}\x1b[0m", l)
            } else if l.contains("INFO") || l.contains("info") {
                l.to_string()
            } else if l.contains("DEBUG") || l.contains("debug") {
                format!("\x1b[90m{}\x1b[0m", l)
            } else {
                l.to_string()
            }
        }
    };

    if follow {
        use std::io::Seek;
        let mut file = match File::open(&log_file) {
            Ok(f) => f,
            Err(e) => return format!("Failed to open log file: {}", e),
        };
        let mut pos = 0;

        // Print requested tail lines first
        if let Ok(f_clone) = File::open(&log_file) {
            let reader = BufReader::new(f_clone);
            let all_lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
            let start = all_lines.len().saturating_sub(n);
            for line in &all_lines[start..] {
                println!("{}", format_line(line));
            }
        }

        // Now move to end
        if let Ok(end) = file.seek(std::io::SeekFrom::End(0)) {
            pos = end;
        }

        let mut buf = String::new();
        loop {
            if let Ok(mut reader) = File::open(&log_file) {
                if reader.seek(std::io::SeekFrom::Start(pos)).is_ok() {
                    let mut br = BufReader::new(reader);
                    while br.read_line(&mut buf).unwrap_or(0) > 0 {
                        if buf.ends_with('\n') {
                            print!("{}", format_line(buf.trim_end()));
                            println!();
                            buf.clear();
                        }
                    }
                    if let Ok(new_pos) = br.stream_position() {
                        pos = new_pos;
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    } else {
        match File::open(&log_file) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let all_lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
                let start = all_lines.len().saturating_sub(n);
                let output: Vec<String> =
                    all_lines[start..].iter().map(|l| format_line(l)).collect();

                output.join("\n")
            }
            Err(e) => format!("Failed to open log file: {}", e),
        }
    }
}
