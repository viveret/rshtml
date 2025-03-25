use std::rc::Rc;
use std::any::Any;

use http::StatusCode;
use uuid::Uuid;

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
        services.add(ServiceDescriptor::new_from::<dyn ILogHttpRequestsLogger, Self>(Self::new_service, ServiceScope::Singleton));
    }
}

impl ILogHttpRequestsLogger for LogHttpRequestsConsoleLogger {
    fn log_request_info(&self, id: &Uuid, version: http::version::Version, method: &http::method::Method, path: &String) {
        println!("{} Inbound HTTP request: {:?} {} {}", id, version, method, path);
    }

    fn log_request_headers(&self, id: &Uuid, path: &String, headers: &http::HeaderMap) {
        println!("{} Request headers for {}:", id, path);
        for header in headers.iter() {
            if header.0 == "Cookie" || header.0 == "cookie" {
                continue;
            }
            println!("{} \t{}: {}", id, header.0, header.1.to_str().expect("header.1.to_str()"));
        }
    }

    fn log_request_cookies(&self, id: &Uuid, path: &String, headers: &http::HeaderMap) {
        for header in headers.iter() {
            if header.0 == "Cookie" || header.0 == "cookie" {
                println!("{} Request cookies for {}:", id, path);
                let cookies: Vec<&str> = header.1.to_str().expect("header.1.to_str()").split(';').map(|x| x.trim()).collect();
                for cookie in cookies {
                    let split_kvp: Vec<&str> = cookie.split('=').collect();
                    if split_kvp.len() == 2 {
                        println!("{}\t\t{}: {}", id, split_kvp[0], split_kvp[1]);
                    } else {
                        println!("{}\t\t{}", id, cookie);
                    }
                }
                return;
            }
        }
        println!("{} Request cookies missing for {}:", id, path);
    }
    
    fn log_response_info(&self, id: &Uuid, status_code: StatusCode, path: &String) {
        println!("{} Outbound HTTP response for {} -> {}", id, path, status_code);
    }
    
    fn log_response_headers(&self, id: &Uuid, path: &String, headers: &http::HeaderMap) {
        println!("{} Response headers for {}:", id, path);
        for header in headers.iter() {
            if header.0 == "Cookie" || header.0 == "cookie" {
                continue;
            }
            println!("{}\t{}: {}", id, header.0, header.1.to_str().expect("header.1.to_str()"));
        }
    }
    
    fn log_response_cookies(&self, id: &Uuid, path: &String, headers: &http::HeaderMap) {
        for header in headers.iter() {
            if header.0 == "Cookie" || header.0 == "cookie" {
                println!("{} Response cookies for {}:", id, path);
                let cookies: Vec<&str> = header.1.to_str().expect("header.1.to_str()").split(';').map(|x| x.trim()).collect();
                for cookie in cookies {
                    let split_kvp: Vec<&str> = cookie.split('=').collect();
                    if split_kvp.len() == 2 {
                        println!("{}\t\t{}: {}", id, split_kvp[0], split_kvp[1]);
                    } else {
                        println!("{}\t\t{}", id, cookie);
                    }
                }
                return;
            }
        }
        println!("{} Response cookies missing for {}:", id, path);
    }
}