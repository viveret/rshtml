use core::panic;
use std::rc::Rc;
use std::any::Any;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions, ServiceCollection};

use super::imodel::IModel;
use super::imodel_binder::IModelBinder;
use super::model_validation_result::ModelValidationResult;


// this trait represents a view model binder resolver which is used to resolve the correct IModelBinder for a given content type and context.
pub trait IModelBinderResolver {
    // resolves the correct IModelBinder for the given content type.
    fn resolve_for_content_type(self: &Self, request_context: &dyn IRequestContext) -> Option<Rc<dyn IModelBinder>>;

    // binds and validates the view model for the given request context.
    // request_context: the request context to bind and validate the view model for.
    // returns: the result of the binding and validation.
    fn bind_and_validate_view_model(self: &Self, request_context: &dyn IRequestContext) -> ModelValidationResult<Rc<dyn IModel>>;
}


// this struct is used to resolve the correct IModelBinder for a given content type and context.
// it is used by the ModelBinderResolverMiddleware.
// it uses the IModelBinder instances registered in the IServiceCollection to validate and bind the view model.
pub struct ModelBinderResolver {
    // the view model binders used to validate and bind the view model.
    model_binders: Vec<Rc<dyn IModelBinder>>,
}

impl ModelBinderResolver {
    // creates a new instance of ModelBinderResolver from the given view model binders.
    // model_binders: the view model binders used to validate and bind the view model.
    pub fn new(model_binders: Vec<Rc<dyn IModelBinder>>) -> Self {
        panic!("ModelBinderResolver::new");
        println!("model_binders: {:?}", model_binders.iter().map(|r| r.as_ref().type_info().type_name.as_ref().to_string()).collect::<Vec<String>>().join(", "));
        Self {
            model_binders: model_binders,
        }
    }

    // creates a new instance of ModelBinderResolver as a service from the given IServiceCollection.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        panic!("ModelBinderResolver::new_service");
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_multiple::<dyn IModelBinder>(services)
        )))]
    }

    // adds the ModelBinderResolver to the given IServiceCollection.
    pub fn add_to_services(services: &mut ServiceCollection) {
        panic!("ModelBinderResolver::add_to_services");
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IModelBinderResolver>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IModelBinderResolver for ModelBinderResolver {
    fn resolve_for_content_type(self: &Self, request_context: &dyn IRequestContext) -> Option<Rc<dyn IModelBinder>> {
        for it in self.model_binders.iter() {
            if it.matches(request_context) {
                return Some(it.clone());
            }
        }
        None
    }

    fn bind_and_validate_view_model(self: &Self, request_context: &dyn IRequestContext) -> ModelValidationResult<Rc<dyn IModel>> {
        if let Some(binder) = self.resolve_for_content_type(request_context.clone()) {
            return binder.bind_model(request_context);
        }
        ModelValidationResult::<Rc<dyn IModel>>::OkNone
    }
}