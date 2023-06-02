use std::any::Any;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;
use crate::services::service_collection::{IServiceCollection, ServiceCollection, ServiceCollectionExtensions};
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;

use super::imodelbinder_service::IModelBinderService;
use super::model_binder_resolver::IModelBinderResolver;
use super::view_model_result::ViewModelResult;




pub struct ModelBinderService {
    resolvers: Vec<Rc<dyn IModelBinderResolver>>,
}

impl ModelBinderService {
    pub fn new(
        resolvers: Vec<Rc<dyn IModelBinderResolver>>,
    ) -> Self {
        Self {
            resolvers: resolvers,
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_multiple::<dyn IModelBinderResolver>(services)
        )) as Rc<dyn IModelBinderService>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IModelBinderService>(), Self::new_service, ServiceScope::Request));
    }
}

impl IModelBinderService for ModelBinderService {
    fn bind_model(&self, request_context: &dyn IRequestContext, model_type: &TypeInfo) -> ViewModelResult<Rc<dyn Any>> {
        for resolver in self.resolvers.iter() {
            if let Some(binder) = resolver.resolve_for_request(request_context) {
                return binder.bind_view_model(request_context);
            }
        }
        ViewModelResult::<Rc<dyn Any>>::OkNone
    }
}