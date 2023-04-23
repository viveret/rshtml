use std::rc::Rc;
use std::any::Any;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions, ServiceCollection};

use super::iviewmodel_decoder::IViewModelDecoder;
use super::view_model_result::ViewModelResult;


pub struct ModelDecoderResolver {
    model_decoders: Vec<Rc<dyn IViewModelDecoder>>,
}


impl ModelDecoderResolver {
    pub fn new(model_decoders: Vec<Rc<dyn IViewModelDecoder>>) -> Self {
        Self {
            model_decoders: model_decoders,
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_multiple::<dyn IViewModelDecoder>(services)
        )))]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<ModelDecoderResolver>(), Self::new_service, ServiceScope::Singleton));
    }

    pub fn resolve_for_content_type(self: &Self, content_type: &str) -> Option<Rc<dyn IViewModelDecoder>> {
        for it in self.model_decoders.iter() {
            if it.matches_content_type(content_type) {
                return Some(it.clone());
            }
        }
        None
    }

    pub fn decode_model(self: &Self, request_context: Rc<dyn IRequestContext>) -> ViewModelResult<Rc<dyn Any>> {
        match request_context.get_headers().get("Content-Type") {
            Some(content_type) => {
                let content_type = content_type.to_str().unwrap();
                if let Some(decoder) = self.resolve_for_content_type(content_type) {
                    return decoder.decode_model(request_context.clone());
                }
            },
            None => {}
        }
        ViewModelResult::<Rc<dyn Any>>::OkNone
    }
}