use std::rc::Rc;
use std::any::Any;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions, ServiceCollection};

use super::iviewmodel_decoder::IViewModelDecoder;
use super::view_model_result::ViewModelResult;


// this struct is used to resolve the correct IViewModelDecoder for a given content type and context.
// it is used by the ModelDecoderMiddleware.
// it uses the IViewModelDecoder instances registered in the IServiceCollection to decode the view model.
pub struct ModelDecoderResolver {
    // the view model binders used to decode the view model.
    model_decoders: Vec<Rc<dyn IViewModelDecoder>>,
}

impl ModelDecoderResolver {
    // creates a new instance of ModelDecoderResolver from the given view model binders.
    // view_model_binders: the view model binders used to decode the view model.
    pub fn new(model_decoders: Vec<Rc<dyn IViewModelDecoder>>) -> Self {
        Self {
            model_decoders: model_decoders,
        }
    }

    // creates a new instance of ModelDecoderResolver as a service from the given IServiceCollection.
    // services: the IServiceCollection to create the ModelDecoderResolver from.
    // returns: a Vec of Box<dyn Any> containing the ModelDecoderResolver as a service.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_multiple::<dyn IViewModelDecoder>(services)
        )))]
    }

    // adds the ModelDecoderResolver to the given IServiceCollection.
    // services: the IServiceCollection to add the ModelDecoderResolver to.
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<ModelDecoderResolver>(), Self::new_service, ServiceScope::Singleton));
    }

    // resolves the correct IViewModelDecoder for the given content type.
    // content_type: the content type to resolve the IViewModelDecoder for.
    // returns: the resolved IViewModelDecoder if found, otherwise None.
    pub fn resolve_for_content_type(self: &Self, content_type: &str) -> Option<Rc<dyn IViewModelDecoder>> {
        for it in self.model_decoders.iter() {
            if it.matches_content_type(content_type) {
                return Some(it.clone());
            }
        }
        None
    }

    // decodes the view model for the given request context.
    // request_context: the request context to decode the view model for.
    // returns: the decoded view model result.
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