// Logger utilities

use std::fs::OpenOptions;
use std::io::Write;

pub fn log_to_file(message: &str) {
    let log_path = "/tmp/ratifact-by-neura-ai.log";
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_path) {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let _ = writeln!(file, "[{}] {}", timestamp, message);
    }
}
