use std::any::Any;
use std::rc::Rc;
use std::error::Error;

use crate::services::service_collection::{ServiceCollection, IServiceCollection};
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;

use super::ierror_context::IErrorContext;
use super::ierror_handler::IErrorHandler;




pub struct LogErrorHandler {

}

impl LogErrorHandler {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IErrorHandler>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new_from::<dyn IErrorHandler, Self>(Self::new_service, ServiceScope::Singleton));
    }
}

impl IErrorHandler for LogErrorHandler {
    fn handle_error(self: &Self, error_context: &dyn IErrorContext) -> Result<bool, Rc<dyn Error>> {
        println!("LogErrorHandler::handle_error: {:?}", error_context.get_error());
        Ok(true)
    }

    fn to_string(self: &Self) -> String {
        nameof::name_of_type!(LogErrorHandler).to_string()
    }
}