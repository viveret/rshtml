use std::any::Any;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use flate2::bufread::GzDecoder;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::{ResponseContext, IResponseContext};
use crate::core::itcp_stream_wrapper::ITcpStreamWrapper;
use crate::core::type_info::TypeInfo;
use crate::services::request_middleware_service::{IRequestMiddlewareService, MiddlewareResult};
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollection, ServiceCollectionExtensions};

use super::http_body_content::{ContentType, IBodyContent};
use super::http_body_format_service::IHttpBodyFormatService;

// this middleware is used to decode the request body.
pub struct RequestDecoderMiddleware {
    // the next middleware in the pipeline
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>,
    // the decoder service used to decode the request body.
    body_content_decoder_service: Rc<dyn IHttpBodyFormatService>
}

impl RequestDecoderMiddleware {
    pub fn new(body_content_decoder_service: Rc<dyn IHttpBodyFormatService>) -> Self {
        Self {
            next: RefCell::new(None),
            body_content_decoder_service: body_content_decoder_service,
        }
    }

    // this is the function that will be called by the service collection to create a new instance of the middleware
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IHttpBodyFormatService>(services)
        )) as Rc<dyn IRequestMiddlewareService>)]
    }

    // this is called by the application to add the middleware to the service collection
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRequestMiddlewareService>(), Self::new_service, ServiceScope::Singleton));
    }

    // decodes the body of the request. this is used by the parse function.
    // , 
    fn decode_body(self: &Self, found_content_length: Option<usize>, found_content_type: Option<ContentType>, request_bytes: Box<Vec<u8>>) -> Option<Rc<dyn IBodyContent>> {
        if found_content_length.unwrap_or(0) > 0 {
            if let Some(content_type) = found_content_type {
                let body_content = self.body_content_decoder_service.decode_from_raw(
                    content_type,
                    found_content_length.unwrap(),
                    request_bytes.as_ref()
                );
                Some(body_content)
            } else {
                println!("no body content type or length found");
                None
            }
        } else {
            None
        }
    }
}

impl IRequestMiddlewareService for RequestDecoderMiddleware {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn Error>> {
        // get content type from request
        let content_type = request_context.get_headers().get("Content-Type").unwrap();
        let content_type_str = content_type.to_str().unwrap();
        let content_type = ContentType::parse(content_type_str);

        // replace source request stream with decoder stream if content encoding is gzip
        if let Some(content_encoding) = request_context.get_headers().get("Content-Encoding") {
            let content_encoding_str = content_encoding.to_str().unwrap();
            if content_encoding_str == "gzip" {
                let tcp_ctx = request_context.get_connection_context().get_tcp_context();
                tcp_ctx.add_decoder(Box::new(GzipBodyStream::new()));
            }
        }
        
        if let Some(next) = self.next.borrow().as_ref() {
            let next_response = next.handle_request(response_context, request_context, services)?;

            match next_response {
                MiddlewareResult::OkBreak => {
                    return Ok(MiddlewareResult::OkBreak); // short circuit middleware
                },
                _ => { }
            }
        }

        Ok(MiddlewareResult::OkContinue)
    }
}

pub struct GzipBodyStream {

}

impl GzipBodyStream {
    // source: Rc<dyn ITcpStreamWrapper>
    pub fn new() -> Self {
        // let mut d = flate2::Decompress::decompress();
        // let mut s = String::new();
        // d.read_to_string(&mut s).unwrap();
        // println!("{}", s);
        Self {

        }
    }
}

impl ITcpStreamWrapper for GzipBodyStream {
    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }

    fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn read_line(&mut self) -> Result<String, std::string::FromUtf8Error> {
        todo!()
    }

    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn write_line(&mut self, b: &String) -> std::io::Result<usize> {
        todo!()
    }

    fn remote_addr(&self) -> std::net::SocketAddr {
        todo!()
    }
}