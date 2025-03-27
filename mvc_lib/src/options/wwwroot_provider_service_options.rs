use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

use super::file_provider_options_base::{FileProviderOptionsBase, IFileProviderOptionsBase};
use super::special_path_options::SpecialPathOptions;

// this trait abstracts the file provider controller options.
pub trait IWwwRootProviderServiceOptions: IFileProviderOptionsBase {
}

// this struct implements IWwwRootProviderControllerOptions.
#[derive(Debug, Clone)]
pub struct WwwRootProviderServiceOptions {
    pub base: FileProviderOptionsBase
}

impl WwwRootProviderServiceOptions {
    // create a new struct from a list of directories to serve files from and a list of files to serve mapped to their aliases.
    // directories: the directories to serve files from.
    // files: the files to serve mapped to their aliases.
    // returns: a new struct.
    pub fn new(
        directories: &'static [&'static str],
        files: &'static phf::Map<&'static str, &'static str>,
        special_path_options: SpecialPathOptions
    ) -> Self {
        Self {
            base: FileProviderOptionsBase {
                directories,
                files,
                special_path_options
            }
        }
    }

    // create a new struct with default values.
    pub fn new_defaults() -> Self {
        static _EMPTY: phf::Map<&'static str, &'static str> = phf::Map::new();
        Self { base: FileProviderOptionsBase { directories: &["wwwroot/"], files: &_EMPTY, special_path_options: SpecialPathOptions::new() } }
    }

    // create a new struct as a service from a list of directories to serve files from and a list of files to serve mapped to their aliases.
    // serving_directories: the directories to serve files from.
    // serving_files: the files to serve mapped to their aliases.
    // returns: a new struct as a service.
    pub fn new_service(
        directories: &'static [&'static str],
        files: &'static phf::Map<&'static str, &'static str>,
        special_path_options: SpecialPathOptions
    ) -> Box<dyn Any> {
        Box::new(Rc::new(Self::new(directories, files, special_path_options)) as Rc<dyn IWwwRootProviderServiceOptions>)
    }

    // create a new struct as a service with default values.
    pub fn new_service_defaults() -> Box<dyn Any> {
        Box::new(Rc::new(Self::new_defaults()) as Rc<dyn IWwwRootProviderServiceOptions>)
    }
}

impl IWwwRootProviderServiceOptions for WwwRootProviderServiceOptions {
}

impl IFileProviderOptionsBase for WwwRootProviderServiceOptions {
    fn get_file(&self, path: String) -> Option<String> {
        self.base.get_file(path)
    }

    fn get_mapped_paths(&self, recursive: bool) -> HashMap<String, String> {
        self.base.get_mapped_paths(recursive)
    }

    fn list(&self, path: &str) -> std::io::Result<Vec<String>> {
        self.base.list(path)
    }
    
    fn resolve_path(&self, path: &str) -> Option<String> {
        self.base.resolve_path(path)
    }
}