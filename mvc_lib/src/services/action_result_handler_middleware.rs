use std::cell::RefCell;
use std::{any::Any, error::Error};
use std::rc::Rc;

use crate::core::type_info::TypeInfo;
use crate::{services::request_middleware_service::IRequestMiddlewareService, contexts::{irequest_context::IRequestContext, response_context::IResponseContext}};

use super::service_scope::ServiceScope;
use super::{service_collection::{IServiceCollection, ServiceCollection}, service_descriptor::ServiceDescriptor, request_middleware_service::MiddlewareResult};




pub struct ActionResultHandlerMiddleware {
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>,
}

impl ActionResultHandlerMiddleware {
    pub fn new() -> Self {
        Self {
            next: RefCell::new(None),
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IRequestMiddlewareService>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new_from::<dyn IRequestMiddlewareService, Self>(Self::new_service, ServiceScope::Singleton));
    }
}

impl IRequestMiddlewareService for ActionResultHandlerMiddleware {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Rc<dyn Error>> {
        let result = response_context.get_action_result();
        // println!("ActionResultHandlerMiddleware::handle_request: result: {}", result.as_ref().map(|x| x.to_string()).unwrap_or("None".to_string()));

        match result {
            Some(_) => {
                // response_context.invoke_action_result(request_context, services)?;
                Ok(MiddlewareResult::OkBreak)
            },
            None => {
                // no action result, so continue to next middleware
                Ok(MiddlewareResult::OkContinue)
            },
        }
    }

    fn get_type_info(&self) -> Box<crate::core::type_info::TypeInfo> {
        Box::new(TypeInfo::of::<ActionResultHandlerMiddleware>())
    }
}