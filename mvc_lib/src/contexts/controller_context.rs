use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::controllers::icontroller::IController;


pub trait IControllerContext {
    fn get_context_data(self: &Self) -> Rc<RefCell<HashMap<String, Rc<Box<dyn Any>>>>>;
    fn get_view_data(self: &Self) -> Rc<RefCell<HashMap<String, Rc<Box<dyn Any>>>>>;
    fn get_view_data_value(self: &Self, key: &str) -> Option<Rc<Box<dyn Any>>>;
    fn get_controller(self: &Self) -> Rc<Box<dyn IController>>;
}

pub struct ControllerContext {
    pub context_data: Rc<RefCell<HashMap<String, Rc<Box<dyn Any>>>>>,
    pub view_data: Rc<RefCell<HashMap<String, Rc<Box<dyn Any>>>>>,
    pub controller: Option<Rc<Box<dyn IController>>>,
}

impl ControllerContext {
    pub fn new(controller: Option<Rc<Box<dyn IController>>>) -> Self {
        Self {
            context_data: Rc::new(RefCell::new(HashMap::new())),
            view_data: Rc::new(RefCell::new(HashMap::new())),
            controller: controller,
        }
    }
}

impl IControllerContext for ControllerContext {
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

    fn get_controller(self: &Self) -> Rc<Box<dyn IController>> {
        self.controller.as_ref().expect("no controllers").clone()
    }
}