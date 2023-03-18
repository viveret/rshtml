use std::any::Any;
use std::path::Path;
use std::rc::Rc;

pub trait IFileProviderControllerOptions {
    fn get_file(self: &Self, path: String) -> Option<String>;
}


pub struct FileProviderControllerOptions {
    pub serving_paths: Vec<String>,
}

impl FileProviderControllerOptions {
    pub fn new(serving_paths: Vec<String>) -> Self {
        Self { serving_paths: serving_paths }
    }

    pub fn new_defaults() -> Self {
        Self { serving_paths: vec!["wwwroot/".to_string()] }
    }

    pub fn new_service(serving_paths: Vec<String>) -> Rc<dyn Any> {
        Rc::new(Box::new(Self::new(serving_paths)) as Box<dyn IFileProviderControllerOptions>)
    }

    pub fn new_service_defaults() -> Rc<dyn Any> {
        Rc::new(Box::new(Self::new_defaults()) as Box<dyn IFileProviderControllerOptions>)
    }
}

impl IFileProviderControllerOptions for FileProviderControllerOptions {
    fn get_file(self: &Self, path: String) -> Option<String> {
        for serving_path in self.serving_paths.iter() {
            let path_string = format!("{}{}", serving_path, if path.starts_with("/") { &path[1..] } else { path.as_str() });
            let full_path = Path::new(&path_string);
            if full_path.exists() && full_path.is_file() {
                return Some(path_string);
            }
        }
        return None;
    }
}
