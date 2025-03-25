use super::ilogging_context::ILoggingContext;


pub struct LoggingPrefix {
    pub prefix: String,
}

impl ILoggingContext for LoggingPrefix {
    fn wrap_log(&self, _: log::Level, message: &str) -> String {
        format!("{} {}", self.prefix, message)
    }
}