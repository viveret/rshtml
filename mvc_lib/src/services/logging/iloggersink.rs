

// ILogger is a trait that defines a logger sink, which is a destination for log messages.
pub trait ILoggerSink {
    fn log(&self, level: log::Level, message: &str);

    fn supports_read(&self) -> bool;
    fn read_logs(&self) -> Vec<String>;

    fn supports_clear(&self) -> bool;
    fn clear_logs(&self);
}