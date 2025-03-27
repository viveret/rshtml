use std::any::Any;
use std::io::Write;
use std::io::Result;
use std::io::Read;
use std::rc::Rc;

use crate::options::app_content_provider_service_options::IAppContentProviderServiceOptions;
use crate::services::service_collection::ServiceCollectionExtensions;

use super::service_collection::ServiceCollection;
use super::service_collection::IServiceCollection;
use super::service_descriptor::ServiceDescriptor;
use super::service_scope::ServiceScope;


pub trait IAppContentProviderService {
    // opens a file for reading.
    fn open_read(&self, path: &str) -> Result<Box<dyn Read>>;
    // opens a file for writing.
    fn open_write(&self, path: &str) -> Result<Box<dyn Write>>;

    // reads a string from a file.
    fn read_string(&self, path: &str) -> Result<String>;
    // writes a string to a file.
    fn write_string(&self, path: &str, data: &String) -> Result<()>;

    // list entries in a directory
    fn list(&self, path: &str) -> Result<Vec<String>>;

    fn resolve_path(&self, path: &str) -> Option<String>;
}

pub struct AppContentProviderService {
    options: Rc<dyn IAppContentProviderServiceOptions>,
}

impl AppContentProviderService {
    pub fn new(options: Rc<dyn IAppContentProviderServiceOptions>) -> Self {
        Self {
            options
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IAppContentProviderServiceOptions>(services)
        )) as Rc<dyn IAppContentProviderService>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new_from::<dyn IAppContentProviderService, Self>(Self::new_service, ServiceScope::Singleton));
    }
}

impl IAppContentProviderService for AppContentProviderService {
    fn open_read(&self, path: &str) -> Result<Box<dyn Read>> {
        todo!()
    }

    fn open_write(&self, path: &str) -> Result<Box<dyn Write>> {
        todo!()
    }

    fn read_string(&self, path: &str) -> Result<String> {
        todo!()
    }

    fn write_string(&self, path: &str, data: &String) -> Result<()> {
        todo!()
    }

    fn list(&self, path: &str) -> Result<Vec<String>> {
        self.options.list(path)
    }
    
    fn resolve_path(&self, path: &str) -> Option<String> {
        self.options.resolve_path(path)
    }
}