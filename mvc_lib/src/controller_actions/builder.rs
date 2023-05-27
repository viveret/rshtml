use std::error::Error;
use std::borrow::Cow;
use std::rc::Rc;
use std::cell::RefCell;
use std::vec;

use http::Method;

use crate::action_results::iaction_result::IActionResult;
use crate::controllers::icontroller::IController;
use crate::services::service_collection::IServiceCollection;
use crate::contexts::controller_context::ControllerContext;

use super::controller_action::IControllerAction;
use super::closure::ControllerActionClosure;
use super::member_fn::ControllerActionMemberFn;


// this enum represents the type of route for the controller action.
pub enum RouteType {
    // the route is a closure function. a closure function is a function that is defined inline.
    Closure,
    // the route is a member function. a member function is a function that is defined in a struct.
    MemberFn,
    // the route is a file. a file is a static file that is served from the disk.
    File,
}

// this struct is used to build a controller action.
pub struct ControllerActionBuilder {
    // use this to pass in input model type for binding and validation
    route_pattern: String,
    // used to determine the type of route when building the controller action
    route_type: RefCell<Option<RouteType>>,
    // the HTTP methods allowed for the controller action
    http_methods: RefCell<Option<Vec<Method>>>,
    // the name of the area
    area_name: RefCell<Option<String>>,
    // the name of the controller
    controller_name: RefCell<Option<Cow<'static, str>>>,
    // the name of the action (name of the member, static, or closure function)
    action_name: RefCell<Option<String>>,
    // whether or not the model should be validated for the controller action
    should_validate_model: RefCell<Option<bool>>,
    // the closure function for the controller action (if the route type is a closure)
    closure_fn: RefCell<Option<Rc<dyn Fn(Rc<ControllerContext>, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>>>>>,
    // member_fn: RefCell<Option<Rc<fn(self_arg: T, Rc<ControllerContext>, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>>>>,
    // the member function for the controller action (if the route type is a member function)
    member_fn_action: RefCell<Option<Rc<dyn IControllerAction>>>,
}

impl ControllerActionBuilder {
    // create a new instance of the builder.
    // route_pattern: the route pattern for the controller action.
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
            member_fn_action: RefCell::new(None),
        }
    }

    // set the route pattern for the controller action.
    pub fn set_area_name(self: &Self) -> &Self {
        self
    }

    // set the controller name for the controller action.
    pub fn set_controller_name(self: &Self, name: Cow<'static, str>) -> &Self {
        self.controller_name.replace(Some(name));
        self
    }

    // set the action name for the controller action.
    pub fn set_name(self: &Self, name: &'static str) -> &Self {
        self.action_name.replace(Some(name.to_string()));
        self
    }

    // set the function for the controller action as a member function.
    pub fn set_member_fn<T:'static + IController>(
        self: &Self, 
        // self_arg: T, 
        member_fn: fn(self_arg: &T, Rc<ControllerContext>, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>>
    ) -> &Self {
        self.route_type.replace(Some(RouteType::MemberFn));
        self.member_fn_action.replace(
            Some(
                Rc::new(
                    ControllerActionMemberFn::new(
                        self.http_methods.borrow().as_ref().unwrap_or(&vec![]).clone(),
                    None,
                    self.route_pattern.clone(),
                    self.action_name.borrow().as_ref().unwrap().clone(),
                    self.controller_name.borrow().as_ref().unwrap().clone(),
                    self.area_name.borrow().as_ref().unwrap_or(&String::new()).clone(),
                    self.should_validate_model.borrow().unwrap_or(false),
                    // self_arg,
                    member_fn
                    )
                )
            )
        );
        self
    }

    // set the HTTP methods allowed for the controller action.
    pub fn methods(self: &Self, methods: &[Method]) -> &Self {
        self.http_methods.borrow_mut().replace(methods.to_vec());
        self
    }

    // build the controller action and return the appropriate type for the function type.
    pub fn build(self: &Self) -> Rc<dyn IControllerAction> {
        match self.route_type.borrow().as_ref().unwrap() {
            RouteType::Closure => self.build_closure(),
            RouteType::MemberFn => self.build_member_fn(),
            RouteType::File => todo!(),
        }
    }

    // build the controller action as a member function.
    fn build_member_fn(self: &Self) -> Rc<dyn IControllerAction> {
        self.member_fn_action.borrow().as_ref().unwrap().clone()
    }

    // build the controller action as a closure function.
    fn build_closure(self: &Self) -> Rc<dyn IControllerAction> {
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

// this struct is used to build all of a controller's actions.
pub struct ControllerActionsBuilder<'a, T: IController> {
    // the controller to build the actions for.
    controller: &'a T,
    // the built actions.
    actions: RefCell<Vec<Rc<ControllerActionBuilder>>>,
}

impl<'a, T: IController> ControllerActionsBuilder<'a, T> {
    // create a new instance of the builder.
    pub fn new(controller: &'a T) -> Self {
        Self {
            controller: controller,
            actions: RefCell::new(vec![]),
        }
    }

    // add a controller action to the builder. this will return the new action builder.
    // route_pattern: the route pattern for the controller action.
    // returns: the new action builder.
    pub fn add(self: &Self, route_pattern: &'static str) -> Rc<ControllerActionBuilder> {
        let action = Rc::new(ControllerActionBuilder::new(route_pattern));
        self.actions.borrow_mut().push(action.clone());
        action.set_controller_name(Cow::Borrowed(self.controller.get_type_name()));
        action
    }

    // build all the actions for the controller and return them as a vector.
    // returns: all the actions for the controller.
    pub fn build(self: &Self) -> Vec<Rc<dyn IControllerAction>> {
        let mut actions = vec![];

        for action in self.actions.borrow().iter() {
            actions.push(action.build());
        }

        actions
    }
}