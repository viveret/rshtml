use std::error::Error;
use std::any::Any;
use std::borrow::{Borrow, Cow};
use std::rc::Rc;
use std::cell::RefCell;

use http::Method;

use crate::action_results::iaction_result::IActionResult;
use crate::services::service_collection::IServiceCollection;
use crate::contexts::controller_context::ControllerContext;

use super::controller_action::IControllerAction;
use super::closure::ControllerActionClosure;


pub enum RouteType {
    Closure,
    MemberFn,
    File,
}

pub struct ControllerActionBuilder {
    // use this to pass in input model type for binding and validation
    route_pattern: String,
    route_type: RefCell<Option<RouteType>>,
    http_methods: RefCell<Option<Vec<Method>>>,
    area_name: RefCell<Option<String>>,
    controller_name: RefCell<Option<Cow<'static, str>>>,
    action_name: RefCell<Option<String>>,
    should_validate_model: RefCell<Option<bool>>,
    closure_fn: RefCell<Option<Rc<dyn Fn(Rc<ControllerContext>, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>>>>>,
}

impl ControllerActionBuilder {
    pub fn new(route_pattern: &'static str) -> Self {
        Self {
            route_pattern: route_pattern.to_string(),
            route_type: RefCell::new(None),
            http_methods: RefCell::new(None),
            area_name: RefCell::new(None),
            controller_name: RefCell::new(None),
            action_name: RefCell::new(None),
            should_validate_model: RefCell::new(None),
            closure_fn: RefCell::new(None),
        }
    }

    pub fn set_area_name(self: &Self) -> &Self {
        self
    }

    pub fn set_controller_name(self: &Self) -> &Self {
        self
    }

    pub fn set_name(self: &Self) -> &Self {
        self
    }

    pub fn set_method(self: &Self) -> &Self {
        self
    }

    pub fn set_methods(self: &Self) -> &Self {
        self
    }

    pub fn build(self: &Self) -> Rc<dyn IControllerAction + '_> {
        match self.route_type.borrow().as_ref().unwrap() {
            RouteType::Closure => self.build_closure(),
            RouteType::MemberFn => todo!(),
            RouteType::File => todo!(),
        }
    }

    fn build_closure(self: &Self) -> Rc<dyn IControllerAction + '_> {
        Rc::new(
            ControllerActionClosure::new(
                self.http_methods.borrow().as_ref().unwrap().clone(),
            None,
            self.route_pattern.clone(),
            self.action_name.borrow().as_ref().unwrap().clone(),
            self.controller_name.borrow().as_ref().unwrap().clone(),
            self.area_name.borrow().as_ref().unwrap().clone(),
            self.should_validate_model.borrow().unwrap(),
            self.closure_fn.borrow().as_ref().unwrap().clone()
            )
        )
    }
}