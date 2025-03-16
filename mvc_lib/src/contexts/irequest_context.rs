use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use http::{Method, HeaderMap};

use crate::core::query_string::QueryString;
use crate::http::http_body_content::{IBodyContent, ContentType};
use crate::http::ihttp_body_stream_format::IHttpBodyStreamFormat;
use crate::model_binder::imodel::AnyIModel;
use crate::model_binder::model_validation_result::ModelValidationResult;
use crate::routing::route_data::RouteData;
use crate::services::authorization_service::IAuthClaim;
use crate::controller_actions::controller_action::IControllerAction;
use crate::services::service_collection::IServiceCollection;

use super::ihttpconnection_context::IHttpConnectionContext;


// this trait represents a HTTP request and its context.
#[mockall::automock]
pub trait IRequestContext {
    // get the HTTP version of the request
    fn get_http_version(&self) -> http::version::Version;
    // get the method of the request
    fn get_url(&self) -> url::Url;
    // get the type name of the request Rust type
    fn get_type_name(&self) -> &'static str;
    // get the host name? of the request
    fn get_host_name(&self) -> &String;
    // get the port of the request
    fn get_port(&self) -> u16;
    // get the scheme of the request
    fn get_scheme(&self) -> &String;
    // get the path of the request
    fn get_path(&self) -> &String;
    // get the method of the request
    fn get_method(&self) -> &Method;

    // get the body of the request if it exists and is not empty.
    // fn get_body_raw(&self) -> Option<Box<Vec<u8>>>;
    // get the body of the request if it exists and is not empty, and was decoded.
    // fn get_body_content(&self) -> Option<Rc<dyn IBodyContent>>;
    
    // get the model validation result of the request
    fn get_model_validation_result(&self) -> Option<ModelValidationResult<AnyIModel>>;
    // set the model validation result of the request
    fn set_model_validation_result(&self, v: Option<ModelValidationResult<AnyIModel>>);

    // get the body model of the request
    fn get_connection_context(&self) -> &dyn IHttpConnectionContext;

    // get the route data of the request
    fn get_route_data(&self) -> RouteData;
    // get the route data of the request as a mutable reference
    fn mut_route_data(&self) -> &RefCell<RouteData>;

    // get the query string of the request
    fn get_query(&self) -> &QueryString;

    // get the controller action of the request if it exists
    fn get_controller_action_optional(&self) -> Option<Rc<dyn IControllerAction>>;
    // get the controller action of the request
    fn get_controller_action(&self) -> Rc<dyn IControllerAction>;
    // set the controller action of the request
    fn set_controller_action(&self, controller: Option<Rc<dyn IControllerAction>>);

    // get the authorization claims of the request
    fn get_auth_claims(&self) -> Vec<Rc<dyn IAuthClaim>>;

    // get the context data of the request
    fn get_str(&self, key: &str) -> String;
    // get the context data of the request
    fn get_string(&self, key: String) -> String;
    // try to get the context data of the request
    fn try_get_string(&self, key: String) -> Option<String>;
    // insert a value into the context data of the request
    fn insert_str(self: &mut Self, key: &str, value: String) -> String;
    // insert a value into the context data of the request
    fn insert_string(self: &mut Self, key: String, value: String) -> String;

    // get the headers of the request
    fn get_headers(&self) -> &HeaderMap;
    // get the cookies of the request
    fn get_cookies_parsed(&self) -> Option<HashMap<String, String>>;

    // use a decoder for the request body.
    fn use_decoder(&self, decoder: Rc<dyn IHttpBodyStreamFormat>);

    // decode and bind the body of the request
    fn decode_and_bind_body(&self, services: &dyn IServiceCollection) -> Option<Rc<dyn IBodyContent>>;

    // get the content type of the request
    fn get_content_type(&self) -> Option<ContentType>;
    // get the content length of the request
    fn get_content_length(&self) -> Option<usize>;

    // get uuid of the request
    fn get_uuid(&self) -> &uuid::Uuid;
}