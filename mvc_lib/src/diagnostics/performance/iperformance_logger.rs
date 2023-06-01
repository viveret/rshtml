use std::{borrow::Cow, time::Duration};

use uuid::{Timestamp, Uuid};



#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub enum PerformanceMetricType {
    Time,
    Memory,
    Cpu,
    Disk,
    Network,
    Request,
    Response,
    File,
    Database,
    Custom,
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub enum PerformanceMetricValueType {
    F32(f32),
    U32(u32),
    I32(i32),
    F64(f64),
    U64(u64),
    I64(i64),
    Duration(std::time::Duration),
}

#[derive(Clone)]
#[derive(Copy)]
pub struct PerformanceMetricValue {
    pub timestamp: Timestamp,
    pub request_id: Option<Uuid>,
    // pub metric_name: Cow<'a, str>,
    pub metric_type: PerformanceMetricType,
    pub metric_value: PerformanceMetricValueType,
}

impl PerformanceMetricValue {
    pub fn new() -> PerformanceMetricValue {
        Self {
            timestamp: Timestamp::from_rfc4122(0, 0),
            request_id: None,
            // metric_name: ,
            metric_type: PerformanceMetricType::Custom,
            metric_value: PerformanceMetricValueType::Duration(Duration::from_secs(0)),
        }
    }
}

pub trait IPerformanceLogger {
    fn log(&self, metric_value: PerformanceMetricValue);

    fn get_metrics(&self, request_id: Option<Uuid>, name: Option<&str>, metric_type: Option<PerformanceMetricType>, since: Option<Timestamp>) -> Vec<PerformanceMetricValue>;
}