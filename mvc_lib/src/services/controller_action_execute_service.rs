use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::error::Error;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::request_context::RequestContext;
use crate::contexts::response_context::ResponseContext;
use crate::contexts::controller_context::IControllerContext;

use crate::controllers::icontroller_extensions::IControllerExtensions;
use crate::controllers::controller_actions_map::IControllerActionsMap;

use crate::services::routemap_service::IRouteMapService;
use crate::services::request_middleware_service::MiddlewareResult;
use crate::services::request_middleware_service::IRequestMiddlewareService;
use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;

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
}

impl IRequestMiddlewareService for ControllerActionExecuteService {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(self: &Self, request_context: Rc<dyn IRequestContext>, response_context: Rc<ResponseContext>, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn Error>> {
        if let Some(action) = request_context.get_controller_action_optional() {
            let controller = self.mapper_service.get_mapper().get_controller(action.get_controller_name().to_string());
            let controller_context = IControllerExtensions::create_context(controller.clone(), request_context.clone());
            action.invoke(controller_context.clone(), services)?;
            let result = controller_context.get_action_result();

            match result {
                Some(action_result) => {
                    response_context.status_code.replace(action_result.get_statuscode());
                    action_result.configure_response(controller_context.clone(), response_context.clone(), request_context.clone(), services);
                },
                None => {
                    // println!("Trying next path if available");
                },
            }
        }
                
        Ok(MiddlewareResult::OkContinue)
    }
}