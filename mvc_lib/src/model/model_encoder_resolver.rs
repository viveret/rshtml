use std::rc::Rc;
use std::any::Any;

use crate::contexts::response_context::ResponseContext;
use crate::core::type_info::TypeInfo;
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions, ServiceCollection};

use super::iviewmodel_encoder::IViewModelEncoder;
use super::view_model_result::ViewModelResult;


// this struct is used to resolve the correct IViewModelEncoder for a given content type and context.
// it is used by the ModelEncoderMiddleware.
// it uses the IViewModelEncoder instances registered in the IServiceCollection to encode the view model.
pub struct ModelEncoderResolver {
    // the view model binders used to encode the view model.
    view_model_binders: Vec<Rc<dyn IViewModelEncoder>>,
}

impl ModelEncoderResolver {
    // creates a new instance of ModelEncoderResolver from the given view model binders.
    // view_model_binders: the view model binders used to encode the view model.
    pub fn new(view_model_binders: Vec<Rc<dyn IViewModelEncoder>>) -> Self {
        Self {
            view_model_binders: view_model_binders,
        }
    }

    // creates a new instance of ModelEncoderResolver as a service from the given IServiceCollection.
    // services: the IServiceCollection to create the ModelEncoderResolver from.
    // returns: a Vec of Box<dyn Any> containing the ModelEncoderResolver as a service.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_multiple::<dyn IViewModelEncoder>(services)
        )))]
    }

    // adds the ModelEncoderResolver to the given IServiceCollection.
    // services: the IServiceCollection to add the ModelEncoderResolver to.
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<ModelEncoderResolver>(), Self::new_service, ServiceScope::Singleton));
    }

    // resolves the correct IViewModelEncoder for the given content type.
    // content_type: the content type to resolve the IViewModelEncoder for.
    // returns: the resolved IViewModelEncoder if found, otherwise None.
    pub fn resolve_for_content_type(self: &Self, content_type: &str) -> Option<Rc<dyn IViewModelEncoder>> {
        for it in self.view_model_binders.iter() {
            if it.matches_content_type(content_type) {
                return Some(it.clone());
            }
        }
        None
    }

    // encodes the view model for the given response context.
    // model: the view model to encode.
    // response_context: the response context to encode the view model for.
    // returns: the result of the encoding.
    pub fn encode_view_model(self: &Self, model: Box<dyn Any>, response_context: Rc<ResponseContext>) -> ViewModelResult<Vec<u8>> {
        let content_type = response_context.as_ref().get_headers().get("Content-Type").unwrap().to_str().unwrap();
        if let Some(binder) = self.resolve_for_content_type(content_type) {
            return binder.encode_model(model, response_context.clone());
        }
        ViewModelResult::<Vec<u8>>::OkNone
    }
}