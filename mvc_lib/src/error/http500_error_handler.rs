use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

use http::StatusCode;

use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;
use crate::services::service_collection::{ServiceCollection, IServiceCollection, ServiceCollectionExtensions};

use super::error_viewmodel_service::{self, IErrorViewModelService};
use super::ierror_context::IErrorContext;
use super::ierror_handler::IErrorHandler;

pub struct Http500ErrorHandler {
}

impl Http500ErrorHandler {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new_from::<dyn IErrorHandler, Self>(Self::new_service, ServiceScope::Singleton));
    }

    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
        )) as Rc<dyn IErrorHandler>)]
    }
}

impl IErrorHandler for Http500ErrorHandler {
    fn handle_error(self: &Self, error_context: &dyn IErrorContext) -> Result<bool, Rc<dyn Error>> {
        error_context.get_response_context().set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
        println!("Status code has been set to 500.");
        Ok(true)
    }
}