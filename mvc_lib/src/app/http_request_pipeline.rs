use std::any::Any;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;
use crate::errors::RequestError;

use crate::contexts::connection_context::IConnectionContext;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;
use crate::services::request_middleware_service::IRequestMiddlewareService;

use crate::options::http_options::IHttpOptions;

use crate::contexts::request_context::RequestContext;
use crate::contexts::response_context::ResponseContext;


pub trait IHttpRequestPipeline {
    fn process_request(self: &Self, http_header: String, headers: Vec<String>, request_bytes: Box<Vec<u8>>, connection_context: Rc<dyn IConnectionContext>, services: &dyn IServiceCollection) -> Result<Vec<u8>, Box<dyn Error>>;
}

pub struct HttpRequestPipeline {
    #[allow(dead_code)]
    options: Rc<dyn IHttpOptions>,
}

impl HttpRequestPipeline {
    pub fn new(
        options: Rc<dyn IHttpOptions>) -> Self {
        Self { options: options }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IHttpOptions>(services)
        )) as Rc<dyn IHttpRequestPipeline>)]
    }

    fn parse_request(self: &Self, http_header: String, headers: Vec<String>, request_bytes: Box<Vec<u8>>, connection_context: Rc<dyn IConnectionContext>) -> Rc<dyn IRequestContext> {
        return RequestContext::parse(http_header, headers, request_bytes, connection_context);
    }

    fn build_response(self: &Self, request_ctx: Rc<dyn IRequestContext>, services: &dyn IServiceCollection) -> Result<Rc<ResponseContext>, Box<dyn Error>> {
        let middleware = ServiceCollectionExtensions::get_required_multiple::<dyn IRequestMiddlewareService>(services);
        if middleware.len() == 0 {
            return Err(Box::new(RequestError(format!("No middleware configured"))));
        }

        let mut it = middleware.iter().cloned().peekable();
        loop {
            let request_handler_option = it.next();
            if let Some(request_handler) = request_handler_option {
                let next_request_handler = it.peek();
                request_handler.set_next(next_request_handler.cloned());
            } else {
                break;
            }
        }

        let response_context = Rc::new(ResponseContext::new(http::version::Version::HTTP_11, http::StatusCode::NOT_FOUND));
        let first = middleware.first().unwrap();
        first.handle_request(request_ctx.clone(), response_context.clone(), services)?;
        Ok(response_context)
    }

    fn write_response(self: &Self, response_bytes: &mut Vec<u8>, response_ctx: Rc<ResponseContext>, _request_ctx: Rc<dyn IRequestContext>) {
        response_bytes.extend_from_slice(b"HTTP/1.1 ");
        response_bytes.extend_from_slice(response_ctx.as_ref().status_code.borrow().as_str().as_bytes());
        response_bytes.extend_from_slice(b" ");
        response_bytes.extend_from_slice(response_ctx.as_ref().status_message().as_str().as_bytes());
        response_bytes.extend_from_slice(b"\r\n");

        for header in response_ctx.headers.borrow().iter() {
            response_bytes.extend_from_slice(header.0.as_str().as_bytes());
            response_bytes.extend_from_slice(b": ");
            response_bytes.extend_from_slice(&header.1.as_bytes());
            response_bytes.extend_from_slice(b"\r\n");
        }
    
        response_bytes.extend_from_slice(b"\r\n");

        response_bytes.extend_from_slice(&response_ctx.body.borrow());
    }
}

impl IHttpRequestPipeline for HttpRequestPipeline {
    fn process_request(self: &Self, http_header: String, headers: Vec<String>, request_bytes: Box<Vec<u8>>, connection_context: Rc<dyn IConnectionContext>, services: &dyn IServiceCollection) -> Result<Vec<u8>, Box<dyn Error>> {
        let request_ctx = self.parse_request(http_header, headers, request_bytes, connection_context);
        let response_ctx = self.build_response(request_ctx.clone(), services)?;
        
        let mut response_bytes = Vec::new();
        self.write_response(&mut response_bytes, response_ctx, request_ctx);

        return Ok(response_bytes);
    }
}