use std::cell::RefCell;
use std::error::Error;
use std::{rc::Rc, any::Any};

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::IResponseContext;
use crate::core::type_info::TypeInfo;
use crate::services::request_middleware_service::{IRequestMiddlewareService, MiddlewareResult};
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollection};


// this middleware is used to encode the response body.
pub struct ResponseEncoderMiddleware {
    // the next middleware in the pipeline
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>
}

impl ResponseEncoderMiddleware {
    pub fn new() -> Self {
        Self {
            next: RefCell::new(None)
        }
    }

    // this is the function that will be called by the service collection to create a new instance of the middleware
    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IRequestMiddlewareService>)]
    }

    // this is called by the application to add the middleware to the service collection
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRequestMiddlewareService>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IRequestMiddlewareService for ResponseEncoderMiddleware {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn Error>> {
        // get accept header from request
        let accept_header = request_context.get_headers().get("Accept").unwrap();
        let _accept_str = accept_header.to_str().unwrap();
        
        // get encoder from service collection
        // let encoder = ServiceCollectionExtensions::get_required_single::<dyn IHttpBodyStreamFormat>(services).get(accept_str);

        if let Some(next) = self.next.borrow().as_ref() {
            let next_response = next.handle_request(response_context, request_context, services)?;

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