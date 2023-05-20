use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use http::HeaderMap;

use crate::contexts::irequest_context::IRequestContext;
use crate::options::logging_services_options::ILogHttpRequestsOptions;

use crate::contexts::response_context::ResponseContext;

use crate::services::service_collection::{ IServiceCollection, ServiceCollectionExtensions };

use crate::services::request_middleware_service::{ IRequestMiddlewareService, MiddlewareResult };

// this is the service that handles logging HTTP requests.
pub struct LogHttpRequestsMiddleware {
    // the options for the service.
    options: Option<Rc<dyn ILogHttpRequestsOptions>>,
    // the next middleware service in the pipeline
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>
}

impl LogHttpRequestsMiddleware {
    // creates a new instance of the service.
    // options: the options for the service.
    // returns: the new instance of the service.
    pub fn new(options: Option<Rc<dyn ILogHttpRequestsOptions>>) -> Self {
        Self { options: options, next: RefCell::new(None) }
    }

    // creates a new instance of the service for the service collection.
    // services: the service collection.
    // returns: a vector containing the new instance of the service.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::try_get_single::<dyn ILogHttpRequestsOptions>(services).expect("could not get options"),
        )) as Rc<dyn IRequestMiddlewareService>)]
    }

    // prints the HTTP headers to the console.
    // headers: the headers to print.
    // log_cookies: whether or not to log cookies.
    // returns: nothing.
    pub fn print_headers(self: &Self, headers: &HeaderMap, log_cookies: bool) {
        for header in headers.iter() {
            if header.0 == "Cookie" || header.0 == "cookie" {
                if log_cookies {
                    println!("\t{}:", header.0);
                    let cookies: Vec<&str> = header.1.to_str().unwrap().split(';').map(|x| x.trim()).collect();
                    for cookie in cookies {
                        let split_kvp: Vec<&str> = cookie.split('=').collect();
                        if split_kvp.len() == 2 {
                            println!("\t\t{}: {}", split_kvp[0], split_kvp[1]);
                        } else {
                            println!("\t\t{}", cookie);
                        }
                    }
                }

                continue;
            }
            println!("\t{}: {}", header.0, header.1.to_str().unwrap());
        }
    }
}

impl IRequestMiddlewareService for LogHttpRequestsMiddleware {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(self: &Self, request_ctx: Rc<dyn IRequestContext>, response_ctx: Rc<ResponseContext>, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn Error>> {
        if let Some(options) = &self.options {
            if options.get_log_request() {
                println!("Inbound HTTP request: {:?} {} {}", request_ctx.get_http_version(), request_ctx.get_method(), request_ctx.get_path());
            }

            if options.get_log_request_headers() {
                println!("Request headers for {}:", request_ctx.get_path());
                self.print_headers(request_ctx.get_headers(), options.get_log_request_cookies());
            }
        }

        if let Some(next) = self.next.borrow().as_ref() {
            let next_response = next.handle_request(request_ctx.clone(), response_ctx.clone(), services)?;
            
            if let Some(options) = &self.options {
                if options.get_log_response() {
                    println!("Outbound HTTP response for {} -> {}", request_ctx.get_path(), response_ctx.status_code.borrow());
                }

                if options.get_log_response_headers() {
                    println!("Response headers for {}:", request_ctx.get_path());
                    self.print_headers(&response_ctx.headers.borrow(), options.get_log_response_cookies());
                }
            }

            match next_response {
                MiddlewareResult::OkBreak => {
                    return Ok(MiddlewareResult::OkBreak); // short circuit middleware
                },
                _ => { }
            }
        }
        Ok(MiddlewareResult::OkContinue)
    }
}