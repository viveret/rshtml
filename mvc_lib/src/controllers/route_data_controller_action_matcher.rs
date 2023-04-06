use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use crate::action_results::iaction_result::IActionResult;

use crate::contexts::controller_context::IControllerContext;
use crate::contexts::controller_context::ControllerContext;
use crate::contexts::request_context::RequestContext;

use crate::controllers::controller_actions_map::IControllerActionsMap;

use crate::services::service_collection::IServiceCollection;


pub struct RouteDataControllerActionMatcher {
    actions_map: Rc<dyn IControllerActionsMap>,
    controller_context: Rc<RefCell<ControllerContext>>
}

impl RouteDataControllerActionMatcher {
    pub fn new(
        actions_map: Rc<dyn IControllerActionsMap>,
        controller_context: Rc<RefCell<ControllerContext>>
    ) -> Self {
        Self {
            actions_map: actions_map,
            controller_context: controller_context,
        }
    }

    pub fn process_request(self: &Self, services: &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>> {
        // Some(self.controller_context.borrow().route_data_result.as_ref().unwrap().clone())
        let actions = self.actions_map.get_actions();
        let route_data = self.controller_context.borrow().get_route_data_result();

        for action in actions {
            // println!("Testing {}", action.get_name());
            
            if !action.is_route_match(self.controller_context.clone())? {
                // println!("Route does not match");
                continue;
            }

            let result = action.invoke(self.controller_context.clone(), services)?;
            match result {
                Some(action_result) => {
                    // println!("Returning result");
                    // todo: make this a part of context_data so we can pass action and have access to the action during rendering
                    self.controller_context.borrow().insert_str("ActionName", action.get_name());
                    self.controller_context.borrow().insert_str("ControllerName", action.get_controller_name());
                    self.controller_context.borrow().insert_str("AreaName", action.get_area_name());
                    return Ok(Some(action_result));
                },
                None => {
                    // println!("Trying next path if available");
                },
            }
        }
        Ok(None)
    }
}