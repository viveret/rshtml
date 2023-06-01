use std::any::Any;
use std::rc::Rc;

use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions};

use super::iperformance_logger_service::IPerformanceLoggerService;
use super::iperformance_logger::IPerformanceLogger;


// this struct implements IPerformanceLoggerService.
pub struct PerformanceLoggerService {
    pub logger: Rc<dyn IPerformanceLogger>,

}

impl PerformanceLoggerService {
    pub fn new(logger: Rc<dyn IPerformanceLogger>) -> Self {
        Self {
            logger: logger,
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        let logger = ServiceCollectionExtensions::get_required_single::<dyn IPerformanceLogger>(services);
        vec![Box::new(Rc::new(Self::new(logger)) as Rc<dyn IPerformanceLoggerService>)]
    }
}

impl IPerformanceLoggerService for PerformanceLoggerService {
    fn get_logger(&self) -> Rc<dyn IPerformanceLogger> {
        self.logger.clone()
    }
}