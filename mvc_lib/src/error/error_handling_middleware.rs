use std::{rc::Rc, cell::RefCell, any::Any};

use crate::services::{request_middleware_service::{IRequestMiddlewareService, MiddlewareResult}, service_collection::{IServiceCollection, ServiceCollectionExtensions, ServiceCollection}, service_descriptor::ServiceDescriptor, service_scope::ServiceScope};

use super::error_handling_service::IErrorHandlingService;



pub struct ErrorHandlingMiddleware {
    error_handling_service: Rc<dyn IErrorHandlingService>,
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>,
}

impl ErrorHandlingMiddleware {
    pub fn new(error_handling_service: Rc<dyn IErrorHandlingService>) -> Self {
        Self {
            error_handling_service,
            next: RefCell::new(None),
        }
    }

    // pub fn set_next(self: &mut Self, next: Box<dyn IRequestMiddlewareService>) {
    //     self.next = Some(next);
    // }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new (
            ServiceCollectionExtensions::get_required_single::<dyn IErrorHandlingService>(services),
        )) as Rc<dyn IRequestMiddlewareService>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new_from::<dyn IRequestMiddlewareService, Self>(Self::new_service, ServiceScope::Request));
    }
}

impl IRequestMiddlewareService for ErrorHandlingMiddleware {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(self: &Self, response_context: &dyn crate::contexts::response_context::IResponseContext, request_context: &dyn crate::contexts::irequest_context::IRequestContext, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn std::error::Error>> {
        let next = self.next.borrow();
        match next.as_ref() {
            Some(next) => {
                let result = next.handle_request(response_context, request_context, services);
                match result {
                    Ok(result) => {
                        Ok(result)
                    },
                    Err(error) => {
                        let result = self.error_handling_service.handle_error(error);
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
}