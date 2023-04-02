use std::cell::RefCell;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;

use crate::action_results::iaction_result::IActionResult;
use crate::contexts::request_context::RequestContext;
use crate::contexts::controller_context::ControllerContext;

use crate::controllers::icontroller::IController;

use crate::services::service_collection::IServiceCollection;

pub trait IControllerAction {
    fn invoke(self: &Self, controller_ctx: Rc<RefCell<ControllerContext>>, request_ctx: Rc<RequestContext>, services: &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>>;
}

pub trait IControllerWithActions: IController {
    fn get_actions(self: &Self, controller_ctx: Rc<RefCell<ControllerContext>>, request_ctx: Rc<RequestContext>, services: &dyn IServiceCollection) -> Vec<Box<dyn IControllerAction>>;
}

pub struct ControllerActionMapped<'a> {
    pub inner_controller: &'a dyn IControllerWithActions
}

impl <'a> ControllerActionMapped<'a> {
    pub fn new(inner_controller: &'a dyn IControllerWithActions) -> Self {
        Self {
            inner_controller: inner_controller
        }
    }
}

impl <'a> IController for ControllerActionMapped <'a> {
    fn process_request(self: &Self, controller_ctx: Rc<RefCell<ControllerContext>>, request_ctx: Rc<RequestContext>, services: &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>> {
        let actions = self.inner_controller.get_actions(controller_ctx.clone(), request_ctx.clone(), services);
        let find_action = actions.first().unwrap();
        return find_action.invoke(controller_ctx, request_ctx, services);
    }

    fn get_route_area(self: &Self) -> Option<String> {
        self.inner_controller.get_route_area()
    }
}