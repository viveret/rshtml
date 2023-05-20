
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use regex::Regex;

use http::{ HeaderName, HeaderValue, HeaderMap, Method };

use crate::contexts::connection_context::IConnectionContext;

use crate::controller_actions::controller_action::IControllerAction;

use crate::core::query_string::QueryString;

use crate::routing::route_data::RouteData;

use crate::services::authorization_service::IAuthClaim;
use crate::model::view_model_result::ViewModelResult;

use super::irequest_context::IRequestContext;



pub struct RequestContext {
    connection_context: Rc<dyn IConnectionContext>,
    http_version: http::version::Version,
    scheme: Box<String>,
    method: Method,
    path: Box<String>,
    query: QueryString,
    query_string: Box<String>,
    headers: HeaderMap,
    body: Vec<u8>,
    model_validation_result: RefCell<Option<ViewModelResult<Rc<dyn Any>>>>,
    body_model: RefCell<Option<Box<dyn Any>>>,
    route_data: RefCell<RouteData>,
    auth_claims: RefCell<Vec<Rc<dyn IAuthClaim>>>,
    context_data: RefCell<HashMap<String, String>>,
    controller_action: RefCell<Option<Rc<dyn IControllerAction>>>,
}

impl RequestContext {
    pub fn new(
        connection_context: Rc<dyn IConnectionContext>,
        http_version: http::version::Version,
        scheme: Option<Box<String>>,
        method_str: Option<Box<String>>,
        method: Option<Method>,
        path: Box<String>,
        query_string: Box<String>,
        request_headers: HeaderMap
    ) -> Self {
        Self {
            connection_context: connection_context,
            http_version: http_version,
            scheme: scheme.unwrap_or(Box::new("http".to_string())),
            method: method.unwrap_or(Method::from_str(method_str.unwrap().as_ref().as_str()).unwrap()),
            path: path,
            query: QueryString::parse(query_string.as_ref()),
            query_string: query_string,
            headers: request_headers,
            body: vec![],
            model_validation_result: RefCell::new(None),
            body_model: RefCell::new(None),
            route_data: RefCell::new(RouteData::new()),
            auth_claims: RefCell::new(Vec::new()),
            context_data: RefCell::new(HashMap::new()),
            controller_action: RefCell::new(None),
        }
    }

    pub fn parse(http_header: String, headers: Vec<String>, _request_bytes: Box<Vec<u8>>, connection_context: Rc<dyn IConnectionContext>) -> Rc<dyn IRequestContext> {
        let re_method: Regex = Regex::new(r"^(GET|HEAD|POST|PUT) ").unwrap();
        let re_version: Regex = Regex::new(r" HTTP/(\d\.\d)$").unwrap();
        let re_header: Regex = Regex::new(r"^([a-zA-Z0-9 _-]+): ").unwrap();

        let method_str = re_method.find(&http_header).expect("HTTP method not found").as_str().trim();
        let version_str = re_version.find(&http_header).expect("HTTP version not found").as_str().trim();
        let version = match version_str {
            "HTTP/0.9" => http::version::Version::HTTP_09,
            "HTTP/1.0" => http::version::Version::HTTP_10,
            "HTTP/1.1" => http::version::Version::HTTP_11,
            _ => panic!("Invalid HTTP version {}", version_str)
        };

        let path_and_query = url::Url::parse("https://localhost").unwrap().join(&http_header[method_str.len() + 1 .. http_header.len() - version_str.len() - 1].trim()).unwrap();

        let path = path_and_query.path();
        let query = path_and_query.query().unwrap_or("");
        // println!("path: {}, query: {}, http_header: {}", path, query, http_header);

        Rc::new(Self::new(
            connection_context,
            version,
            Some(Box::new(path_and_query.scheme().to_string())),
            Some(Box::new(method_str.to_string())),
            None,
            Box::new(path.to_string()),
            Box::new(query.to_string()),
            HeaderMap::from_iter(headers.iter().map(|x| {
                let mut name = re_header.find(x).expect(&format!("Invalid header format: {}", x)).as_str();
                let value = x[name.len()..].to_string();
                name = &name[..name.len()-2];
                (HeaderName::from_bytes(name.as_bytes()).unwrap(), HeaderValue::from_bytes(value.as_bytes()).unwrap())
                // HttpHeader::from_httparse_header(name, &value.as_bytes().to_vec())
            }))
        ))
    }
}

impl IRequestContext for RequestContext {
    fn get_name(self: &Self) -> &'static str {
        nameof::name_of_type!(RequestContext)
    }

    fn get_url(self: &Self) -> url::Url {
        url::Url::parse(&format!("{}://{}{}?{}", self.get_scheme(), self.get_name(), self.get_path(), self.get_query().to_string())).unwrap()
    }

    fn get_scheme(self: &Self) -> &String {
        self.scheme.as_ref()
    }

    fn get_path(self: &Self) -> &String {
        self.path.as_ref()
    }

    fn get_query(self: &Self) -> &QueryString {
        &self.query
    }

    fn get_connection_context(self: &Self) -> Rc<dyn IConnectionContext> {
        self.connection_context.clone()
    }

    fn get_controller_action_optional(self: &Self) -> Option<Rc<dyn IControllerAction>> {
        self.controller_action.borrow().clone()
    }

    fn get_controller_action(self: &Self) -> Rc<dyn IControllerAction> {
        self.controller_action.borrow().as_ref().unwrap().clone()
    }

    fn set_controller_action(self: &Self, controller: Option<Rc<dyn IControllerAction>>) {
        self.controller_action.replace(controller);
    }

    fn get_string(self: &Self, key: String) -> String {
        match self.route_data.borrow().map.get(&key) {
            Some(v) => v.clone(),
            None => {
                self.context_data.borrow().get(&key).unwrap_or(&String::new()).to_string()
            }
        }
    }

    fn get_str(self: &Self, key: &str) -> String {
        self.get_string(key.to_string())
    }

    fn insert_string(self: &mut Self, key: String, value: String) -> String {
        self.context_data.borrow_mut().insert(key, value.clone());
        value
    }

    fn insert_str(self: &mut Self, key: &str, value: String) -> String {
        self.insert_string(key.to_string(), value)
    }

    fn get_method(self: &Self) -> &Method {
        &self.method
    }

    fn get_body(self: &Self) -> &Vec<u8> {
        &self.body
    }

    fn get_auth_claims(self: &Self) -> Vec<Rc<dyn IAuthClaim>> {
        self.auth_claims.borrow().clone()
    }
    
    fn get_cookies_parsed(self: &Self) -> Option<HashMap<String, String>> {
        let cookie_header = self.headers.get("cookie");
        match cookie_header {
            Some(header) => {
                Some(header.to_str().unwrap().split(';')
                    .map(|x| x.trim())
                    .map(|cookie| {
                    // println!("{}", cookie);
                    let split_kvp = cookie.split('=').map(|x| x.to_string()).collect::<Vec<String>>();
                    if split_kvp.len() == 2 {
                        (split_kvp.get(0).unwrap().clone(), split_kvp.get(1).unwrap().clone())
                    } else {
                        (split_kvp.get(0).unwrap().clone(), String::new())
                    }
                }).collect())
            },
            None => None,
        }
    }

    fn get_http_version(self: &Self) -> http::version::Version {
        self.http_version
    }

    fn get_headers(self: &Self) -> &HeaderMap {
        &self.headers
    }

    fn get_route_data(self: &Self) -> RouteData {
        self.route_data.borrow().clone()
    }

    fn mut_route_data(self: &Self) -> &RefCell<RouteData> {
        &self.route_data
    }

    fn get_model_validation_result(self: &Self) -> Option<ViewModelResult<Rc<dyn Any>>> {
        self.model_validation_result.borrow().clone()
    }

    fn set_model_validation_result(self: &Self, v: Option<ViewModelResult<Rc<dyn Any>>>) {
        if let Some(v2) = v {
            self.model_validation_result.replace(Some(v2.clone()));
            match v2 {
                ViewModelResult::OkNone => {
                },
                ViewModelResult::Ok(ref model) => {
                    self.body_model.replace(Some(Box::new(model.clone())));
                },
                ViewModelResult::ModelError(..) => {
                },
                ViewModelResult::PropertyError(..) => {
                },
            }
        }
    }
}