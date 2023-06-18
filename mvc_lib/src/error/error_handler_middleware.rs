use std::error::Error;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::contexts::response_context::IResponseContext;
use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;
use crate::services::service_collection::IServiceCollection;
use crate::services::request_middleware_service::MiddlewareResult;
use crate::services::request_middleware_service::IRequestMiddlewareService;

use super::error_handler_service::IErrorHandlerService;


// this is a middleware that handles errors from other middleware.
// it is the first middleware in the pipeline so that it can handle errors from all other middleware.
// an error can be handled by displaying an error page, logging the error, sending a notification to devs, or any combination.
// unhanded errors will cause the application to panic.
pub struct ErrorHandlerMiddleware {
    error_handling_service: Rc<dyn IErrorHandlerService>,
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>,
}

impl ErrorHandlerMiddleware {
    pub fn new(error_handling_service: Rc<dyn IErrorHandlerService>) -> Self {
        Self {
            error_handling_service,
            next: RefCell::new(None),
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new (
            ServiceCollectionExtensions::get_required_single::<dyn IErrorHandlerService>(services),
        )) as Rc<dyn IRequestMiddlewareService>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new_from::<dyn IRequestMiddlewareService, Self>(Self::new_service, ServiceScope::Request));
    }
}

impl IRequestMiddlewareService for ErrorHandlerMiddleware {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Rc<dyn Error>> {
        let next = self.next.borrow();
        match next.as_ref() {
            Some(next) => {
                let result = next.handle_request(response_context, request_context, services);
                match result {
                    Ok(result) => {
                        Ok(result)
                    },
                    Err(error) => {
                        let result = self.error_handling_service.handle_error(error, Some(request_context), Some(response_context));
                        match result {
                            Ok(_) => {
                                Ok(MiddlewareResult::OkBreak)
                            },
                            Err(error) => {
                                Err(error)
                            },
                        }
                    },
                }
            },
            None => {
                Ok(MiddlewareResult::OkContinue)
            },
        }
    }

    fn get_type_info(&self) -> Box<crate::core::type_info::TypeInfo> {
        Box::new(TypeInfo::of::<ErrorHandlerMiddleware>())
    }
}