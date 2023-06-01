use std::rc::Rc;
use std::any::Any;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions, ServiceCollection};

use super::iviewmodel_binder::IViewModelBinder;
use super::view_model_result::ViewModelResult;


// this struct is used to resolve the correct IViewModelBinder for a given content type and context.
// it is used by the ModelDecoderMiddleware.
// it uses the IViewModelBinder instances registered in the IServiceCollection to decode the view model.
pub struct ModelBinderResolver {
    // the view model binders used to decode the view model.
    model_binders: Vec<Rc<dyn IViewModelBinder>>,
}

impl ModelBinderResolver {
    // creates a new instance of ModelBinderResolver from the given view model binders.
    // view_model_binders: the view model binders used to decode the view model.
    pub fn new(model_binders: Vec<Rc<dyn IViewModelBinder>>) -> Self {
        Self {
            model_binders: model_binders,
        }
    }

    // creates a new instance of ModelBinderResolver as a service from the given IServiceCollection.
    // services: the IServiceCollection to create the ModelBinderResolver from.
    // returns: a Vec of Box<dyn Any> containing the ModelBinderResolver as a service.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_multiple::<dyn IViewModelBinder>(services)
        )))]
    }

    // adds the ModelBinderResolver to the given IServiceCollection.
    // services: the IServiceCollection to add the ModelBinderResolver to.
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<ModelBinderResolver>(), Self::new_service, ServiceScope::Singleton));
    }

    // resolves the correct IViewModelBinder for the given content type.
    // content_type: the content type to resolve the IViewModelBinder for.
    // returns: the resolved IViewModelBinder if found, otherwise None.
    pub fn resolve_for_request(self: &Self, request_context: &dyn IRequestContext) -> Option<Rc<dyn IViewModelBinder>> {
        // // expecting content-length in order to read, decode, and parse the body.
        // let mut found_content_length = request_context.get_content_length();

        // let mut found_content_type = request_context.get_content_type();

        for it in self.model_binders.iter() {
            if it.matches(request_context) {
                return Some(it.clone());
            }
        }
        None
    }

    // decodes the view model for the given request context.
    // request_context: the request context to decode the view model for.
    // returns: the decoded view model result.
    pub fn bind_view_model(self: &Self, request_context: &dyn IRequestContext) -> ViewModelResult<Rc<dyn Any>> {
        if let Some(binder) = self.resolve_for_request(request_context) {
            return binder.bind_view_model(request_context);
        }
        ViewModelResult::<Rc<dyn Any>>::OkNone
    }
}