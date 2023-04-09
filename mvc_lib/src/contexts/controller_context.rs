use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

use crate::action_results::iaction_result::IActionResult;

use crate::contexts::request_context::RequestContext;

use crate::controllers::icontroller::IController;
use crate::controllers::icontroller_extensions::IControllerExtensions;

use crate::routing::route_data::RouteData;


pub trait IControllerContext {
    fn get_request_context(self: &Self) -> Rc<RequestContext>;
    fn get_context_data(self: &Self) -> HashMap<String, Rc<Box<dyn Any>>>;
    fn get_view_data(self: &Self) -> HashMap<String, String>;
    fn get_controller(self: &Self) -> Rc<dyn IController>;
    fn get_route_data_result(self: &Self) -> Result<Box<RouteData>, Box<dyn Error>>;

    fn get_action_result(self: &Self) -> Option<Rc<dyn IActionResult>>;
    fn set_action_result(self: &Self, action_result: Option<Rc<dyn IActionResult>>);

    fn get_string(self: &Self, key: String) -> String;
    fn get_str(self: &Self, key: &str) -> String;
    
    fn insert_string(self: &Self, key: String, value: String) -> String;
    fn insert_str(self: &Self, key: &str, value: String) -> String;
}

pub struct ControllerContext {
    pub request_context: Rc<RequestContext>,
    pub context_data: RefCell<HashMap<String, Rc<Box<dyn Any>>>>,
    pub view_data: RefCell<HashMap<String, String>>,
    pub controller: Rc<dyn IController>,
    pub action_result: RefCell<Option<Rc<dyn IActionResult>>>,
}

impl ControllerContext {
    pub fn new(
        controller: Rc<dyn IController>,
        request_context: Rc<RequestContext>
    ) -> Self {
        Self {
            request_context: request_context.clone(),
            context_data: RefCell::new(HashMap::new()),
            view_data: RefCell::new(HashMap::new()),
            controller: controller.clone(),
            action_result: RefCell::new(None),
        }
    }

    pub fn parse_route_data(
        self: &Self,
    ) -> Result<Box<RouteData>, Box<dyn Error>> {
        let mut route_data = RouteData::new();
        
        let mut area_name = self.get_str("AreaName");
        if area_name.len() == 0 {
            area_name = self.controller.get_route_area().to_string();
        }
        route_data.map.insert("area".to_string(), area_name);
        
        let mut controller_name = self.get_str("ControllerName");
        if controller_name.len() == 0 {
            controller_name = IControllerExtensions::get_name(self.controller.clone());
        }
        route_data.map.insert("controller".to_string(), controller_name);


        let action_name = self.get_str("ActionName");
        // if action_name.len() == 0 {
        //     action_name = action.get_name();
        // }
        route_data.map.insert("action".to_string(), action_name);

        // todo: search actions for applicable patterns

        // todo: insert query params or route params

        Ok(Box::new(route_data))
    }
}

impl IControllerContext for ControllerContext {
    fn get_request_context(self: &Self) -> Rc<RequestContext> {
        self.request_context.clone()
    }

    fn get_context_data(self: &Self) -> HashMap<String, Rc<Box<dyn Any>>> {
        self.context_data.borrow().clone()
    }

    fn get_view_data(self: &Self) -> HashMap<String, String> {
        self.view_data.borrow().clone()
    }

    fn get_controller(self: &Self) -> Rc<dyn IController> {
        self.controller.clone()
    }

    fn get_route_data_result(self: &Self) -> Result<Box<RouteData>, Box<dyn Error>> {
        self.parse_route_data()
    }

    fn get_string(self: &Self, key: String) -> String {
        match self.view_data.borrow().get(&key) {
            Some(s) => s.clone(),
            None => {
                self.request_context.as_ref().get_string(key)
            },
        }
    }

    fn get_str(self: &Self, key: &str) -> String {
        self.get_string(key.to_string())
    }
    
    fn insert_string(self: &Self, key: String, value: String) -> String {
        self.view_data.borrow_mut().insert(key, value.clone());
        value
    }

    fn insert_str(self: &Self, key: &str, value: String) -> String {
        self.insert_string(key.to_string(), value)
    }

    fn get_action_result(self: &Self) -> Option<Rc<dyn IActionResult>> {
        match self.action_result.borrow().clone() {
            Some(action_result) => Some(action_result),
            None => None,
        }
    }

    fn set_action_result(self: &Self, action_result: Option<Rc<dyn IActionResult>>) {
        self.action_result.replace(action_result);
    }

}