use std::rc::Rc;
use std::any::Any;

use crate::core::type_info::TypeInfo;
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollection};

use super::ilog_http_requests_logger::ILogHttpRequestsLogger;


pub struct LogHttpRequestsConsoleLogger {

}

impl LogHttpRequestsConsoleLogger {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
        )) as Rc<dyn ILogHttpRequestsLogger>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn ILogHttpRequestsLogger>(), Self::new_service, ServiceScope::Singleton));
        // services.add(ServiceDescriptor::new_from::<dyn ILogHttpRequestsLogger, Self>(Self::new_service, ServiceScope::Singleton));
    }
}

impl ILogHttpRequestsLogger for LogHttpRequestsConsoleLogger {
    fn log_request_info(&self, version: http::version::Version, method: &http::method::Method, path: &String) {
        println!("Inbound HTTP request: {:?} {} {}", version, method, path);
    }

    fn log_request_headers(&self, path: &String, headers: &http::HeaderMap) {
        println!("Request headers for {}:", path);
        for header in headers.iter() {
            if header.0 == "Cookie" || header.0 == "cookie" {
                continue;
            }
            println!("\t{}: {}", header.0, header.1.to_str().expect("header.1.to_str()"));
        }
    }

    fn log_request_cookies(&self, path: &String, headers: &http::HeaderMap) {
        for header in headers.iter() {
            if header.0 == "Cookie" || header.0 == "cookie" {
                println!("Request cookies for {}:", path);
                let cookies: Vec<&str> = header.1.to_str().expect("header.1.to_str()").split(';').map(|x| x.trim()).collect();
                for cookie in cookies {
                    let split_kvp: Vec<&str> = cookie.split('=').collect();
                    if split_kvp.len() == 2 {
                        println!("\t\t{}: {}", split_kvp[0], split_kvp[1]);
                    } else {
                        println!("\t\t{}", cookie);
                    }
                }
                return;
            }
        }
        println!("Request cookies missing for {}:", path);
    }
}