use std::rc::Rc;
use std::any::Any;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::ResponseContext;
use crate::core::type_info::TypeInfo;
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions, ServiceCollection};

use super::iviewmodel_encoder::IViewModelEncoder;
use super::view_model_result::ViewModelResult;









pub struct ModelEncoderResolver {
    view_model_binders: Vec<Rc<dyn IViewModelEncoder>>,
}


impl ModelEncoderResolver {
    pub fn new(view_model_binders: Vec<Rc<dyn IViewModelEncoder>>) -> Self {
        Self {
            view_model_binders: view_model_binders,
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_multiple::<dyn IViewModelEncoder>(services)
        )))]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<ModelEncoderResolver>(), Self::new_service, ServiceScope::Singleton));
    }

    pub fn resolve_for_content_type(self: &Self, content_type: &str) -> Option<Rc<dyn IViewModelEncoder>> {
        for it in self.view_model_binders.iter() {
            if it.matches_content_type(content_type) {
                return Some(it.clone());
            }
        }
        None
    }

    pub fn encode_view_model(self: &Self, model: Box<dyn Any>, response_context: Rc<ResponseContext>) -> ViewModelResult<Vec<u8>> {
        let content_type = response_context.as_ref().get_headers().get("Content-Type").unwrap().to_str().unwrap();
        if let Some(binder) = self.resolve_for_content_type(content_type) {
            return binder.encode_model(model, response_context.clone());
        }
        ViewModelResult::<Vec<u8>>::OkNone
    }
}