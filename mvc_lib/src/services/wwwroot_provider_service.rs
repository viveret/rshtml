use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter, Result};
use std::rc::Rc;
use std::any::Any;

use crate::core::type_info::TypeInfo;
use crate::options::wwwroot_provider_service_options::IWwwRootProviderServiceOptions;
use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions};

use super::service_descriptor::ServiceDescriptor;
use super::service_scope::ServiceScope;

// this is a trait for a class that can provide file services.
pub trait IWwwRootProviderService {
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

    // get the file path for a given path.
    // path: the path to get the file path for.
    fn get_file(&self, path: String) -> Option<String>;

    // get the mapped paths with the alias as the key and the path as the value.
    // recursive: whether to get the paths recursively.
    fn get_mapped_paths(&self, recursive: bool) -> HashMap<String, String>;
}

// implementation of the file provider service.
pub struct WwwRootProviderService {
    options: Vec<Rc<dyn IWwwRootProviderServiceOptions>>,

}

impl WwwRootProviderService {
    // creates a new instance of the file provider service.
    pub fn new(options: Vec<Rc<dyn IWwwRootProviderServiceOptions>>) -> Self {
        Self {
            options
        }
    }

    // creates the file provider service as a service.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_multiple::<dyn IWwwRootProviderServiceOptions>(services)
        )) as Rc<dyn IWwwRootProviderService>)]
    }

    // adds the file provider service to the given service collection.
    pub fn add_to_services(services: &mut super::service_collection::ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IWwwRootProviderService>(), WwwRootProviderService::new_service, ServiceScope::Singleton));
    }
}

impl IWwwRootProviderService for WwwRootProviderService {
    fn open_read(&self, path: &str) -> Result<Box<dyn Read>> {
        let paths = self.get_mapped_paths(true);
        let mapped_path = paths.get(&path.to_string());
        if let Some(path) = mapped_path {
            let file = File::open(path)?;
            Ok(Box::new(BufReader::new(file)))
        } else {
            std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("could not find {}", path)))
        }
    }

    fn open_write(&self, path: &str) -> Result<Box<dyn Write>> {
        let file = File::open(path)?;
        Ok(Box::new(BufWriter::new(file)))
    }

    fn read_string(&self, path: &str) -> Result<String> {
        Ok(std::fs::read_to_string(path)?)
    }
    
    fn write_string(&self, path: &str, data: &String) -> Result<()> {
        let mut file = File::create(path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
    
    fn list(&self, path: &str) -> Result<Vec<String>> {
        Ok(
            // self.options.iter()
            //     .map(|x| x.list(path))
            //     .filter_map(|x| x.ok())
            //     .flatten()
            //     .collect()
            self.get_mapped_paths(true)
            .keys().into_iter().cloned().collect()
        )
    }
    
    fn get_file(&self, path: String) -> Option<String> {
        for option in self.options.iter() {
            if let Some(f) = option.get_file(path.clone()) {
                return Some(f);
            }
        }

        None
    }
    
    fn get_mapped_paths(&self, recursive: bool) -> HashMap<String, String> {
        self.options.iter()
            .map(|x| x.get_mapped_paths(recursive))
            .flat_map(|x| x.iter().map(|x| (x.0.clone(), x.1.clone())).collect::<Vec<(String, String)>>())
            .collect()
    }
}