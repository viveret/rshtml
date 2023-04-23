use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;
use std::str::FromStr;

use http::method::Method;

use crate::action_results::iaction_result::IActionResult;
use crate::action_results::file_result::FileResult;

use crate::contexts::irequest_context::IRequestContext;
use crate::controller_action_features::controller_action_feature::IControllerActionFeature;

use crate::contexts::controller_context::ControllerContext;

use crate::services::service_collection::IServiceCollection;

use super::route_pattern::ControllerActionRoutePattern;


pub trait IControllerAction {
    fn to_string(self: &Self) -> String;
    fn get_path(self: &Self) -> String;

    fn get_name(self: &Self) -> String;
    fn get_controller_name(self: &Self) -> Cow<'static, str>;
    fn get_area_name(self: &Self) -> String;
    fn get_route_pattern(self: &Self) -> Rc<ControllerActionRoutePattern>;

    fn get_http_methods_allowed(self: &Self) -> Vec<Method>;
    
    fn get_should_validate_model(self: &Self) -> bool;

    fn get_features(self: &Self) -> Vec<Rc<dyn IControllerActionFeature>>;

    fn is_route_match(self: &Self, request_context: Rc<dyn IRequestContext>) -> Result<bool, Box<dyn Error>>;

    fn invoke(self: &Self, request_context: Rc<ControllerContext>, services: &dyn IServiceCollection) -> Result<(), Box<dyn Error>>;
}


pub struct IControllerActionExtensions {}
impl IControllerActionExtensions {
    pub fn is_method_match(action: &dyn IControllerAction, request_context: Rc<dyn IRequestContext>) -> bool {
        let http_methods_allowed = action.get_http_methods_allowed();
        let r = http_methods_allowed.len() == 0 ||
            http_methods_allowed.contains(request_context.get_method());

        // println!("is_method_match: {} is in {:?} = {}", request_context.method.as_ref(), http_methods_allowed, r);

        r
    }
}