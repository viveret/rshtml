use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;

use crate::errors::RequestError;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;
use crate::services::request_handler_service::IRequestHandlerService;

use crate::options::http_options::IHttpOptions;

use crate::contexts::request_context::RequestContext;
use crate::contexts::response_context::ResponseContext;


pub trait IHttpRequestPipeline {
    fn process_request(self: &Self, http_header: String, headers: Vec<String>, request_bytes: Box<Vec<u8>>, services: &dyn IServiceCollection) -> Result<Vec<u8>, Box<dyn Error>>;
}

pub struct HttpRequestPipeline {
    request_handlers: Vec<Rc<dyn IRequestHandlerService>>,
    #[allow(dead_code)]
    options: Rc<dyn IHttpOptions>,
}

impl HttpRequestPipeline {
    pub fn new(
        request_handlers: Vec<Rc<dyn IRequestHandlerService>>,
        options: Rc<dyn IHttpOptions>) -> Self {
        Self { request_handlers: request_handlers, options: options }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_multiple::<dyn IRequestHandlerService>(services),
            ServiceCollectionExtensions::get_required_single::<dyn IHttpOptions>(services)
        )) as Rc<dyn IHttpRequestPipeline>)]
    }

    fn parse_request(self: &Self, http_header: String, headers: Vec<String>, request_bytes: Box<Vec<u8>>) -> Rc<RequestContext> {
        return RequestContext::parse(http_header, headers, request_bytes);
    }

    fn build_response(self: &Self, request_ctx: Rc<RequestContext>, services: &dyn IServiceCollection) -> Result<Rc<RefCell<ResponseContext>>, Box<dyn Error>> {
        for request_handler in self.request_handlers.iter() {
            let response = request_handler.handle_request(request_ctx.clone(), services)?;
            match response {
                Some(response_ctx) => {
                    return Ok(response_ctx)
                },
                None => { }
            }
        }
        return Err(Box::new(RequestError("Could not handle request".to_string())))
    }

    fn write_response(self: &Self, response_bytes: &mut Vec<u8>, response_ctx: Rc<RefCell<ResponseContext>>, _request_ctx: Rc<RequestContext>) {
        response_bytes.extend_from_slice(b"HTTP/1.1 ");
        response_bytes.extend_from_slice(response_ctx.as_ref().borrow().status_code.as_str().as_bytes());
        response_bytes.extend_from_slice(b" ");
        response_bytes.extend_from_slice(response_ctx.as_ref().borrow().status_message().as_str().as_bytes());
        response_bytes.extend_from_slice(b"\r\n");

        for header in response_ctx.borrow().headers.iter() {
            response_bytes.extend_from_slice(header.name.as_bytes());
            response_bytes.extend_from_slice(b": ");
            response_bytes.extend_from_slice(&header.value);
            response_bytes.extend_from_slice(b"\r\n");
        }
    
        response_bytes.extend_from_slice(b"\r\n");

        response_bytes.extend_from_slice(&response_ctx.as_ref().borrow().body);
    }
}

impl IHttpRequestPipeline for HttpRequestPipeline {
    fn process_request(self: &Self, http_header: String, headers: Vec<String>, request_bytes: Box<Vec<u8>>, services: &dyn IServiceCollection) -> Result<Vec<u8>, Box<dyn Error>> {
        let request_ctx = self.parse_request(http_header, headers, request_bytes);
        let response_ctx = self.build_response(request_ctx.clone(), services)?;
        println!("{} -> {}", request_ctx.path, response_ctx.borrow().status_code);
        
        let mut response_bytes = Vec::new();
        self.write_response(&mut response_bytes, response_ctx, request_ctx);

        return Ok(response_bytes);
    }
}