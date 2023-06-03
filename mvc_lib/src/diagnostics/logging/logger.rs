use std::cell::RefCell;
use std::rc::Rc;

use super::ilogger::ILogger;
use super::ilogging_context::ILoggingContext;
use super::sinks::consoleloggersink::ConsoleLoggerSink;
use super::sinks::fileloggersink::FileLoggerSink;
use super::sinks::multiloggersink::MultiLoggerSink;
use super::iloggersink::ILoggerSink;


struct TimestampLoggingContext {}

impl TimestampLoggingContext {
    pub fn new() -> Self {
        Self {}
    }
}

impl ILoggingContext for TimestampLoggingContext {
    fn wrap_log(&self, _: log::Level, message: &str) -> String {
        format!("{} {}", chrono::Local::now().to_rfc3339(), message)
    }
}

// this struct implements ILogger.
pub struct Logger {
    pub loggers: MultiLoggerSink,
    pub contexts: RefCell<Vec<Rc<dyn ILoggingContext>>>,
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
            contexts: RefCell::new(vec![
                Rc::new(TimestampLoggingContext::new())
            ]),
        }
    }

    pub fn send_through_contexts(&self, level: log::Level, message: &str) -> String {
        let mut message = String::from(message);
        for context in self.contexts.borrow().iter() {
            message = context.wrap_log(level, &message);
        }
        message
    }
}

impl ILogger for Logger {
    fn log_trace(&self, message: &str) {
        self.log(log::Level::Trace, message);
    }

    fn log_debug(&self, message: &str) {
        self.log(log::Level::Debug, message);
    }

    fn log_info(&self, message: &str) {
        self.log(log::Level::Info, message);
    }

    fn log_warn(&self, message: &str) {
        self.log(log::Level::Warn, message);
    }

    fn log_error(&self, message: &str) {
        self.log(log::Level::Error, message);
    }
}

impl ILoggerSink for Logger {
    fn log(&self, level: log::Level, message: &str) {
        self.loggers.log(level, self.send_through_contexts(level, message).as_str());
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