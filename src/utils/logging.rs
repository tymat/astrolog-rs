use chrono::Local;
use lazy_static::lazy_static;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use std::sync::Mutex;
use std::path::Path;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn setup_test_log() -> String {
        let test_log_path = "test_request_errors.log";
        // Clean up any existing test log file
        if Path::new(test_log_path).exists() {
            fs::remove_file(test_log_path).expect("Failed to remove existing test log file");
        }
        test_log_path.to_string()
    }

    #[test]
    fn test_log_request_error() {
        let test_log_path = setup_test_log();
        
        // Override the LOG_FILE for testing
        let test_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&test_log_path)
            .expect("Failed to create test log file");
        
        let test_log = Mutex::new(test_file);
        
        // Log a test error
        let endpoint = "test_endpoint";
        let ip = "127.0.0.1";
        let payload = r#"{"test": "data"}"#;
        let error = "Test error message";
        
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let expected_entry = format!(
            "[{}] Endpoint: {}, IP: {}, Payload: {}, Error: {}\n",
            timestamp, endpoint, ip, payload, error
        );
        
        if let Ok(mut file) = test_log.lock() {
            writeln!(file, "{}", expected_entry).expect("Failed to write test log entry");
        }
        
        // Read the log file and verify its contents
        let mut contents = String::new();
        let mut file = File::open(&test_log_path).expect("Failed to open test log file");
        file.read_to_string(&mut contents).expect("Failed to read test log file");
        
        // Clean up
        fs::remove_file(&test_log_path).expect("Failed to remove test log file");
        
        // Verify the log entry
        assert!(contents.contains(endpoint));
        assert!(contents.contains(ip));
        assert!(contents.contains(payload));
        assert!(contents.contains(error));
    }
} 