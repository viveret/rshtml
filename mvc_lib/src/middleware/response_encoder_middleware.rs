use std::cell::RefCell;
use std::error::Error;
use std::{rc::Rc, any::Any};

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::ResponseContext;
use crate::core::type_info::TypeInfo;
use crate::auth::iauthroles_dbset_provider::IAuthRolesDbSetProvider;
use crate::services::request_middleware_service::{IRequestMiddlewareService, MiddlewareResult};
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollection};


pub struct ResponseEncoderMiddleware {
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>
}

impl ResponseEncoderMiddleware {
    pub fn new() -> Self {
        Self {
            next: RefCell::new(None)
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IRequestMiddlewareService>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRequestMiddlewareService>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IRequestMiddlewareService for ResponseEncoderMiddleware {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(self: &Self, request_context: Rc<dyn IRequestContext>, response_context: Rc<ResponseContext>, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn Error>> {
        if let Some(next) = self.next.borrow().as_ref() {
            let next_response = next.handle_request(request_context.clone(), response_context.clone(), services)?;

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