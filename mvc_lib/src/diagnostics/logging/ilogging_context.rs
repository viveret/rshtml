

pub trait ILoggingContext {
    fn wrap_log(&self, level: log::Level, message: &str) -> String;
}