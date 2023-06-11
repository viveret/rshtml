use std::cell::RefCell;

use uuid::Timestamp;

use super::iperformance_logger::{PerformanceMetricValue, IPerformanceLogger, PerformanceMetricType};



pub struct InMemoryLogger {
    pub metrics: RefCell<Vec<PerformanceMetricValue>>,
}

impl InMemoryLogger {
    pub fn new() -> Self {
        Self {
            metrics: RefCell::new(Vec::new()),
        }
    }
}

impl IPerformanceLogger for InMemoryLogger {
    fn log(&self, metric_value: PerformanceMetricValue) {
        self.metrics.borrow_mut().push(metric_value);
    }

    fn get_metrics(&self, request_id: Option<uuid::Uuid>, _name: Option<&str>, metric_type: Option<super::iperformance_logger::PerformanceMetricType>, since: Option<uuid::Timestamp>) -> Vec<PerformanceMetricValue> {
        let src = self.metrics.borrow();
        let mut iter: Box<dyn Iterator<Item = &PerformanceMetricValue>> = Box::new(src.iter().filter(|_x| true));

        if let Some(_) = request_id {
            iter = Box::new(iter.filter(|x| x.request_id == request_id));
        }

        // if let Some(_) = name {
        //     iter = Box::new(iter.filter(|x| x.metric_name.as_ref() == name.unwrap_or_default()));
        // }

        if let Some(_) = metric_type {
            iter = Box::new(iter.filter(|x| &x.metric_type == metric_type.as_ref().unwrap_or(&PerformanceMetricType::Custom)));
        }

        if let Some(_) = since {
            iter = Box::new(iter.filter(|x| x.timestamp.to_unix().0 > since.unwrap_or(Timestamp::from_rfc4122(0, 0)).to_unix().0));
        }

        iter.map(|x| x.clone()).collect()
    }
}