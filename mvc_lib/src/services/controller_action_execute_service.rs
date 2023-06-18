use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::error::Error;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::IResponseContext;
use crate::contexts::controller_context::IControllerContext;

use crate::controllers::icontroller_extensions::IControllerExtensions;

use crate::core::type_info::TypeInfo;
use crate::services::routemap_service::IRouteMapService;
use crate::services::request_middleware_service::MiddlewareResult;
use crate::services::request_middleware_service::IRequestMiddlewareService;
use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;

use super::service_descriptor::ServiceDescriptor;
use super::service_scope::ServiceScope;

// this is the service that handles executing controller actions.
pub struct ControllerActionExecuteService {
    // the route map service.
    mapper_service: Rc<dyn IRouteMapService>,
    // the next middleware service in the pipeline
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>
}

impl ControllerActionExecuteService {
    // creates a new instance of the service.
    // mapper_service: the route map service.
    // returns: the new instance of the service.
    pub fn new(mapper_service: Rc<dyn IRouteMapService>) -> Self {
        Self { mapper_service: mapper_service, next: RefCell::new(None) }
    }

    // creates a new instance of the service for the service collection.
    // services: the service collection.
    // returns: a vector containing the new instance of the service.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IRouteMapService>(services)
        )) as Rc<dyn IRequestMiddlewareService>)]
    }

    // adds the service to the service collection.
    pub fn add_to_services(services: &mut super::service_collection::ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRequestMiddlewareService>(), ControllerActionExecuteService::new_service, ServiceScope::Singleton));
    }
}

impl IRequestMiddlewareService for ControllerActionExecuteService {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Rc<dyn Error>> {
        if let Some(action) = request_context.get_controller_action_optional() {
            let controller = self.mapper_service.get_mapper().get_controller(action.get_controller_name().to_string());
            let controller_context = IControllerExtensions::create_context(controller.clone(), request_context, response_context);
            action.invoke(&controller_context, services)?;
        }

        if let Some(next) = self.next.borrow().as_ref() {
            next.handle_request(response_context, request_context, services)
        } else {
            Ok(MiddlewareResult::OkContinue)
        }
    }

    fn get_type_info(&self) -> Box<TypeInfo> {
        Box::new(TypeInfo::of::<ControllerActionExecuteService>())
    }
}