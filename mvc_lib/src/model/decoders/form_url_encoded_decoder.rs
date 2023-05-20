use std::{rc::Rc, any::Any};

use crate::core::type_info::TypeInfo;
use crate::model::form_url_encoded_model::FormUrlEncodedModel;
use crate::model::iviewmodel_decoder::IViewModelDecoder;
use crate::model::view_model_result::ViewModelResult;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;
use crate::services::service_collection::IServiceCollection;
use crate::contexts::irequest_context::IRequestContext;


// this struct is used to decode the view model from the request body.
pub struct FormUrlEncodedDecoder {
}

impl FormUrlEncodedDecoder {
    // creates a new instance of FormUrlEncodedDecoder.
    pub fn new() -> Self {
        Self {
        }
    }

    // creates a new instance of FormUrlEncodedDecoder as a service from the given IServiceCollection.
    // services: the IServiceCollection to create the FormUrlEncodedDecoder from.
    // returns: a Vec of Box<dyn Any> containing the FormUrlEncodedDecoder as a service.
    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
        )) as Rc<dyn IViewModelDecoder>)]
    }

    // adds the FormUrlEncodedDecoder to the given IServiceCollection.
    // services: the IServiceCollection to add the FormUrlEncodedDecoder to.
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IViewModelDecoder>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IViewModelDecoder for FormUrlEncodedDecoder {
    fn matches_content_type(self: &Self, content_type: &str) -> bool {
        content_type.starts_with("application/x-www-form-urlencoded")
    }

    fn decode_model(self: &Self, request_context: Rc<dyn IRequestContext>) -> ViewModelResult<Rc<dyn Any>> {
        ViewModelResult::<Rc<dyn Any>>::Ok(Rc::new(Rc::new(FormUrlEncodedModel::parse_body(&request_context.get_body()))))
    }
}