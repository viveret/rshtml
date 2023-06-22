use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use crate::core::type_info::TypeInfo;

use crate::action_results::view_result::ViewResult;

use crate::services::service_collection::{ServiceCollection, ServiceCollectionExtensions};
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;
use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::iresponse_context::IResponseContext;
use crate::services::service_collection::IServiceCollection;
use crate::services::request_middleware_service::MiddlewareResult;
use crate::services::request_middleware_service::IRequestMiddlewareService;

use super::error_handler_service::IErrorHandlerService;
use super::error_viewmodel_service::IErrorViewModelService;


// Renders an error view if an error occurs.
pub struct ErrorViewMiddleware {
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>,
    error_handler_service: Rc<dyn IErrorHandlerService>,
    error_viewmodel_service: Rc<dyn IErrorViewModelService>,
}

impl ErrorViewMiddleware {
    pub fn new(
        error_handler_service: Rc<dyn IErrorHandlerService>,
        error_viewmodel_service: Rc<dyn IErrorViewModelService>,
    ) -> Self {
        Self {
            next: RefCell::new(None),
            error_handler_service,
            error_viewmodel_service,
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new (
            ServiceCollectionExtensions::get_required_single::<dyn IErrorHandlerService>(services),
            ServiceCollectionExtensions::get_required_single::<dyn IErrorViewModelService>(services),
        )) as Rc<dyn IRequestMiddlewareService>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new_from::<dyn IRequestMiddlewareService, Self>(Self::new_service, ServiceScope::Request));
    }
}

impl IRequestMiddlewareService for ErrorViewMiddleware {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Rc<dyn Error>> {
        if let Some(next) = self.next.borrow().as_ref() {
            let result = next.handle_request(response_context, request_context, services);
            match result {
                Ok(result) => Ok(result),
                Err(error) => {
                    self.error_handler_service.handle_error(error.clone(), Some(request_context), Some(response_context))?;
                    response_context.set_action_result(Some(Rc::new(ViewResult::new("/shared/error".to_string(), self.error_viewmodel_service.create_error_viewmodel(error)))));
                    Ok(MiddlewareResult::OkBreak)
                },
            }
        } else {
            Ok(MiddlewareResult::OkContinue)
        }
    }

    fn get_type_info(&self) -> Box<crate::core::type_info::TypeInfo> {
        Box::new(TypeInfo::of::<ErrorViewMiddleware>())
    }
}