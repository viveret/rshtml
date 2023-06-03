use crate::diagnostics::logging::iloggersink::ILoggerSink;



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