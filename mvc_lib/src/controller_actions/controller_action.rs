use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;

use http::method::Method;

use crate::contexts::irequest_context::IRequestContext;
use crate::controller_action_features::controller_action_feature::IControllerActionFeature;

use crate::contexts::controller_context::ControllerContext;

use crate::routing::action_path::ActionPath;
use crate::services::service_collection::IServiceCollection;

use super::route_pattern::ControllerActionRoutePattern;


// this trait represents a controller action that can be invoked by HTTP requests.
// controller actions are the main way that HTTP requests are processed.
// controller actions are mapped to HTTP requests by using a route pattern.
// controller actions can be invoked by using a member function, a static function, or a closure.
// controller actions can be decorated with controller action features to add functionality to the controller action.
pub trait IControllerAction {
    // get a string representation of the controller action.
    fn to_string(self: &Self) -> String;
    // get the path for the controller action.
    fn get_path(self: &Self) -> ActionPath;
    // get the name of the controller action (the name of the member function, the name of the static function, or the name of the closure).
    fn get_name(self: &Self) -> String;
    // get the name of the controller.
    fn get_controller_name(self: &Self) -> Cow<'static, str>;
    // get the name of the area.
    fn get_area_name(self: &Self) -> String;
    // get the route pattern for the controller action.
    fn get_route_pattern(self: &Self) -> Rc<ControllerActionRoutePattern>;
    // get the HTTP methods allowed for the controller action.
    fn get_http_methods_allowed(self: &Self) -> Vec<Method>;
    // get whether or not the model should be validated for the controller action.
    fn get_should_validate_model(self: &Self) -> bool;
    // get the controller action features for the controller action.
    fn get_features(self: &Self) -> Vec<Rc<dyn IControllerActionFeature>>;
    // get whether or not the action matches the request.
    fn is_route_match(self: &Self, request_context: Rc<dyn IRequestContext>) -> Result<bool, Box<dyn Error>>;
    // invoke the controller action for the request and context.
    fn invoke(self: &Self, request_context: Rc<ControllerContext>, services: &dyn IServiceCollection) -> Result<(), Box<dyn Error>>;
}

// extension methods for IControllerAction
pub struct IControllerActionExtensions {}
impl IControllerActionExtensions {
    // get whether or not the action matches the request.
    // action: the controller action.
    // request_context: the request context.
    // returns: whether or not the action matches the request.
    pub fn is_method_match(action: &dyn IControllerAction, request_context: Rc<dyn IRequestContext>) -> bool {
        let http_methods_allowed = action.get_http_methods_allowed();
        let r = http_methods_allowed.len() == 0 ||
            http_methods_allowed.contains(request_context.get_method());

        // println!("is_method_match: {} is in {:?} = {}", request_context.method.as_ref(), http_methods_allowed, r);

        r
    }
}