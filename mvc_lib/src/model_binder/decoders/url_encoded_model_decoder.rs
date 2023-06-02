use std::{rc::Rc, any::Any};

use crate::core::itcp_stream_wrapper::ITcpStreamWrapper;
use crate::core::type_info::TypeInfo;
use crate::http::http_body_content::{IBodyContent, ContentType};
use crate::http::ihttp_body_stream_format::IHttpBodyStreamFormat;
use crate::model_binder::url_encoded_model::UrlEncodedModel;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;
use crate::services::service_collection::IServiceCollection;


// this struct is used to decode the view model from the request body.
pub struct UrlEncodedStream {
    inner_stream: Rc<dyn ITcpStreamWrapper>,
}

impl UrlEncodedStream {
    // creates a new instance of UrlEncodedStream.
    pub fn new(inner_stream: Rc<dyn ITcpStreamWrapper>) -> Self {
        Self {
            inner_stream: inner_stream,
        }
    }
}

impl ITcpStreamWrapper for UrlEncodedStream {
    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        self.inner_stream.shutdown(how)
    }

    fn flush(&self) -> std::io::Result<()> {
        self.inner_stream.flush()
    }

    fn read(&self, b: &mut [u8]) -> std::io::Result<usize> {
        self.inner_stream.read(b)
    }

    fn read_line(&self) -> Result<String, std::string::FromUtf8Error> {
        self.inner_stream.read_line()
    }

    fn write(&self, b: &[u8]) -> std::io::Result<usize> {
        self.inner_stream.write(b)
    }

    fn write_line(&self, b: &String) -> std::io::Result<usize> {
        self.inner_stream.write_line(b)
    }

    fn remote_addr(&self) -> std::net::SocketAddr {
        self.inner_stream.remote_addr()
    }
}

pub struct UrlEncodedFormatResolver {

}

impl UrlEncodedFormatResolver {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
        )) as Rc<dyn IHttpBodyStreamFormat>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IHttpBodyStreamFormat>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IHttpBodyStreamFormat for UrlEncodedFormatResolver {
    fn matches_content_type(self: &Self, content_type: &ContentType) -> bool {
        content_type.mime_type.starts_with("application/x-www-form-urlencoded")
    }

    fn decode(self: &Self, body: Rc<dyn ITcpStreamWrapper>, content_type: &ContentType) -> Rc<dyn ITcpStreamWrapper> {
        Rc::new(UrlEncodedStream::new(body))
    }

    fn type_info(self: &Self) -> Box<TypeInfo> {
        TypeInfo::rc_of::<UrlEncodedStream>()
    }

    fn encode(self: &Self, stream: Rc<dyn ITcpStreamWrapper>, content_type: &ContentType) -> Rc<dyn ITcpStreamWrapper> {
        Rc::new(UrlEncodedStream::new(stream))
    }
}

// impl IViewModelBinder for UrlEncodedDecoder {
//     fn matches_content_type(self: &Self, content_type: &str) -> bool {
//         content_type.starts_with("application/x-www-form-urlencoded")
//     }

//     fn bind_view_model(self: &Self, request_context: Rc<dyn IRequestContext>) -> ViewModelResult<Rc<dyn Any>> {
//         todo!()
//     }

//     // fn bind_model(self: &Self, request_context: Rc<dyn IRequestContext>) -> ViewModelResult<Rc<dyn Any>> {
//     //     if let Some(body) = request_context.get_body() {
//     //         ViewModelResult::<Rc<dyn Any>>::Ok(Rc::new(Rc::new(UrlEncodedModel::new(body)) as Rc<dyn IViewModel>))
//     //     } else {
//     //         panic!("UrlEncodedDecoder::decode_model: request_context.get_body() returned None.");
//     //     }
//     // }
// }