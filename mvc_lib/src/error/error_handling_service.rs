use std::any::Any;
use std::rc::Rc;
use std::error::Error;

use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollection};


// this is a trait for a class that can be used to handle errors by using any number of error handlers.
pub trait IErrorHandlingService {
    // fn handle_error(self: &Self, error: Box<dyn Error>) -> Result<bool, Box<dyn Error>>;
    fn handle_error(self: &Self, error: Box<dyn Error>) -> Result<(), Box<dyn Error>>;
}

pub struct ErrorHandlingService {
}

impl ErrorHandlingService {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IErrorHandlingService>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new_from::<dyn IErrorHandlingService, Self>(Self::new_service, ServiceScope::Singleton));
    }
}

impl IErrorHandlingService for ErrorHandlingService {
    fn handle_error(self: &Self, error: Box<dyn Error>) -> Result<(), Box<dyn Error>> {
        // unhandled at this point.
        println!("ErrorHandlingService::handle_error: {:?}", error);
        Ok(())
    }
}