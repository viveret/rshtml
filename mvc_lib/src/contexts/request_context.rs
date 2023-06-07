
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use regex::Regex;

use http::{ HeaderName, HeaderValue, HeaderMap, Method };
use syn::token::Ref;

use crate::controller_actions::controller_action::IControllerAction;

use crate::core::itcp_stream_wrapper::ITcpStreamWrapper;
use crate::core::query_string::QueryString;

use crate::core::type_info::TypeInfo;
use crate::http::http_body_content::ContentType;
use crate::http::http_body_content::IBodyContent;
use crate::http::http_body_content::StreamBodyContent;
use crate::http::ihttp_body_stream_format::IHttpBodyStreamFormat;
use crate::model_binder::imodel::AnyIModel;
use crate::model_binder::imodel::IModel;
use crate::model_binder::imodelbinder_service::IModelBinderService;
use crate::routing::route_data::RouteData;

use crate::services::authorization_service::IAuthClaim;
use crate::model_binder::model_validation_result::ModelValidationResult;
use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;

use super::connection_context::IHttpConnectionContext;
use super::irequest_context::IRequestContext;

// this struct represents a HTTP request and its context.
// it is created by the server and passed to middleware and the controller action.
// it is also passed to the view renderer and view.
pub struct RequestContext<'a> {
    // the unique identifier of the request
    uuid: uuid::Uuid,
    // the HTTP connection context of the request
    connection_context: &'a dyn IHttpConnectionContext,
    // the HTTP version of the request
    http_version: http::version::Version,
    // the scheme of the request
    scheme: Box<String>,
    // the method of the request
    method: Method,
    // the port of the the connection the request was received on
    port: u16,
    // the host name of the request (should be the same as the current server host name)
    host_name: Box<String>,
    // the path of the request
    path: Box<String>,
    // the query string of the request
    query: QueryString,
    // the query string of the request as a string
    query_string: Box<String>,
    // the headers of the request
    headers: HeaderMap,
    // decoders used to decode the request body
    decoders: RefCell<Vec<Rc<dyn IHttpBodyStreamFormat>>>,
    // the decoded body content of the request
    body_content: RefCell<Option<Rc<dyn IBodyContent>>>,
    // the body stream of the request
    body_stream: RefCell<Option<Rc<dyn ITcpStreamWrapper>>>,
    // the model validation result of the request
    model_validation_result: RefCell<Option<ModelValidationResult<AnyIModel>>>,
    // the body model of the request
    body_model: RefCell<Option<AnyIModel>>,
    // the route data of the request
    route_data: RefCell<RouteData>,
    // the authorization claims of the request
    auth_claims: RefCell<Vec<Rc<dyn IAuthClaim>>>,
    // the context data of the request
    context_data: RefCell<HashMap<String, String>>,
    // the controller action for the request
    controller_action: RefCell<Option<Rc<dyn IControllerAction>>>,
}

impl <'a> RequestContext<'a> {
    // creates a new request context.
    // connection_context: the HTTP connection context of the request
    // http_version: the HTTP version of the request
    // scheme: the scheme of the request
    // method_str: the method of the request as a string
    // method: the method of the request
    // path: the path of the request
    // query_string: the query string of the request
    // request_headers: the headers of the request
    // returns: the new request context.
    pub fn new(
        connection_context: &'a dyn IHttpConnectionContext,
        http_version: http::version::Version,
        scheme: Option<Box<String>>,
        method_str: Option<Box<String>>,
        method: Option<Method>,
        host_name: Box<String>,
        port: u16,
        path: Box<String>,
        query_string: Box<String>,
        request_headers: HeaderMap,
    ) -> Self {
        Self {
            uuid: uuid::Uuid::new_v4(),
            connection_context: connection_context,
            http_version: http_version,
            scheme: scheme.unwrap_or(Box::new("http".to_string())),
            method: method.unwrap_or(Method::from_str(method_str.unwrap().as_ref().as_str()).unwrap()),
            host_name: host_name,
            port: port,
            path: path,
            query: QueryString::parse(query_string.as_ref()),
            query_string: query_string,
            headers: request_headers,
            body_content: RefCell::new(None),
            body_stream: RefCell::new(None),
            model_validation_result: RefCell::new(None),
            body_model: RefCell::new(None),
            route_data: RefCell::new(RouteData::new()),
            auth_claims: RefCell::new(Vec::new()),
            context_data: RefCell::new(HashMap::new()),
            controller_action: RefCell::new(None),
            decoders: RefCell::new(Vec::new()),
        }
    }

    // parses a HTTP request into a request context.
    // http_header: the HTTP header of the request
    // headers: the headers of the request
    // request_bytes: the body of the request
    // connection_context: the HTTP connection context of the request
    // returns: the new request context.
    pub fn parse(connection_context: &'a dyn IHttpConnectionContext) -> Result<RequestContext<'a>, std::io::Error> {
        let mut request_headers: Vec<String> = vec![];
        loop {
            let read_result = connection_context.read_line();
            match read_result {
                Ok(line) => {
                    if line.trim() == "" {
                        break;
                    }
        
                    request_headers.push(line);
                },
                Err(err) => {
                    println!("Could not read http headers: {}", err);
                    break;
                },
            }
        }

        if request_headers.len() == 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Could not read http headers: no headers found."));
        }

        let http_header: String = request_headers.remove(0);

        let method_str = &http_header[..http_header.find(' ').unwrap()];
        let version_str = &http_header[http_header.rfind(' ').unwrap() + 1..];

        let re_method_valid: Regex = Regex::new(r"^GET|HEAD|POST|PUT$").unwrap();
        let re_header: Regex = Regex::new(r"^([a-zA-Z0-9 _-]+): ").unwrap();

        // println!("Received request: {}", http_header);

        if !re_method_valid.is_match(&http_header) {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid HTTP method: {}", method_str)));
        }

        let version = match version_str {
            "HTTP/0.9" => http::version::Version::HTTP_09,
            "HTTP/1.0" => http::version::Version::HTTP_10,
            "HTTP/1.1" => http::version::Version::HTTP_11,
            _ => {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid HTTP version {}", version_str)));
            }
        };

        let mut errors = vec![];
        let headers = HeaderMap::from_iter(request_headers.iter().map(|x| {
            let mut name = re_header.find(x).expect(&format!("Invalid header format: {}", x)).as_str();
            let value = x[name.len()..].to_string();
            name = &name[..name.len()-2];
            let value_str = value.trim();

            match HeaderValue::from_str(value_str) {
                Ok(header_val) => {
                    Some((HeaderName::from_bytes(name.as_bytes()).unwrap(), header_val))
                },
                Err(err) => {
                    errors.push(err);
                    None
                },
            }
        }).filter(Option::is_some).map(|x| x.unwrap()));

        if errors.len() > 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Could not parse headers: {:?}", errors)));
        }

        let host_header_value = headers.get("Host").unwrap();
        let host_header_string = format!("http://{}", host_header_value.to_str().unwrap());
        let host_header_url = url::Url::parse(host_header_string.as_str()).unwrap();

        let request_url = host_header_url;
        let path_and_query = request_url.join(&http_header[method_str.len() + 1 .. http_header.len() - version_str.len() - 1].trim()).unwrap();

        let path = path_and_query.path();
        let query = path_and_query.query().unwrap_or("");

        Ok(Self::new(
            connection_context,
            version,
            Some(Box::new(path_and_query.scheme().to_string())),
            Some(Box::new(method_str.to_string())),
            None,
            Box::new(request_url.host().unwrap().to_string()),
            request_url.port().unwrap(),
            Box::new(path.to_string()),
            Box::new(query.to_string()),
            headers,
        ))
    }
    
}

impl<'a> IRequestContext for RequestContext<'a> {
    fn get_type_name(self: &Self) -> &'static str {
        nameof::name_of_type!(RequestContext)
    }

    fn get_host_name(self: &Self) -> &String {
        self.host_name.as_ref()
    }

    fn get_url(self: &Self) -> url::Url {
        let port = self.get_port();
        let port_str = if port == 80 || port == 443 { "".to_string() } else { format!(":{}", port) };

        let query = self.get_query().to_string();
        let query_str = if query.is_empty() { "".to_string() } else { format!("?{}", query) };

        let url_str = &format!(
            "{}://{}{}{}{}",
            self.get_scheme(),
            self.get_host_name(),
            port_str,
            self.get_path(),
            query_str
        );
        url::Url::parse(url_str).unwrap()
    }

    fn get_scheme(self: &Self) -> &String {
        self.scheme.as_ref()
    }

    fn get_port(self: &Self) -> u16 {
        self.port
    }

    fn get_path(self: &Self) -> &String {
        self.path.as_ref()
    }

    fn get_query(self: &Self) -> &QueryString {
        &self.query
    }

    fn get_connection_context(self: &Self) -> &dyn IHttpConnectionContext {
        self.connection_context
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

    fn get_body_content(self: &Self) -> Option<Rc<dyn IBodyContent>> {
        self.body_content.borrow().clone()
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
                    })
                    .collect()
                )
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

    fn get_model_validation_result(self: &Self) -> Option<ModelValidationResult<AnyIModel>> {
        self.model_validation_result.borrow().clone()
    }

    fn set_model_validation_result(self: &Self, v: Option<ModelValidationResult<AnyIModel>>) {
        if let Some(ref v2) = v {
            self.model_validation_result.replace(Some(v2.clone()));
            match v2 {
                ModelValidationResult::OkNone => {
                },
                ModelValidationResult::Ok(model) => {
                    self.body_model.replace(Some(model.clone()));
                },
                ModelValidationResult::ModelError(..) => {
                },
                ModelValidationResult::PropertyError(..) => {
                },
                ModelValidationResult::OtherError(..) => {
                },
            }
        }
    }
    
    // this function is used to get the content length from the headers.
    fn get_content_length(self: &Self) -> Option<usize> {
        if let Some(content_length_header_val) = self.headers.get("Content-Length") {
            match content_length_header_val.to_str() {
                Ok(content_length_str) => {
                    match content_length_str.parse::<usize>() {
                        Ok(content_length) => {
                            println!("found_content_length: {}", content_length);
                            Some(content_length)
                        },
                        Err(e) => {
                            panic!("Invalid content-length header value: {}", e);
                        },
                    }
                },
                Err(e) => {
                    panic!("Invalid content-length header value: {}", e);
                },
            }
        } else {
            None
        }
    }
    
    // this function is used to get the content type from the headers.
    fn get_content_type(self: &Self) -> Option<ContentType> {
        // get_headers().get("Content-Type").unwrap()
        if let Some(content_type_header_val) = self.headers.get("Content-Type") {
            match content_type_header_val.to_str() {
                Ok(content_type_str) => {
                    println!("found_content_type: {}", content_type_str);
                    Some(ContentType::parse(&content_type_str.to_string()))
                },
                Err(e) => {
                    panic!("Invalid content-type header value: {}", e);
                },
            }
        } else {
            None
        }
    }

    fn use_decoder(self: &Self, decoder: Rc<dyn IHttpBodyStreamFormat>) {
        self.decoders.borrow_mut().push(decoder);
    }

    fn decode_and_bind_body(self: &Self, services: &dyn IServiceCollection) -> Option<Rc<dyn IBodyContent>> {
        // // get content type from request

        // if let Some(content_type) = request_context.get_content_type() {
        //     if let Some(action) = request_context.get_controller_action() {
        //     }
        //     // replace source request stream with decoder stream if content encoding is gzip
        //     // if let Some(content_encoding) = request_context.get_headers().get("Content-Encoding") {
        //     //     let content_encoding_str = content_encoding.to_str().unwrap();
        //     //     if let Some(formatter) = self.body_content_decoder_service.resolve(content_type.clone()) {
        //     //         if formatter.matches_content_type(&content_type) {
        //     //             request_context.decode_body(formatter);
        //     //         }
        //     //     }
        //     // }
        // }

        if let Some(action) = self.get_controller_action_optional() {
            if let Some(model_type) = action.get_model_type() {
                // only decode request body if the method is post and if the content type is defined
                if self.method == http::method::Method::POST {
                    if let Some(content_type) = self.get_content_type().as_ref() {
                        println!("decode_and_bind_body: {} has model type {}", self.get_path(), model_type.to_string());
                        // check if model has been bound yet
                        if let Some(_) = self.body_model.borrow().as_ref() {
                            println!("decode_and_bind_body: {} already has model bound", self.get_path());
                            return None;
                        }
        
                        println!("decode_and_bind_body: {} binding model, none bound", self.get_path());
        
                        let model_binder_service: Rc<dyn IModelBinderService> = ServiceCollectionExtensions::get_required_single(services);
                        
                        let final_stream = self.connection_context.get_stream();
                        for decoder in self.decoders.borrow().iter() {
                            let decoded_stream = decoder.decode(final_stream.borrow().clone(), content_type);
                            final_stream.replace(decoded_stream);
                        }
                        self.body_stream.replace(Some(final_stream.borrow().clone()));
                        self.body_content.replace(Some(Rc::new(StreamBodyContent::new(
                            content_type.clone(),
                            self.get_content_length().unwrap(),
                            final_stream.borrow().clone()
                        ))));
                
                        // use model binder to try and read stream into model
                        let bind_result = model_binder_service.bind_model(self, model_type.as_ref());
                        match &bind_result {
                            ModelValidationResult::OtherError(e) => {
                                println!("{} decode_and_bind_body: bind_result: {}", self.uuid, e);
                            },
                            ModelValidationResult::Ok(model) => {
                                println!("{} decode_and_bind_body: bind_result: Ok({})", self.uuid, model.to_string());
                            },
                            _ => {
                                println!("{} decode_and_bind_body: bind_result: {}", self.uuid, bind_result.to_string());
                            },
                        }
                        self.set_model_validation_result(Some(bind_result));
                    } else {
                        panic!("decode_and_bind_body: {} does not have content type", self.get_path());
                    }
                } else {
                    // decode model from query string, since there is no request body.
                    let query_string = self.get_query();
                    if query_string.to_str().len() > 0 {
                        let model_binder_service: Rc<dyn IModelBinderService> = ServiceCollectionExtensions::get_required_single(services);
                        let bind_result = model_binder_service.bind_model(self, model_type.as_ref());
                        match &bind_result {
                            ModelValidationResult::OtherError(e) => {
                                println!("{} decode_and_bind_body: bind_result: {}", self.uuid, e);
                            },
                            ModelValidationResult::Ok(model) => {
                                println!("{} decode_and_bind_body: bind_result: Ok({})", self.uuid, model.to_string());
                            },
                            _ => {
                                println!("{} decode_and_bind_body: bind_result: {}", self.uuid, bind_result.to_string());
                            },
                        }
                        self.set_model_validation_result(Some(bind_result));
                    } else {
                        // panic!("decode_and_bind_body: {} does not have query string", self.get_path());
                    }
                }
            } else {
                // println!("decode_and_bind_body: {} does not have model type", self.get_path());
            }
        } else {
            // println!("decode_and_bind_body: {} does not have controller action", self.get_path());
        }

        None
    }

    fn get_uuid(self: &Self) -> &uuid::Uuid {
        &self.uuid
    }
}