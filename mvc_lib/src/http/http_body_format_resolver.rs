use std::any::Any;
use std::rc::Rc;

use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions, ServiceCollection};
use crate::services::service_descriptor::ServiceDescriptor;

use super::ihttp_body_format_resolver::IHttpBodyFormatResolver;
use super::ihttp_body_stream_format::IHttpBodyStreamFormat;
use super::request_decoder_middleware::GzipBodyStreamFormat;
use super::http_body_content::ContentType;


pub struct HttpBodyFormatResolver {
    pub formats: Vec<Rc<dyn IHttpBodyStreamFormat>>,
}

impl HttpBodyFormatResolver {
    pub fn new(
        formats: Vec<Rc<dyn IHttpBodyStreamFormat>>,
    ) -> Self {
        Self {
            formats,
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![
            Box::new(Rc::new(Self::new(
                ServiceCollectionExtensions::get_required_multiple::<dyn IHttpBodyStreamFormat>(services)
            )) as Rc<dyn IHttpBodyFormatResolver>)
        ]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(
            crate::core::type_info::TypeInfo::rc_of::<dyn IHttpBodyFormatResolver>(),
            Self::new_service,
            crate::services::service_scope::ServiceScope::Singleton,
        ));
    }
}

impl IHttpBodyFormatResolver for HttpBodyFormatResolver {
    fn resolve(&self, content_type: &ContentType) -> Option<Rc<dyn IHttpBodyStreamFormat>> {
        match content_type.mime_type.as_str() {
            "application/gzip" | "gzip" => {
                Some(Rc::new(GzipBodyStreamFormat::new()) as Rc<dyn IHttpBodyStreamFormat>)
            },
            _ => {
                for format in &self.formats {
                    if format.matches_content_type(&content_type) {
                        return Some(format.clone());
                    }
                }
                None
            }
        }
    }
}