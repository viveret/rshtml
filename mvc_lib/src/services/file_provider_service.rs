use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter, Result};
use std::rc::Rc;
use std::any::Any;

use crate::services::service_collection::IServiceCollection;

pub trait IFileProviderService {
    fn open_read(self: &Self, path: &str) -> Result<Box<dyn Read>>;
    fn open_write(self: &Self, path: &str) -> Result<Box<dyn Write>>;

    fn read_string(self: &Self, path: &str) -> Result<String>;
    fn write_string(self: &Self, path: &str, data: &String) -> Result<()>;
}

pub struct FileProviderService {
    // fn open_read(path: &str) -> Read,
    // fn open_write(path: &str) -> Write,
}

impl FileProviderService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Rc<dyn Any>> {
        vec![Rc::new(Self::new())]
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