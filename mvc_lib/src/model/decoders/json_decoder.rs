use std::any::Any;
use std::rc::Rc;

use crate::core::type_info::TypeInfo;
use crate::model::iviewmodel_decoder::IViewModelDecoder;
use crate::model::view_model_result::ViewModelResult;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollection};
use crate::services::service_scope::ServiceScope;



pub struct JsonDecoder {

}

impl JsonDecoder {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
        )) as Rc<dyn IViewModelDecoder>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IViewModelDecoder>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IViewModelDecoder for JsonDecoder {
    fn matches_content_type(self: &Self, content_type: &str) -> bool {
        false
    }

    fn decode_model(self: &Self, request_context: Rc<dyn crate::contexts::irequest_context::IRequestContext>) -> ViewModelResult<Rc<dyn Any>> {
        todo!()
    }
}