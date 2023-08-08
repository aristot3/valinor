use log::{Record, Metadata, Log};
use std::fs::{OpenOptions, File};
use std::sync::Mutex;
use std::io::Write;

pub struct JsonLogger {
    file: Mutex<File>,
    file_name: Option<String>,
}

impl JsonLogger {
    pub fn new(log_file_path: &str, file_name: Option<String>) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(log_file_path)
            .expect("Unable to open or create log file");

        JsonLogger {
            file: Mutex::new(file),
            file_name,
        }
    }
}

impl Log for JsonLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!(
                "{{\"timestamp\": \"{}\", \"level\": \"{}\", \"message\": \"{}\", \"file.name\": \"{}\"}}\n",
                chrono::Local::now().to_rfc3339(),
                record.level(),
                record.args(),
                self.file_name.as_ref().unwrap_or(&"".to_string())
            );

            let mut file = self.file.lock().unwrap();
            file.write_all(msg.as_bytes()).unwrap();
        }
    }

    fn flush(&self) {}
}
