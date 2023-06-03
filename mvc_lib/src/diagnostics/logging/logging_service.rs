use std::any::Any;
use std::error::Error;
use std::rc::Rc;

use crate::options::logging_services_options::ILogHttpRequestsOptions;

use crate::services::service_collection::{ IServiceCollection, ServiceCollectionExtensions };

use super::ilogger::ILogger;
use super::iloggersink::ILoggerSink;

pub trait ILoggingService: ILogger {
    fn get_logger(&self) -> Rc<dyn ILogger>;
}

pub struct LoggingService {
    // the options for the service.
    options: Option<Rc<dyn ILogHttpRequestsOptions>>,
    logger: Rc<dyn ILogger>,
}

impl LoggingService {
    pub fn new(
        options: Option<Rc<dyn ILogHttpRequestsOptions>>
    ) -> Self {
        Self { 
            options: options,
            logger: Rc::new(super::logger::Logger::new(None)),
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::try_get_single::<dyn ILogHttpRequestsOptions>(services).expect("could not get options"),
        )) as Rc<dyn ILoggingService>)]
    }

    pub fn get_service(services: &dyn IServiceCollection) -> Rc<dyn ILoggingService> {
        ServiceCollectionExtensions::get_required_single::<dyn ILoggingService>(services)
    }

    pub fn try_get_service<'a>(services: &'a (dyn IServiceCollection + 'a)) -> Result<Option<Rc<dyn ILoggingService + 'a>>, Box<dyn Error>> {
        ServiceCollectionExtensions::try_get_single::<dyn ILoggingService>(services)
    }
}

impl ILoggingService for LoggingService {
    fn get_logger(&self) -> Rc<dyn ILogger> {
        self.logger.clone()
    }
}

impl ILogger for LoggingService {
    fn log_trace(&self, message: &str) {
        self.logger.log_trace(message);
    }

    fn log_debug(&self, message: &str) {
        self.logger.log_debug(message);
    }

    fn log_info(&self, message: &str) {
        self.logger.log_info(message);
    }

    fn log_warn(&self, message: &str) {
        self.logger.log_warn(message);
    }

    fn log_error(&self, message: &str) {
        self.logger.log_error(message);
    }
}

impl ILoggerSink for LoggingService {
    fn log(self: &Self, level: log::Level, message: &str) {
        self.logger.log(level, message);
    }

    fn supports_read(&self) -> bool {
        self.logger.supports_read()
    }

    fn read_logs(&self) -> Vec<String> {
        self.logger.read_logs()
    }

    fn supports_clear(&self) -> bool {
        self.logger.supports_clear()
    }

    fn clear_logs(&self) {
        self.logger.clear_logs();
    }
}