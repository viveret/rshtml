use std::any::Any;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;

use crate::contexts::connection_context::IHttpConnectionContext;
use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::IResponseContext;
use crate::diagnostics::logging::logging_service::ILoggingService;
use crate::diagnostics::logging::logging_service::LoggingService;
use crate::errors::RequestError;

use crate::contexts::connection_context::IConnectionContext;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;
use crate::services::request_middleware_service::IRequestMiddlewareService;

use crate::options::http_options::IHttpOptions;

use crate::contexts::request_context::RequestContext;
use crate::contexts::response_context::ResponseContext;

// this is a trait for a class that can process an HTTP request and return an HTTP response.
// the way requests are processed is by using a pipeline of middleware services.
pub trait IHttpRequestPipeline {
    fn process_request(self: &Self, connection_context: &dyn IHttpConnectionContext, services: &dyn IServiceCollection) -> Result<(), Box<dyn Error>>;
}

// this is a struct that implements IHttpRequestPipeline.
pub struct HttpRequestPipeline {
    #[allow(dead_code)]
    options: Rc<dyn IHttpOptions>,
    logger_service: Rc<dyn ILoggingService>,
}

impl HttpRequestPipeline {
    pub fn new(
        options: Rc<dyn IHttpOptions>,
        logger_service: Rc<dyn ILoggingService>,
    ) -> Self {
        Self { 
            options: options,
            logger_service: logger_service,
        }
    }

    // creates HTTP request pipeline as a service.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IHttpOptions>(services.clone()),
            LoggingService::get_service(services),
        )) as Rc<dyn IHttpRequestPipeline>)]
    }

    /// Process the request using the middleware.
    /// 
    /// # Arguments
    /// * `request_context` - The request context
    /// * `services` - The service collection
    /// 
    /// # Returns
    /// * The result of processing the request.
    fn process_request_using_middleware(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<(), Box<dyn Error>> {
        // Get the middleware services
        let middleware = ServiceCollectionExtensions::get_required_multiple::<dyn IRequestMiddlewareService>(services);
        // Throw an error if there are no middleware services
        if middleware.len() == 0 {
            return Err(Box::new(RequestError(format!("No middleware configured"))));
        }

        self.for_each_set_next(&middleware);

        // Get the first middleware service
        let first = middleware.first().unwrap();
        // Handle the request
        first.handle_request(response_context, request_context, services)?;
        
        Ok(())
    }

    // Set the next middleware service for each middleware service.
    // This creates a linked list of middleware services that can be used to process a request.
    // middleware: the middleware services.
    fn for_each_set_next(self: &Self, middleware: &Vec<Rc<dyn IRequestMiddlewareService>>) {
        // Create an iterator over the middleware
        let mut it = middleware.iter().cloned().peekable();
        // Loop through each middleware service
        loop {
            // Get the next middleware service
            let request_handler_option = it.next();
            // Check if there is a next middleware service
            if let Some(request_handler) = request_handler_option {
                // Get the next middleware service
                let next_request_handler = it.peek();
                // Set the next middleware service
                request_handler.set_next(next_request_handler.cloned());
            } else {
                // There are no more middleware services
                break;
            }
        }
    }

}

impl IHttpRequestPipeline for HttpRequestPipeline {
    fn process_request<'a>(self: &Self, connection_context: &dyn IHttpConnectionContext, services: &dyn IServiceCollection) -> Result<(), Box<dyn Error>> {
        let request_context = RequestContext::parse(connection_context);
        let response_context = ResponseContext::new(&request_context);

        self.process_request_using_middleware(&response_context, &request_context, services)?;
        return Ok(());
    }
}