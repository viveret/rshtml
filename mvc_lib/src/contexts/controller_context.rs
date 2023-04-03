use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use std::ops::Deref;

use crate::contexts::request_context::RequestContext;

use crate::controllers::icontroller::IController;

use crate::routing::route_data::RouteData;


pub trait IControllerContext {
    fn get_request_context(self: &Self) -> Rc<RequestContext>;
    fn get_context_data(self: &Self) -> Rc<RefCell<HashMap<String, Rc<Box<dyn Any>>>>>;
    fn get_view_data(self: &Self) -> Rc<RefCell<HashMap<String, Rc<Box<dyn Any>>>>>;
    fn get_view_data_value(self: &Self, key: &str) -> Option<Rc<Box<dyn Any>>>;
    fn get_controller(self: &Self) -> Rc<dyn IController>;
    fn get_route_data_result(self: &Self) -> Result<Box<RouteData>, Box<dyn Error>>;
}

pub struct ControllerContext {
    pub request_context: Rc<RequestContext>,
    pub context_data: Rc<RefCell<HashMap<String, Rc<Box<dyn Any>>>>>,
    pub view_data: Rc<RefCell<HashMap<String, Rc<Box<dyn Any>>>>>,
    pub controller: Rc<dyn IController>,
}

impl ControllerContext {
    pub fn new(
        controller: Rc<dyn IController>,
        request_context: Rc<RequestContext>
    ) -> Self {
        Self {
            request_context: request_context.clone(),
            context_data: Rc::new(RefCell::new(HashMap::new())),
            view_data: Rc::new(RefCell::new(HashMap::new())),
            controller: controller.clone(),
        }
    }

    pub fn parse_route_data(
        controller: Rc<dyn IController>,
        request_context: Rc<RequestContext>
    ) -> Result<Box<RouteData>, Box<dyn Error>> {
        let mut route_data = RouteData::new();
        if let Some(route_area) = controller.get_route_area() {
            route_data.map.insert("area".to_string(), route_area);
        }
        
        if let Some(controller_name) = controller.get_name() {
            route_data.map.insert("controller".to_string(), controller_name);
        }

        // todo: search actions for applicable patterns

        // todo: insert query params or route params

        Ok(Box::new(route_data))
    }
}

impl IControllerContext for ControllerContext {
    fn get_request_context(self: &Self) -> Rc<RequestContext> {
        self.request_context.clone()
    }

    fn get_context_data(self: &Self) -> Rc<RefCell<HashMap<String, Rc<Box<dyn Any>>>>> {
        self.context_data.clone()
    }

    fn get_view_data(self: &Self) -> Rc<RefCell<HashMap<String, Rc<Box<dyn Any>>>>> {
        self.view_data.clone()
    }
    
    fn get_view_data_value(self: &Self, key: &str) -> Option<Rc<Box<dyn Any>>> {
        if self.view_data.as_ref().borrow().contains_key(key) {
            Some(self.view_data.as_ref().borrow().get(key).expect("oops").clone())
        } else {
            None
        }
    }

    fn get_controller(self: &Self) -> Rc<dyn IController> {
        self.controller.clone()
    }

    fn get_route_data_result(self: &Self) -> Result<Box<RouteData>, Box<dyn Error>> {
        Self::parse_route_data(self.controller.clone(), self.request_context.clone())
    }
}