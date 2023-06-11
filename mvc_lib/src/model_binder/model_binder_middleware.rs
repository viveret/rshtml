use std::{rc::Rc, any::Any, cell::RefCell};

use crate::contexts::response_context::IResponseContext;
use crate::core::type_info::TypeInfo;

use crate::contexts::irequest_context::IRequestContext;
use crate::services::request_middleware_service::MiddlewareResult;
use crate::services::request_middleware_service::IRequestMiddlewareService;
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::ServiceCollectionExtensions;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_collection::IServiceCollection;

use super::imodelbinder_service::IModelBinderService;


// this struct represents a middleware that binds the model of the request.
pub struct ModelBinderMiddleware {
    // the model binder service
    _model_binder_service: Rc<dyn IModelBinderService>,
    // the next middleware in the pipeline
    next: Rc<RefCell<Option<Rc<dyn IRequestMiddlewareService>>>>,
}

impl ModelBinderMiddleware {
    // creates a new instance of ModelBinderMiddleware.
    // model_binder_service: the model binder service.
    // returns: a new instance of ModelBinderMiddleware.
    pub fn new(
        model_binder_service: Rc<dyn IModelBinderService>,
    ) -> Self {
        Self {
            _model_binder_service: model_binder_service,
            next: Rc::new(RefCell::new(None)),
        }
    }

    // creates a new instance of ModelBinderMiddleware as a service from the given IServiceCollection.
    // services: the IServiceCollection to create the ModelBinderMiddleware from.
    // returns: a Vec of Box<dyn Any> containing the ModelBinderMiddleware as a service.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IModelBinderService>(services),
        )) as Rc<dyn IRequestMiddlewareService>)]
    }

    // adds the ModelBinderMiddleware to the given IServiceCollection.
    // services: the IServiceCollection to add the ModelBinderMiddleware to.
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(
            ServiceDescriptor::new(
                TypeInfo::rc_of::<dyn IRequestMiddlewareService>(),
                Self::new_service,
                ServiceScope::Singleton,
            ),
        );
    }
}

impl IRequestMiddlewareService for ModelBinderMiddleware {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        *self.next.borrow_mut() = next;
    }

    fn handle_request(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn std::error::Error>> {
        // println!("ModelBinderMiddleware.handle_request");

        // request_context.set_model_validation_result(Some(self.model_binder_service.bind_model(request_context, &request_context.get_type_info()));
        // request_context.bind

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