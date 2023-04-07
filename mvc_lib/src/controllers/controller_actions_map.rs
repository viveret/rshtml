use std::cell::RefCell;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;

use crate::action_results::iaction_result::IActionResult;
use crate::contexts::controller_context::IControllerContext;
use crate::contexts::controller_context::ControllerContext;

use crate::services::service_collection::IServiceCollection;


pub trait IControllerAction {
    fn get_name(self: &Self) -> String;
    fn get_controller_name(self: &Self) -> String;
    fn get_area_name(self: &Self) -> String;
    fn get_route_pattern(self: &Self) -> String;

    fn is_route_match(self: &Self, controller_context: Rc<RefCell<ControllerContext>>) -> Result<bool, Box<dyn Error>>;
    fn invoke(self: &Self, controller_context: Rc<RefCell<ControllerContext>>, services: &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>>;
}

pub struct ControllerActionClosure<T> where T: Fn(Rc<RefCell<ControllerContext>>, &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>> {
    pub closure_fn: T,
    pub name: String,
    pub controller_name: String,
    pub area_name: String,
    pub route_pattern: String,
}

impl<T> ControllerActionClosure<T> where T: Fn(Rc<RefCell<ControllerContext>>, &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>> {
    pub fn new(
        route_pattern: &'static str,
        name: &'static str,
        controller_name: &'static str,
        area_name: &'static str,
        closure_fn: T) -> Self {
        Self {
            name: name.to_string(),
            controller_name: controller_name.to_string(),
            area_name: area_name.to_string(),
            route_pattern: route_pattern.to_string(),
            closure_fn: closure_fn
        }
    }
    
    pub fn new_default_area(
        route_pattern: &'static str,
        name: &'static str,
        controller_name: &'static str,
        closure_fn: T) -> Self {
        Self {
            name: name.to_string(),
            controller_name: controller_name.to_string(),
            area_name: "".to_string(),
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
        let route_pattern = self.get_route_pattern();

        // println!("Testing path {} against pattern {}", path, route_pattern);

        if route_pattern.ends_with("..") {
            Ok(path.starts_with(&route_pattern[..route_pattern.len() - 2]))
        } else {
            Ok(path == route_pattern)
        }
    }

    fn get_name(self: &Self) -> String {
        self.name.clone()
    }

    fn get_controller_name(self: &Self) -> String {
        self.controller_name.clone()
    }

    fn get_area_name(self: &Self) -> String {
        self.area_name.clone()
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
    pub fn new(controller_context: Rc<RefCell<ControllerContext>>, _services: &dyn IServiceCollection) -> Self {
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