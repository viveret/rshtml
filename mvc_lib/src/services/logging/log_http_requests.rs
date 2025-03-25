use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;
use crate::options::logging_services_options::ILogHttpRequestsOptions;

use crate::contexts::iresponse_context::IResponseContext;

use crate::services::logger::ilog_http_requests_logger::ILogHttpRequestsLogger;
use crate::services::service_collection::{ IServiceCollection, ServiceCollectionExtensions };

use crate::services::request_middleware_service::{ IRequestMiddlewareService, MiddlewareResult };



// this is the service that handles logging HTTP requests.
pub struct LogHttpRequestsMiddleware {
    // the options for the service.
    options: Option<Rc<dyn ILogHttpRequestsOptions>>,
    // loggers
    loggers: Vec<Rc<dyn ILogHttpRequestsLogger>>,
    // the next middleware service in the pipeline
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>
}

impl LogHttpRequestsMiddleware {
    // creates a new instance of the service.
    // options: the options for the service.
    // loggers: where to send request/response data to.
    // returns: the new instance of the service.
    pub fn new(
        options: Option<Rc<dyn ILogHttpRequestsOptions>>,
        loggers: Vec<Rc<dyn ILogHttpRequestsLogger>>,
    ) -> Self {
        Self { options, loggers, next: RefCell::new(None) }
    }

    // creates a new instance of the service for the service collection.
    // services: the service collection.
    // returns: a vector containing the new instance of the service.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::try_get_single::<dyn ILogHttpRequestsOptions>(services).expect("could not get options"),
            ServiceCollectionExtensions::try_get_multiple::<dyn ILogHttpRequestsLogger>(services).unwrap_or(Vec::<Rc<dyn ILogHttpRequestsLogger>>::new()),
        )) as Rc<dyn IRequestMiddlewareService>)]
    }
}

impl IRequestMiddlewareService for LogHttpRequestsMiddleware {
    fn set_next(&self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(&self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Rc<dyn Error>> {
        if let Some(options) = &self.options {
            if options.get_log_request() {
                for logger in self.loggers.iter() {
                    logger.log_request_info(request_context.get_uuid(), request_context.get_http_version(), request_context.get_method(), request_context.get_path());
                    if options.get_log_request_headers() {
                        logger.log_request_headers(request_context.get_uuid(), request_context.get_path(), request_context.get_headers());
                    }
                    if options.get_log_request_cookies() {
                        logger.log_request_cookies(request_context.get_uuid(), request_context.get_path(), request_context.get_headers());
                    }
                }
            }
        }

        if let Some(next) = self.next.borrow().as_ref() {
            let next_response = next.handle_request(response_context, request_context, services)?;
            
            if let Some(options) = &self.options {
                if options.get_log_response() {
                    for logger in self.loggers.iter() {
                        logger.log_response_info(request_context.get_uuid(), response_context.get_status_code(), request_context.get_path());
                        if options.get_log_response_headers() {
                            logger.log_response_headers(request_context.get_uuid(), request_context.get_path(), &response_context.get_headers());
                        }
                        if options.get_log_response_cookies() {
                            logger.log_response_cookies(request_context.get_uuid(), request_context.get_path(), &response_context.get_headers());
                        }
                    }
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

    fn get_type_info(&self) -> Box<crate::core::type_info::TypeInfo> {
        Box::new(TypeInfo::of::<LogHttpRequestsMiddleware>())
    }
}