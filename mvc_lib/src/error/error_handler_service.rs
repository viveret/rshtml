use std::any::Any;
use std::rc::Rc;
use std::error::Error;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::IResponseContext;
use crate::diagnostics::logging::logging_service::ILoggingService;
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::ServiceCollectionExtensions;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_collection::IServiceCollection;

use super::error_context::ErrorContext;
use super::ierror_context::IErrorContext;
use super::ierror_handler::IErrorHandler;


// this is a trait for a class that can be used to handle errors by using any number of error handlers.
pub trait IErrorHandlerService {
    fn handle_error(self: &Self, error: Rc<dyn Error>, request_context: &dyn IRequestContext, response_context: &dyn IResponseContext) -> Result<(), Rc<dyn Error>>;
}

pub struct ErrorHandlerService {
    logging_service: Rc<dyn ILoggingService>,
    error_handlers: Vec<Rc<dyn IErrorHandler>>,
}

impl ErrorHandlerService {
    pub fn new(
        logging_service: Rc<dyn ILoggingService>,
        error_handlers: Vec<Rc<dyn IErrorHandler>>,
    ) -> Self {
        Self {
            logging_service: logging_service,
            error_handlers: error_handlers,
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn ILoggingService>(services),
            ServiceCollectionExtensions::try_get_multiple::<dyn IErrorHandler>(services).unwrap_or(vec![]),
        )) as Rc<dyn IErrorHandlerService>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new_from::<dyn IErrorHandlerService, Self>(Self::new_service, ServiceScope::Singleton));
    }
}

impl IErrorHandlerService for ErrorHandlerService {
    fn handle_error(self: &Self, error: Rc<dyn Error>, request_context: &dyn IRequestContext, response_context: &dyn IResponseContext) -> Result<(), Rc<dyn Error>> {
        self.logging_service.log_error(format!("ErrorHandlerService::handle_error: {:?}", error).as_str());

        // try to handle the error with the error handlers. If at least one error handler handles the error, then return Ok(true).
        // if none of the error handlers handle the error, then return Err(error).
        let error_context = ErrorContext::new(error, request_context, response_context);
        for error_handler in self.error_handlers.iter() {
            match error_handler.handle_error(&error_context) {
                Ok(_) => error_context.set_handled(),
                Err(e) => {
                    // error while processing error, not good.
                    return Err(e);
                },
            }
        }

        if error_context.is_handled() {
            // at least one error handler handled the error.
            Ok(())
        } else {
            // none of the error handlers handled the error.
            Err(error_context.get_error())
        }
    }
}