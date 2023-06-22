use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::iresponse_context::IResponseContext;
use crate::core::type_info::TypeInfo;

use crate::services::authorization_service::AuthResult;
use crate::services::authorization_service::IAuthorizationService;

use crate::services::request_middleware_service::IRequestMiddlewareService;
use crate::services::request_middleware_service::MiddlewareResult;

use crate::services::routemap_service::IRouteMapService;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;


// this middleware is used to authorize a controller action.
pub struct AuthorizeControllerActionFeatureMiddleware {
    mapper_service: Rc<dyn IRouteMapService>,
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>
}

impl AuthorizeControllerActionFeatureMiddleware {
    // create a new instance of the middleware.
    // mapper_service - the route map service. this is used to get the controller action.
    // returns the new instance of the middleware.
    pub fn new(mapper_service: Rc<dyn IRouteMapService>) -> Self {
        Self { mapper_service: mapper_service, next: RefCell::new(None) }
    }

    // this is the function that will be called by the service collection to create a new instance of the middleware
    // services - the service collection
    // returns a vector containing the new instance of the middleware.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IRouteMapService>(services)
        )) as Rc<dyn IRequestMiddlewareService>)]
    }
    
    // this is called by the application to add the middleware to the service collection
    // services - the service collection
    // returns nothing
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRequestMiddlewareService>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IRequestMiddlewareService for AuthorizeControllerActionFeatureMiddleware {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Rc<dyn Error>> {
        let auth_service = ServiceCollectionExtensions::get_required_single::<dyn IAuthorizationService>(services);
        let controller_name = request_context.get_str("ControllerName");

        if controller_name.len() > 0 {
            let controller = self.mapper_service.get_mapper().get_controller(controller_name.clone());
            match auth_service.authenticate_http_request(controller, request_context)? {
                AuthResult::Ok => {
                    // should make note of authorization somewhere
                },
                AuthResult::Rejection(reason) => {
                    println!("Request denied, unauthorized: {:?}", reason);
                    response_context.set_status_code(http::StatusCode::NOT_FOUND);
                    return Ok(MiddlewareResult::OkBreak); // short circuit middleware
                },
            }
        }
        
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

    fn get_type_info(&self) -> Box<TypeInfo> {
        Box::new(TypeInfo::of::<AuthorizeControllerActionFeatureMiddleware>())
    }
}