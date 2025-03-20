use std::any::Any;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter, Result};
use std::rc::Rc;

use crate::app::ihttp_request_pipeline::IHttpRequestPipeline;
use crate::options::app_content_provider_service_options::AppContentProviderServiceOptions;
use crate::services::service_collection::ServiceCollectionExtensions;

use super::service_collection::{IServiceCollection, ServiceCollection};
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
}

pub struct AppContentProviderService {
    options: Rc<AppContentProviderServiceOptions>,
}

impl AppContentProviderService {
    pub fn new(options: Rc<AppContentProviderServiceOptions>) -> Self {
        Self {
            options
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<AppContentProviderServiceOptions>(services)
        )) as Rc<dyn IAppContentProviderService>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new_from::<dyn IHttpRequestPipeline, Self>(Self::new_service, ServiceScope::Singleton));
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
        todo!()
    }
}