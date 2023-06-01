use std::{rc::Rc, io::BufRead};



// ILogger is a trait that defines a logger sink, which is a destination for log messages.
pub trait ILoggerSink {
    fn log(&self, level: log::Level, message: &str);

    fn supports_read(&self) -> bool;
    fn read_logs(&self) -> Vec<String>;

    fn supports_clear(&self) -> bool;
    fn clear_logs(&self);
}

// this struct implements ILoggerSink and logs to multiple sinks.
pub struct MultiLoggerSink {
    pub loggers: Vec<Rc<dyn ILoggerSink>>,
}

impl ILoggerSink for MultiLoggerSink {
    fn log(&self, level: log::Level, message: &str) {
        for logger in self.loggers.iter() {
            logger.log(level, message);
        }
    }

    fn supports_read(&self) -> bool {
        self.loggers.iter().any(|logger| logger.supports_read())
    }

    fn read_logs(&self) -> Vec<String> {
        self.loggers.iter().filter(|logger| {
            logger.supports_read()
        }).nth(0).unwrap().read_logs()
    }

    fn supports_clear(&self) -> bool {
        self.loggers.iter().any(|logger| logger.supports_clear())
    }

    fn clear_logs(&self) {
        for logger in self.loggers.iter() {
            if logger.supports_clear() {
                logger.clear_logs();
            }
        }
    }
}

// this struct implements ILoggerSink and logs to the console.
pub struct ConsoleLoggerSink {

}

impl ILoggerSink for ConsoleLoggerSink {
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
        false
    }

    fn read_logs(&self) -> Vec<String> {
        vec![]
    }

    fn supports_clear(&self) -> bool {
        false
    }

    fn clear_logs(&self) { }
}

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



// ILogger is a trait that defines a logger. It is used by the logging service to log messages.
pub trait ILogger: ILoggerSink {
    fn log_trace(&self, message: &str);
    fn log_debug(&self, message: &str);
    fn log_info(&self, message: &str);
    fn log_warn(&self, message: &str);
    fn log_error(&self, message: &str);
}

// this struct implements ILogger.
pub struct Logger {
    pub loggers: MultiLoggerSink,
}

impl Logger {
    pub fn new(sinks: Option<Vec<Rc<dyn ILoggerSink>>>) -> Self {
        Self {
            loggers: MultiLoggerSink {
                loggers: sinks.unwrap_or(vec![
                    Rc::new(ConsoleLoggerSink {}),
                    Rc::new(FileLoggerSink {
                        file_path: String::from("log.txt"),
                    }),
                ]),
            },
        }
    }
}

impl ILogger for Logger {
    fn log_trace(&self, message: &str) {
        self.loggers.log(log::Level::Trace, message);
    }

    fn log_debug(&self, message: &str) {
        self.loggers.log(log::Level::Debug, message);
    }

    fn log_info(&self, message: &str) {
        self.loggers.log(log::Level::Info, message);
    }

    fn log_warn(&self, message: &str) {
        self.loggers.log(log::Level::Warn, message);
    }

    fn log_error(&self, message: &str) {
        self.loggers.log(log::Level::Error, message);
    }
}

impl ILoggerSink for Logger {
    fn log(&self, level: log::Level, message: &str) {
        self.loggers.log(level, message);
    }

    fn supports_read(&self) -> bool {
        self.loggers.supports_read()
    }

    fn read_logs(&self) -> Vec<String> {
        self.loggers.read_logs()
    }

    fn supports_clear(&self) -> bool {
        self.loggers.supports_clear()
    }

    fn clear_logs(&self) {
        self.loggers.clear_logs();
    }
}