use std::{rc::Rc, any::Any};

use crate::core::type_info::TypeInfo;
use crate::http::http_body_content::{IBodyContent, ContentType};
use crate::http::ihttp_body_stream_format::IHttpBodyStreamFormat;
use crate::model_binder::url_encoded_model::UrlEncodedModel;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;
use crate::services::service_collection::IServiceCollection;


// this struct is used to decode the view model from the request body.
pub struct UrlEncodedDecoder {
}

impl UrlEncodedDecoder {
    // creates a new instance of UrlEncodedDecoder.
    pub fn new() -> Self {
        Self {
        }
    }

    // creates a new instance of UrlEncodedDecoder as a service from the given IServiceCollection.
    // services: the IServiceCollection to create the UrlEncodedDecoder from.
    // returns: a Vec of Box<dyn Any> containing the UrlEncodedDecoder as a service.
    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
        )) as Rc<dyn IHttpBodyStreamFormat>)]
    }

    // adds the UrlEncodedDecoder to the given IServiceCollection.
    // services: the IServiceCollection to add the UrlEncodedDecoder to.
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IHttpBodyStreamFormat>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IHttpBodyStreamFormat for UrlEncodedDecoder {
    fn matches_content_type(self: &Self, content_type: ContentType) -> bool {
        content_type.mime_type.starts_with("application/x-www-form-urlencoded")
    }

    fn decode(self: &Self, body: Rc<dyn IBodyContent>) -> Rc<dyn IBodyContent> {
        Rc::new(UrlEncodedModel::new(body))
    }

    fn type_info(self: &Self) -> Box<TypeInfo> {
        TypeInfo::rc_of::<UrlEncodedDecoder>()
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