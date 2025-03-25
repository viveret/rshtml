use super::iloggersink::ILoggerSink;



// ILogger is a trait that defines a logger. It is used by the logging service to log messages.
pub trait ILogger: ILoggerSink {
    fn log_trace(&self, message: &str);
    fn log_debug(&self, message: &str);
    fn log_info(&self, message: &str);
    fn log_warn(&self, message: &str);
    fn log_error(&self, message: &str);
}