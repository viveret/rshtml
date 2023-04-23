use std::{rc::Rc, any::Any};

use crate::contexts::response_context::ResponseContext;
use crate::core::type_info::TypeInfo;
use crate::model::form_url_encoded_model::FormUrlEncodedModel;
use crate::model::iviewmodel_binder::IViewModelBinder;
use crate::model::iviewmodel_encoder::IViewModelEncoder;
use crate::model::view_model_result::ViewModelResult;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;
use crate::services::service_collection::IServiceCollection;

pub struct FormUrlEncodedEncoder {
}

impl FormUrlEncodedEncoder {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
        )) as Rc<dyn IViewModelEncoder>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IViewModelBinder>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IViewModelEncoder for FormUrlEncodedEncoder {
    fn matches_content_type(self: &Self, content_type: &str) -> bool {
        content_type.starts_with("application/x-www-form-urlencoded") // Content-Type
    }

    fn encode_model(self: &Self, model: Box<dyn Any>, response_context: Rc<ResponseContext>) -> ViewModelResult<Vec<u8>> {
        panic!("not implemented");
        ViewModelResult::<Vec<u8>>::Ok(vec![])
    }
}