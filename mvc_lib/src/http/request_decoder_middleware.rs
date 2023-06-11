use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;


use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::IResponseContext;
use crate::core::itcp_stream_wrapper::ITcpStreamWrapper;
use crate::core::type_info::TypeInfo;
use crate::services::request_middleware_service::{IRequestMiddlewareService, MiddlewareResult};
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollection, ServiceCollectionExtensions};

use super::http_body_content::{ContentType};
use super::http_body_format_service::IHttpBodyFormatService;
use super::ihttp_body_stream_format::IHttpBodyStreamFormat;

// this middleware is used to decode the request body.
pub struct RequestDecoderMiddleware {
    // the next middleware in the pipeline
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>,
}

impl RequestDecoderMiddleware {
    pub fn new(_body_content_decoder_service: Rc<dyn IHttpBodyFormatService>) -> Self {
        Self {
            next: RefCell::new(None),
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
}

impl IRequestMiddlewareService for RequestDecoderMiddleware {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn Error>> {
        request_context.decode_and_bind_body(services);
        
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
    _inner_stream: Rc<dyn ITcpStreamWrapper>
}

impl GzipBodyStream {
    // source: Rc<dyn ITcpStreamWrapper>
    pub fn new(inner_stream: Rc<dyn ITcpStreamWrapper>) -> Self {
        // let mut d = flate2::Decompress::decompress();
        // let mut s = String::new();
        // d.read_to_string(&mut s).unwrap();
        // println!("{}", s);
        Self {
            _inner_stream: inner_stream,
        }
    }
}

impl ITcpStreamWrapper for GzipBodyStream {
    fn shutdown(&self, _how: std::net::Shutdown) -> std::io::Result<()> {
        todo!()
    }

    fn flush(&self) -> std::io::Result<()> {
        todo!()
    }

    fn read(&self, _b: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn read_line(&self) -> Result<String, std::string::FromUtf8Error> {
        todo!()
    }

    fn write(&self, _b: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn write_line(&self, _b: &String) -> std::io::Result<usize> {
        todo!()
    }

    fn remote_addr(&self) -> std::net::SocketAddr {
        todo!()
    }
}


pub struct GzipBodyStreamFormat {

}

impl GzipBodyStreamFormat {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl IHttpBodyStreamFormat for GzipBodyStreamFormat {
    fn matches_content_type(&self, content_type: &ContentType) -> bool {
        content_type.mime_type == "application/gzip" || content_type.mime_type == "gzip"
    }

    fn decode(&self, stream: Rc<dyn ITcpStreamWrapper>, _content_type: &ContentType) -> Rc<dyn ITcpStreamWrapper> {
        Rc::new(GzipBodyStream::new(stream))
    }

    fn encode(self: &Self, stream: Rc<dyn ITcpStreamWrapper>, _content_type: &ContentType) -> Rc<dyn ITcpStreamWrapper> {
        Rc::new(GzipBodyStream::new(stream))
    }

    fn type_info(self: &Self) -> Box<TypeInfo> {
        Box::new(TypeInfo::of::<Self>())
    }
}
