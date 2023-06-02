use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter, Result};
use std::rc::Rc;
use std::any::Any;

use crate::core::type_info::TypeInfo;
use crate::services::service_collection::IServiceCollection;

use super::service_descriptor::ServiceDescriptor;
use super::service_scope::ServiceScope;

// this is a trait for a class that can provide file services.
pub trait IFileProviderService {
    // opens a file for reading.
    fn open_read(self: &Self, path: &str) -> Result<Box<dyn Read>>;
    // opens a file for writing.
    fn open_write(self: &Self, path: &str) -> Result<Box<dyn Write>>;

    // reads a string from a file.
    fn read_string(self: &Self, path: &str) -> Result<String>;
    // writes a string to a file.
    fn write_string(self: &Self, path: &str, data: &String) -> Result<()>;
}

// implementation of the file provider service.
pub struct FileProviderService {
    // fn open_read(path: &str) -> Read,
    // fn open_write(path: &str) -> Write,
}

impl FileProviderService {
    // creates a new instance of the file provider service.
    pub fn new() -> Self {
        Self {}
    }

    // creates the file provider service as a service.
    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()))]
    }

    // adds the file provider service to the given service collection.
    pub fn add_to_services(services: &mut super::service_collection::ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IFileProviderService>(), FileProviderService::new_service, ServiceScope::Singleton));
    }
}

impl IFileProviderService for FileProviderService {
    fn open_read(self: &Self, path: &str) -> Result<Box<dyn Read>> {
        let file = File::open(path)?;
        Ok(Box::new(BufReader::new(file)))
    }

    fn open_write(self: &Self, path: &str) -> Result<Box<dyn Write>> {
        let file = File::open(path)?;
        Ok(Box::new(BufWriter::new(file)))
    }

    fn read_string(self: &Self, path: &str) -> Result<String> {
        Ok(std::fs::read_to_string(path)?)
    }
    
    fn write_string(self: &Self, path: &str, data: &String) -> Result<()> {
        let mut file = File::create(path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}