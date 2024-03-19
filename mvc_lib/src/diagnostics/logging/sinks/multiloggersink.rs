use std::rc::Rc;

use crate::diagnostics::logging::iloggersink::ILoggerSink;


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
        }).nth(0).expect("no loggers").read_logs()
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