use std::rc::Rc;

use super::iperformance_logger::IPerformanceLogger;


// this trait represents a service that provides a performance logger.
pub trait IPerformanceLoggerService {
    fn get_logger(&self) -> Rc<dyn IPerformanceLogger>;
}