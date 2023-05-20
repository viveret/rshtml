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

// this is a trait for a class that can process an HTTP request and return an HTTP response.
// the way requests are processed is by using a pipeline of middleware services.
pub trait IHttpRequestPipeline {
    fn process_request(self: &Self, http_header: String, headers: Vec<String>, request_bytes: Box<Vec<u8>>, connection_context: Rc<dyn IConnectionContext>, services: &dyn IServiceCollection) -> Result<Vec<u8>, Box<dyn Error>>;
}

// this is a struct that implements IHttpRequestPipeline.
pub struct HttpRequestPipeline {
    #[allow(dead_code)]
    options: Rc<dyn IHttpOptions>,
}

impl HttpRequestPipeline {
    pub fn new(
        options: Rc<dyn IHttpOptions>) -> Self {
        Self { options: options }
    }

    // creates HTTP request pipeline as a service.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IHttpOptions>(services)
        )) as Rc<dyn IHttpRequestPipeline>)]
    }

    // parses an HTTP request.
    fn parse_request(self: &Self, http_header: String, headers: Vec<String>, request_bytes: Box<Vec<u8>>, connection_context: Rc<dyn IConnectionContext>) -> Rc<dyn IRequestContext> {
        return RequestContext::parse(http_header, headers, request_bytes, connection_context);
    }

    /// Build a response for a request
    /// 
    /// # Arguments
    /// * `request_ctx` - The request context
    /// * `services` - The service collection
    /// 
    /// # Returns
    /// The response context
    fn build_response(self: &Self, request_ctx: Rc<dyn IRequestContext>, services: &dyn IServiceCollection) -> Result<Rc<ResponseContext>, Box<dyn Error>> {
        // Get the middleware services
        let middleware = ServiceCollectionExtensions::get_required_multiple::<dyn IRequestMiddlewareService>(services);
        // Throw an error if there are no middleware services
        if middleware.len() == 0 {
            return Err(Box::new(RequestError(format!("No middleware configured"))));
        }

        // Create an iterator over the middleware
        let mut it = middleware.iter().cloned().peekable();
        // Loop through each middleware service
        loop {
            // Get the next middleware service
            let request_handler_option = it.next();
            // Check if there is a next middleware service
            if let Some(request_handler) = request_handler_option {
                // Get the next middleware service
                let next_request_handler = it.peek();
                // Set the next middleware service
                request_handler.set_next(next_request_handler.cloned());
            } else {
                // There are no more middleware services
                break;
            }
        }

        // Create a response context
        let response_context = Rc::new(ResponseContext::new(http::version::Version::HTTP_11, http::StatusCode::NOT_FOUND));
        // Get the first middleware service
        let first = middleware.first().unwrap();
        // Handle the request
        first.handle_request(request_ctx.clone(), response_context.clone(), services)?;
        // Return the response context
        Ok(response_context)
    }

    /// Writes an HTTP response to the given destination buffer.
    ///
    /// This function is used to write an HTTP response, including the status line,
    /// headers, and body, to the given destination buffer. This function is used by the
    /// `Response::write_to` function to write the response to the stream.
    ///
    /// # Parameters
    ///
    /// * `self` - The HTTP response object.
    ///
    /// * `response_bytes` - The destination buffer to which the response will be written.
    ///
    /// * `response_ctx` - The context of the response.
    ///
    /// * `_request_ctx` - The context of the request that elicited this response.
    ///
    /// # Returns
    /// This function does not return a value.
    fn write_response(self: &Self, response_bytes: &mut Vec<u8>, response_ctx: Rc<ResponseContext>, _request_ctx: Rc<dyn IRequestContext>) {
        // write the http version, status code, and status message
        response_bytes.extend_from_slice(b"HTTP/1.1 ");
        response_bytes.extend_from_slice(response_ctx.as_ref().status_code.borrow().as_str().as_bytes());
        response_bytes.extend_from_slice(b" ");
        response_bytes.extend_from_slice(response_ctx.as_ref().status_message().as_str().as_bytes());
        response_bytes.extend_from_slice(b"\r\n");

        // write the headers
        for header in response_ctx.headers.borrow().iter() {
            // write the header name
            response_bytes.extend_from_slice(header.0.as_str().as_bytes());
            response_bytes.extend_from_slice(b": ");
            // write the header value
            response_bytes.extend_from_slice(&header.1.as_bytes());
            // write the header new line
            response_bytes.extend_from_slice(b"\r\n");
        }
    
        // write another new line
        response_bytes.extend_from_slice(b"\r\n");

        // append the response context body to the response bytes to be sent over the wire
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