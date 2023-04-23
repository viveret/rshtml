use std::rc::Rc;
use std::any::Any;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions, ServiceCollection};

use super::iviewmodel_binder::IViewModelBinder;
use super::view_model_result::ViewModelResult;



pub struct ViewModelBinderResolver {
    view_model_binders: Vec<Rc<dyn IViewModelBinder>>,
}


impl ViewModelBinderResolver {
    pub fn new(view_model_binders: Vec<Rc<dyn IViewModelBinder>>) -> Self {
        Self {
            view_model_binders: view_model_binders,
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_multiple::<dyn IViewModelBinder>(services)
        )))]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<ViewModelBinderResolver>(), Self::new_service, ServiceScope::Singleton));
    }

    pub fn resolve_for_content_type(self: &Self, content_type: &str) -> Option<Rc<dyn IViewModelBinder>> {
        for it in self.view_model_binders.iter() {
            if it.matches_content_type(content_type) {
                return Some(it.clone());
            }
        }
        None
    }

    pub fn bind_and_validate_view_model(self: &Self, request_context: Rc<dyn IRequestContext>) -> ViewModelResult<Box<dyn Any>> {
        let content_type = request_context.get_headers().get("Content-Type").unwrap().to_str().unwrap();
        if let Some(binder) = self.resolve_for_content_type(content_type) {
            return binder.bind_view_model(request_context.clone());
        }
        ViewModelResult::<Box<dyn Any>>::OkNone
    }
}