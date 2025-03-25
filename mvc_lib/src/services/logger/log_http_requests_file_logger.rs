use std::rc::Rc;
use std::any::Any;

use uuid::Uuid;

use crate::core::type_info::TypeInfo;
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollection};

use super::ilog_http_requests_logger::ILogHttpRequestsLogger;


pub struct LogHttpRequestsFileLogger {

}

impl LogHttpRequestsFileLogger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
        )) as Rc<dyn ILogHttpRequestsLogger>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn ILogHttpRequestsLogger>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl ILogHttpRequestsLogger for LogHttpRequestsFileLogger {
    fn log_request_info(&self, id: &Uuid, version: http::version::Version, method: &http::method::Method, path: &String) {
        // todo!()
    }

    fn log_request_headers(&self, id: &Uuid, path: &String, headers: &http::HeaderMap) {
        // todo!()
    }

    fn log_request_cookies(&self, id: &Uuid, path: &String, headers: &http::HeaderMap) {
        // todo!()
    }
    
    fn log_response_info(&self, id: &Uuid, status_code: http::status::StatusCode, path: &String) {
        // todo!()
    }
    
    fn log_response_headers(&self, id: &Uuid, path: &String, headers: &http::HeaderMap) {
        // todo!()
    }
    
    fn log_response_cookies(&self, id: &Uuid, path: &String, headers: &http::HeaderMap) {
        // todo!()
    }
}