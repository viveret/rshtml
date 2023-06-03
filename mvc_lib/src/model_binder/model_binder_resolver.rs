use std::rc::Rc;
use std::any::Any;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions, ServiceCollection};

use super::imodel_binder::IModelBinder;

// this trait is used to resolve the correct IModelBinder for a given content type and context.
pub trait IModelBinderResolver {
    // resolves the correct IModelBinder for the given content type.
    // content_type: the content type to resolve the IModelBinder for.
    // returns: the resolved IModelBinder if found, otherwise None.
    fn resolve_for_request(self: &Self, request_context: &dyn IRequestContext) -> Option<Rc<dyn IModelBinder>>;

    // get the IModelBinder instances used by this IModelBinderResolver.
    fn get_binders(self: &Self) -> Vec<Rc<dyn IModelBinder>>;
}

// this struct is used to resolve the correct IModelBinder for a given content type and context.
// it is used by the ModelDecoderMiddleware.
// it uses the IModelBinder instances registered in the IServiceCollection to decode the view model.
pub struct ModelBinderResolver {
    // the view model binders used to decode the view model.
    model_binders: Vec<Rc<dyn IModelBinder>>,
}

impl ModelBinderResolver {
    // creates a new instance of ModelBinderResolver from the given view model binders.
    // view_model_binders: the view model binders used to decode the view model.
    pub fn new(model_binders: Vec<Rc<dyn IModelBinder>>) -> Self {
        Self {
            model_binders: model_binders,
        }
    }

    // creates a new instance of ModelBinderResolver as a service from the given IServiceCollection.
    // services: the IServiceCollection to create the ModelBinderResolver from.
    // returns: a Vec of Box<dyn Any> containing the ModelBinderResolver as a service.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_multiple::<dyn IModelBinder>(services)
        )))]
    }

    // adds the ModelBinderResolver to the given IServiceCollection.
    // services: the IServiceCollection to add the ModelBinderResolver to.
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<ModelBinderResolver>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IModelBinderResolver for ModelBinderResolver {
    fn resolve_for_request(self: &Self, request_context: &dyn IRequestContext) -> Option<Rc<dyn IModelBinder>> {
        for it in self.model_binders.iter() {
            if it.matches(request_context) {
                return Some(it.clone());
            }
        }
        None
    }

    fn get_binders(self: &Self) -> Vec<Rc<dyn IModelBinder>> {
        self.model_binders.clone()
    }
}