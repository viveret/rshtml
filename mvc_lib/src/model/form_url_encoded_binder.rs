// use std::{rc::Rc, any::Any};

// use crate::core::type_info::TypeInfo;
// use crate::services::service_collection::ServiceCollection;
// use crate::services::service_descriptor::ServiceDescriptor;
// use crate::services::service_scope::ServiceScope;
// use crate::services::service_collection::IServiceCollection;
// use crate::contexts::irequest_context::IRequestContext;

// use super::form_url_encoded_model::FormUrlEncodedModel;
// use super::iviewmodel_binder::IViewModelBinder;
// use super::view_model_result::ViewModelResult;

// pub struct FormUrlEncodedBinder {
// }

// impl FormUrlEncodedBinder {
//     pub fn new() -> Self {
//         Self {
//         }
//     }

//     pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
//         vec![Box::new(Rc::new(Self::new(
//         )) as Rc<dyn IViewModelBinder>)]
//     }

//     pub fn add_to_services(services: &mut ServiceCollection) {
//         services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IViewModelBinder>(), Self::new_service, ServiceScope::Singleton));
//     }
// }

// impl IViewModelBinder for FormUrlEncodedBinder {
//     fn matches_content_type(self: &Self, content_type: &str) -> bool {
//         content_type.starts_with("application/x-www-form-urlencoded") // Content-Type
//     }

//     fn bind_view_model(self: &Self, request_context: Rc<dyn IRequestContext>) -> ViewModelResult<Box<dyn Any>> {
//         ViewModelResult::Ok(Box::new(Rc::new(FormUrlEncodedModel::parse_body(&request_context.get_body()))))
//     }
// }