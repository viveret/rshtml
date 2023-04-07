use std::rc::Rc;
use std::cell::RefCell;
use std::error::Error;

use crate::action_results::iaction_result::IActionResult;

use crate::controllers::icontroller::IController;
use crate::controllers::controller_actions_map::ControllerActionsMap;

use crate::contexts::controller_context::ControllerContext;
use crate::contexts::request_context::RequestContext;

use crate::services::service_collection::IServiceCollection;

use crate::controllers::route_data_controller_action_matcher::RouteDataControllerActionMatcher;


pub struct IControllerExtensions {
    
}

impl IControllerExtensions {
    pub fn create_context(
        controller: Rc<dyn IController>,
        request_context: Rc<RequestContext>
    ) -> Rc<RefCell<ControllerContext>> {
        Rc::new(RefCell::new(ControllerContext::new(controller, request_context)))
    }

    pub fn process_mvc_request(controller_ctx: Rc<RefCell<ControllerContext>>, services: &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>> {
        let actions_map = Rc::new(ControllerActionsMap::new(controller_ctx.clone(), services));
        let route_matcher = RouteDataControllerActionMatcher::new(actions_map, controller_ctx);
        route_matcher.process_request(services)
    }
}