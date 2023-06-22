use std::any::Any;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;

use crate::app::ihttp_request_pipeline::IHttpRequestPipeline;
use crate::contexts::ihttpconnection_context::IHttpConnectionContext;
use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::iresponse_context::IResponseContext;
use crate::diagnostics::logging::logging_service::ILoggingService;
use crate::diagnostics::logging::logging_service::LoggingService;
use crate::error::error_handler_service::IErrorHandlerService;
use crate::errors::RequestError;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;
use crate::services::request_middleware_service::IRequestMiddlewareService;

use crate::options::http_options::IHttpOptions;

use crate::contexts::request_context::RequestContext;
use crate::contexts::response_context::ResponseContext;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;

// this is a struct that implements IHttpRequestPipeline.
pub struct HttpRequestPipeline {
    #[allow(dead_code)]
    options: Rc<dyn IHttpOptions>,
    _logger_service: Rc<dyn ILoggingService>,
    // times_called: RefCell<i32>,
    error_handler_service: Rc<dyn IErrorHandlerService>,
}

impl HttpRequestPipeline {
    pub fn new(
        options: Rc<dyn IHttpOptions>,
        logger_service: Rc<dyn ILoggingService>,
        // http_error_handlers: Vec<Rc<dyn IHttpErrorHandler>>,
        error_handler_service: Rc<dyn IErrorHandlerService>,
    ) -> Self {
        Self { 
            options: options,
            _logger_service: logger_service,
            // times_called: RefCell::new(0),
            // http_error_handlers: http_error_handlers,
            error_handler_service: error_handler_service,
        }
    }

    // creates HTTP request pipeline as a service.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IHttpOptions>(services.clone()),
            LoggingService::get_service(services),
            // ServiceCollectionExtensions::get_required_multiple::<dyn IHttpErrorHandler>(services),
            ServiceCollectionExtensions::get_required_single::<dyn IErrorHandlerService>(services.clone()),
        )) as Rc<dyn IHttpRequestPipeline>)]
    }

    // adds the HTTP request pipeline to the given service collection.
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new_from::<dyn IHttpRequestPipeline, Self>(Self::new_service, ServiceScope::Request));
    }

    /// Process the request using the middleware.
    /// 
    /// # Arguments
    /// * `request_context` - The request context
    /// * `services` - The service collection
    /// 
    /// # Returns
    /// * The result of processing the request.
    fn process_request_using_middleware(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<(), Rc<dyn Error>> {
        // Get the middleware services
        let middleware = ServiceCollectionExtensions::get_required_multiple::<dyn IRequestMiddlewareService>(services);
        // Throw an error if there are no middleware services
        if middleware.len() == 0 {
            return Err(Rc::new(RequestError(format!("No middleware configured"))));
        }
        // println!("{} Count of middleware services: {}", request_context.get_path(), middleware.len());
        // println!("{} middleware: {:?}", request_context.get_path(), middleware.iter().map(|x| x.get_type_info().type_name.to_string()).collect::<Vec<String>>());

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
    fn process_request<'a>(self: &Self, connection_context: &dyn IHttpConnectionContext, services: &dyn IServiceCollection) -> Result<(), Rc<dyn Error>> {
        // println!("HttpRequestPipeline::process_request {}", self.times_called.borrow());
        // *self.times_called.borrow_mut() += 1;
        
        let request_result = RequestContext::parse(connection_context);
        match request_result {
            Ok(request_context) => {
                let response_context = ResponseContext::new(&request_context);
                match self.process_request_using_middleware(&response_context, &request_context, services) {
                    Ok(_) => {
                    },
                    Err(err) => {
                        self.error_handler_service.handle_error(err, Some(&request_context), Some(&response_context))?;
                    }
                }

                response_context.set_result_500_if_not_started_writing();
                match response_context.invoke_action_result(&request_context, services) {
                    Ok(_) => {
                    },
                    Err(err) => {
                        self.error_handler_service.handle_error(err, Some(&request_context), Some(&response_context))?;
                    }
                }

                match response_context.connection_context.end_reading_begin_writing() {
                    Ok(_) => {
                        Ok(())
                    },
                    Err(err) => {
                        self.error_handler_service.handle_error(Rc::new(err), Some(&request_context), Some(&response_context))
                    }
                }
            },
            Err(err) => {
                self.error_handler_service.handle_error(Rc::new(err), None, None)
            }
        }
    }
}