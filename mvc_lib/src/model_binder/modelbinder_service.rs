use std::any::Any;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;
use crate::services::service_collection::{IServiceCollection, ServiceCollection, ServiceCollectionExtensions};
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;

use super::imodel::IModel;
use super::imodelbinder_service::IModelBinderService;
use super::model_binder_resolver::IModelBinderResolver;
use super::model_validation_result::ModelValidationResult;




pub struct ModelBinderService {
    resolvers: Vec<Rc<dyn IModelBinderResolver>>,
}

impl ModelBinderService {
    pub fn new(
        resolvers: Vec<Rc<dyn IModelBinderResolver>>,
    ) -> Self {
        if resolvers.len() == 0 {
            panic!("ModelBinderService::new: no model binder resolvers provided");
        }

        Self {
            resolvers: resolvers,
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_one_or_more::<dyn IModelBinderResolver>(services)
        )) as Rc<dyn IModelBinderService>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IModelBinderService>(), Self::new_service, ServiceScope::Request));
    }
}

impl IModelBinderService for ModelBinderService {
    fn bind_model(&self, request_context: &dyn IRequestContext, model_type: &TypeInfo) -> ModelValidationResult<Rc<dyn IModel>> {
        for resolver in self.resolvers.iter() {
            if let Some(binder) = resolver.resolve_for_request(request_context) {
                return binder.bind_model(request_context);
            }
        }

        // println!("Resolver count: {}", self.resolvers.len());

        let avail_types_str = self.resolvers
            .iter()
            .flat_map(|r| r.get_binders())
            .map(|r| r.as_ref().type_info().type_name.as_ref().to_string())
            .collect::<Vec<String>>()
            .join(", ");

        println!("No model binder found for model type: {}\nAvailable types: {}", model_type.type_name, avail_types_str);
        // should also be able to check type for default instantiation function (constructor)
        // and then check if the model implements an interface that accepts the body content as a parameter
        // such that the type can be instantiated with defaults then the interface can be used to get values
        // from the body content and bind them to the model.
        ModelValidationResult::<Rc<dyn IModel>>::OkNone
    }
}