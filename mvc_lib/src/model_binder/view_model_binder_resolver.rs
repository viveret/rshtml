use std::rc::Rc;
use std::any::Any;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions, ServiceCollection};

use super::iviewmodel_binder::IViewModelBinder;
use super::view_model_result::ViewModelResult;


// this trait represents a view model binder resolver which is used to resolve the correct IViewModelBinder for a given content type and context.
pub trait IViewModelBinderResolver {
    // resolves the correct IViewModelBinder for the given content type.
    fn resolve_for_content_type(self: &Self, request_context: &dyn IRequestContext) -> Option<Rc<dyn IViewModelBinder>>;

    // binds and validates the view model for the given request context.
    // request_context: the request context to bind and validate the view model for.
    // returns: the result of the binding and validation.
    fn bind_and_validate_view_model(self: &Self, request_context: &dyn IRequestContext) -> ViewModelResult<Rc<dyn Any>>;
}


// this struct is used to resolve the correct IViewModelBinder for a given content type and context.
// it is used by the ModelBinderResolverMiddleware.
// it uses the IViewModelBinder instances registered in the IServiceCollection to validate and bind the view model.
pub struct ViewModelBinderResolver {
    // the view model binders used to validate and bind the view model.
    view_model_binders: Vec<Rc<dyn IViewModelBinder>>,
}

impl ViewModelBinderResolver {
    // creates a new instance of ViewModelBinderResolver from the given view model binders.
    // view_model_binders: the view model binders used to validate and bind the view model.
    pub fn new(view_model_binders: Vec<Rc<dyn IViewModelBinder>>) -> Self {
        Self {
            view_model_binders: view_model_binders,
        }
    }

    // creates a new instance of ViewModelBinderResolver as a service from the given IServiceCollection.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_multiple::<dyn IViewModelBinder>(services)
        )))]
    }

    // adds the ViewModelBinderResolver to the given IServiceCollection.
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<ViewModelBinderResolver>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IViewModelBinderResolver for ViewModelBinderResolver {
    fn resolve_for_content_type(self: &Self, request_context: &dyn IRequestContext) -> Option<Rc<dyn IViewModelBinder>> {
        for it in self.view_model_binders.iter() {
            if it.matches(request_context) {
                return Some(it.clone());
            }
        }
        None
    }

    fn bind_and_validate_view_model(self: &Self, request_context: &dyn IRequestContext) -> ViewModelResult<Rc<dyn Any>> {
        if let Some(binder) = self.resolve_for_content_type(request_context.clone()) {
            return binder.bind_view_model(request_context);
        }
        ViewModelResult::<Rc<dyn Any>>::OkNone
    }
}