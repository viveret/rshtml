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


// this trait represents a HTTP request and its context.
pub trait IRequestContext {
    // get the HTTP version of the request
    fn get_http_version(self: &Self) -> http::version::Version;
    // get the method of the request
    fn get_url(self: &Self) -> url::Url;
    // get the type name of the request Rust type
    fn get_type_name(self: &Self) -> &'static str;
    // get the host name? of the request
    fn get_host_name(self: &Self) -> &String;
    // get the port of the request
    fn get_port(self: &Self) -> u16;
    // get the scheme of the request
    fn get_scheme(self: &Self) -> &String;
    // get the path of the request
    fn get_path(self: &Self) -> &String;
    // get the method of the request
    fn get_method(self: &Self) -> &Method;
    // get the body of the request
    fn get_body(self: &Self) -> &Vec<u8>;
    
    // get the model validation result of the request
    fn get_model_validation_result(self: &Self) -> Option<ViewModelResult<Rc<dyn Any>>>;
    // set the model validation result of the request
    fn set_model_validation_result(self: &Self, v: Option<ViewModelResult<Rc<dyn Any>>>);

    // get the body model of the request
    fn get_connection_context(self: &Self) -> Rc<dyn IConnectionContext>;

    // get the route data of the request
    fn get_route_data(self: &Self) -> RouteData;
    // get the route data of the request as a mutable reference
    fn mut_route_data(self: &Self) -> &RefCell<RouteData>;

    // get the query string of the request
    fn get_query(self: &Self) -> &QueryString;

    // get the controller action of the request if it exists
    fn get_controller_action_optional(self: &Self) -> Option<Rc<dyn IControllerAction>>;
    // get the controller action of the request
    fn get_controller_action(self: &Self) -> Rc<dyn IControllerAction>;
    // set the controller action of the request
    fn set_controller_action(self: &Self, controller: Option<Rc<dyn IControllerAction>>);

    // get the authorization claims of the request
    fn get_auth_claims(self: &Self) -> Vec<Rc<dyn IAuthClaim>>;

    // get the context data of the request
    fn get_str(self: &Self, key: &str) -> String;
    // get the context data of the request
    fn get_string(self: &Self, key: String) -> String;
    // insert a value into the context data of the request
    fn insert_str(self: &mut Self, key: &str, value: String) -> String;
    // insert a value into the context data of the request
    fn insert_string(self: &mut Self, key: String, value: String) -> String;

    // get the headers of the request
    fn get_headers(self: &Self) -> &HeaderMap;
    // get the cookies of the request
    fn get_cookies_parsed(self: &Self) -> Option<HashMap<String, String>>;
}