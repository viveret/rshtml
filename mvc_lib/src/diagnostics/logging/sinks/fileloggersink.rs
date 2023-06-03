use std::io::BufRead;

use crate::diagnostics::logging::iloggersink::ILoggerSink;



// this struct implements ILoggerSink and logs to a file.
pub struct FileLoggerSink {
    pub file_path: String,
}

impl ILoggerSink for FileLoggerSink {
    fn log(&self, level: log::Level, message: &str) {
        match level {
            log::Level::Trace => println!("[TRACE] {}", message),
            log::Level::Debug => println!("[DEBUG] {}", message),
            log::Level::Info => println!("[INFO] {}", message),
            log::Level::Warn => println!("[WARN] {}", message),
            log::Level::Error => println!("[ERROR] {}", message),
        }
    }

    fn supports_read(&self) -> bool {
        true
    }

    fn read_logs(&self) -> Vec<String> {
        let file = std::fs::File::open(&self.file_path);
        match file {
            Ok(file) => std::io::BufReader::new(file)
                .lines()
                .map(|line| line.unwrap())
                .collect(),
            Err(_) => vec![],
        }
    }

    fn supports_clear(&self) -> bool {
        true
    }

    fn clear_logs(&self) {
        let file = std::path::Path::new(&self.file_path);
        if file.exists() {
            std::fs::remove_file(&self.file_path).unwrap();
        }
    }
}
