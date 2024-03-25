use std::{rc::Rc, any::Any};

use crate::contexts::iresponse_context::IResponseContext;
use crate::core::type_info::TypeInfo;
use crate::model_binder::imodel::AnyIModel;
use crate::model_binder::imodel_binder::IModelBinder;
use crate::model_binder::iviewmodel_encoder::IViewModelEncoder;
use crate::model_binder::model_validation_result::ModelValidationResult;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;
use crate::services::service_collection::IServiceCollection;


// this struct is used to encode the view model as form url encoded.
pub struct FormUrlEncodedEncoder {}

impl FormUrlEncodedEncoder {
    // creates a new instance of FormUrlEncodedEncoder.
    pub fn new() -> Self {
        Self {
        }
    }

    // creates a new instance of FormUrlEncodedEncoder as a service from the given IServiceCollection.
    // services: the IServiceCollection to create the FormUrlEncodedEncoder from.
    // returns: a Vec of Box<dyn Any> containing the FormUrlEncodedEncoder as a service.
    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
        )) as Rc<dyn IViewModelEncoder>)]
    }

    // adds the FormUrlEncodedEncoder to the given IServiceCollection.
    // services: the IServiceCollection to add the FormUrlEncodedEncoder to.
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IModelBinder>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IViewModelEncoder for FormUrlEncodedEncoder {
    fn matches_content_type(self: &Self, content_type: &str) -> bool {
        content_type.starts_with("application/x-www-form-urlencoded") // Content-Type
    }

    fn encode_model(self: &Self, _: Box<dyn Any>, _: &dyn IResponseContext) -> ModelValidationResult<AnyIModel> {
        panic!("encode_model not implemented for FormUrlEncodedEncoder");
    }
}