use std::cell::RefCell;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;

use crate::action_results::iaction_result::IActionResult;
use crate::contexts::controller_context::IControllerContext;
use crate::contexts::controller_context::ControllerContext;

use crate::controllers::icontroller::IController;

use crate::routing::route_data::RouteData;

use crate::services::service_collection::IServiceCollection;

pub trait IControllerAction {
    fn get_name(self: &Self) -> String;
    fn get_route_pattern(self: &Self) -> String;

    fn is_route_match(self: &Self, controller_context: Rc<RefCell<ControllerContext>>) -> Result<bool, Box<dyn Error>>;
    fn invoke(self: &Self, controller_context: Rc<RefCell<ControllerContext>>, services: &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>>;
}

pub struct ControllerActionClosure<T> where T: Fn(Rc<RefCell<ControllerContext>>, &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>> {
    pub closure_fn: T,
    pub name: String,
    pub route_pattern: String,
}

impl<T> ControllerActionClosure<T> where T: Fn(Rc<RefCell<ControllerContext>>, &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>> {
    pub fn new(
        route_pattern: &str,
        name: &str,
        closure_fn: T) -> Self {
        Self {
            name: name.to_string(),
            route_pattern: route_pattern.to_string(),
            closure_fn: closure_fn
        }
    }
}

impl<T> IControllerAction for ControllerActionClosure<T> where T: Fn(Rc<RefCell<ControllerContext>>, &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>> {
    fn invoke(self: &Self, controller_context: Rc<RefCell<ControllerContext>>, services: &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>> {
        (self.closure_fn)(controller_context, services)
    }

    fn is_route_match(self: &Self, controller_context: Rc<RefCell<ControllerContext>>) -> Result<bool, Box<dyn Error>> {
        let request_context = controller_context.borrow().get_request_context();
        let path = request_context.path.as_str().trim();
        let expected_pattern = self.get_route_pattern();

        // println!("Testing path {} against pattern {}", path, expected_pattern);

        if expected_pattern.ends_with("..") {
            Ok(path.starts_with(&expected_pattern[..expected_pattern.len() - 2]))
        } else {
            Ok(path == expected_pattern)
        }
    }

    fn get_name(self: &Self) -> String {
        self.name.clone()
    }

    fn get_route_pattern(self: &Self) -> String {
        self.route_pattern.clone()
    }

}

pub trait IControllerActionsMap {
    fn get_actions(self: &Self) -> Vec<Box<dyn IControllerAction>>;
}

pub struct ControllerActionsMap {
    pub controller_context: Rc<RefCell<ControllerContext>>
}

impl ControllerActionsMap {
    pub fn new(controller_context: Rc<RefCell<ControllerContext>>, services: &dyn IServiceCollection) -> Self {
        Self {
            controller_context: controller_context
        }
    }
}

impl IControllerActionsMap for ControllerActionsMap  {
    fn get_actions(self: &Self) -> Vec<Box<dyn IControllerAction>> {
        self.controller_context.borrow().controller.get_actions()
    }
}

impl ControllerActionsMap  {
}