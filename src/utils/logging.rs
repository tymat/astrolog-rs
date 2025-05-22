use chrono::Local;
use lazy_static::lazy_static;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Mutex;

lazy_static! {
    static ref LOG_FILE: Mutex<File> = Mutex::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open("request_errors.log")
            .expect("Failed to open log file")
    );
}

pub fn log_request_error(endpoint: &str, ip: &str, payload: &str, error: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_entry = format!(
        "[{}] Endpoint: {}, IP: {}, Payload: {}, Error: {}\n",
        timestamp, endpoint, ip, payload, error
    );

    if let Ok(mut file) = LOG_FILE.lock() {
        if let Err(e) = writeln!(file, "{}", log_entry) {
            eprintln!("Failed to write to log file: {}", e);
        }
    }
} 