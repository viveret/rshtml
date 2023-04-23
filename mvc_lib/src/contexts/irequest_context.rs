use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use http::{Method, HeaderMap};

use crate::core::query_string::QueryString;
use crate::model::view_model_result::ViewModelResult;
use crate::routing::route_data::RouteData;
use crate::services::authorization_service::IAuthClaim;
use crate::controller_actions::controller_action::IControllerAction;

use super::connection_context::IConnectionContext;



pub trait IRequestContext {
    fn get_http_version(self: &Self) -> http::version::Version;
    fn get_name(self: &Self) -> &'static str;
    fn get_path(self: &Self) -> &String;
    fn get_method(self: &Self) -> &Method;
    fn get_body(self: &Self) -> &Vec<u8>;
    
    fn get_model_validation_result(self: &Self) -> Option<ViewModelResult<Rc<dyn Any>>>;
    fn set_model_validation_result(self: &Self, v: Option<ViewModelResult<Rc<dyn Any>>>);

    fn get_connection_context(self: &Self) -> Rc<dyn IConnectionContext>;

    fn get_route_data(self: &Self) -> RouteData;
    fn mut_route_data(self: &Self) -> &RefCell<RouteData>;

    fn get_query(self: &Self) -> &QueryString;

    fn get_controller_action_optional(self: &Self) -> Option<Rc<dyn IControllerAction>>;
    fn get_controller_action(self: &Self) -> Rc<dyn IControllerAction>;
    fn set_controller_action(self: &Self, controller: Option<Rc<dyn IControllerAction>>);

    fn get_auth_claims(self: &Self) -> Vec<Rc<dyn IAuthClaim>>;

    fn get_str(self: &Self, key: &str) -> String;
    fn get_string(self: &Self, key: String) -> String;
    fn insert_str(self: &mut Self, key: &str, value: String) -> String;
    fn insert_string(self: &mut Self, key: String, value: String) -> String;

    fn get_headers(self: &Self) -> &HeaderMap;
    fn get_cookies_parsed(self: &Self) -> Option<HashMap<String, String>>;
}