
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use regex::Regex;

use http::{ HeaderName, HeaderValue, HeaderMap };

use crate::contexts::connection_context::IConnectionContext;

use crate::controllers::controller_action::IControllerAction;

use crate::routing::route_data::RouteData;


pub struct RequestContext {
    pub connection_context: Rc<dyn IConnectionContext>,
    pub http_version: http::version::Version,
    pub method: Box<String>,
    pub path: Box<String>,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
    pub route_data: RefCell<RouteData>,
    pub context_data: RefCell<HashMap<String, String>>,
    pub controller_action: RefCell<Option<Rc<dyn IControllerAction>>>,
}

impl RequestContext {
    pub fn new(
        connection_context: Rc<dyn IConnectionContext>,
        http_version: http::version::Version,
        method: Box<String>, path: Box<String>,
        request_headers: HeaderMap
    ) -> Self {
        Self {
            connection_context: connection_context,
            http_version: http_version,
            method: method,
            path: path,
            headers: request_headers,
            body: vec![],
            route_data: RefCell::new(RouteData::new()),
            context_data: RefCell::new(HashMap::new()),
            controller_action: RefCell::new(None),
        }
    }

    pub fn parse(http_header: String, headers: Vec<String>, _request_bytes: Box<Vec<u8>>, connection_context: Rc<dyn IConnectionContext>) -> Rc<RequestContext> {
        let re_method: Regex = Regex::new(r"^(GET|POST|PUT) ").unwrap();
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

        let path = &http_header[method_str.len() + 1 .. http_header.len() - version_str.len() - 1].trim();

        Rc::new(Self::new(
            connection_context,
            version,
            Box::new(method_str.to_string()),
            Box::new(path.to_string()),
            HeaderMap::from_iter(headers.iter().map(|x| {
                let mut name = re_header.find(x).expect(&format!("Invalid header format: {}", x)).as_str();
                let value = x[name.len()..].to_string();
                name = &name[..name.len()-2];
                (HeaderName::from_bytes(name.as_bytes()).unwrap(), HeaderValue::from_bytes(value.as_bytes()).unwrap())
                // HttpHeader::from_httparse_header(name, &value.as_bytes().to_vec())
            }))
        ))
    }

    pub fn get_string(self: &Self, key: String) -> String {
        match self.route_data.borrow().map.get(&key) {
            Some(v) => v.clone(),
            None => {
                self.context_data.borrow().get(&key).unwrap_or(&String::new()).to_string()
            }
        }
    }

    pub fn get_str(self: &Self, key: &str) -> String {
        self.get_string(key.to_string())
    }

    pub fn insert_string(self: &mut Self, key: String, value: String) -> String {
        self.context_data.borrow_mut().insert(key, value.clone());
        value
    }

    pub fn insert_str(self: &mut Self, key: &str, value: String) -> String {
        self.insert_string(key.to_string(), value)
    }

}