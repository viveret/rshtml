use std::any::Any;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use glob::glob;

use super::special_path_options::SpecialPathOptions;

// this trait abstracts the file provider controller options.
pub trait IWwwRootProviderServiceOptions {
        // get the file path for a given path.
    // path: the path to get the file path for.
    fn get_file(&self, path: String) -> Option<String>;

    // get the mapped paths with the alias as the key and the path as the value.
    // recursive: whether to get the paths recursively.
    fn get_mapped_paths(&self, recursive: bool) -> HashMap<String, String>;

    // list entries in a directory
    fn list(&self, path: &str) -> std::io::Result<Vec<String>>;
}

// this struct implements IFileProviderControllerOptions.
#[derive(Debug, Clone)]
pub struct WwwRootProviderServiceOptions {
    // the directories to serve files from.
    pub serving_directories: &'static [&'static str],
    // the files to serve mapped to their aliases.
    pub serving_files: &'static phf::Map<&'static str, &'static str>,
    pub special_path_options: SpecialPathOptions,
}

impl WwwRootProviderServiceOptions {
    // create a new WwwRootProviderServiceOptions struct from a list of directories to serve files from and a list of files to serve mapped to their aliases.
    // serving_directories: the directories to serve files from.
    // serving_files: the files to serve mapped to their aliases.
    // returns: a new WwwRootProviderServiceOptions struct.
    pub fn new(
        serving_directories: &'static [&'static str],
        serving_files: &'static phf::Map<&'static str, &'static str>,
        special_path_options: SpecialPathOptions
    ) -> Self {
        Self {
            serving_directories: serving_directories,
            serving_files: serving_files,
            special_path_options
        }
    }

    // create a new FileProviderControllerOptions struct with default values.
    pub fn new_defaults() -> Self {
        static _EMPTY: phf::Map<&'static str, &'static str> = phf::Map::new();
        Self { serving_directories: &["wwwroot/"], serving_files: &_EMPTY, special_path_options: SpecialPathOptions::new() }
    }

    // create a new FileProviderControllerOptions struct as a service from a list of directories to serve files from and a list of files to serve mapped to their aliases.
    // serving_directories: the directories to serve files from.
    // serving_files: the files to serve mapped to their aliases.
    // returns: a new FileProviderControllerOptions struct as a service.
    pub fn new_service(
        serving_directories: &'static [&'static str],
        serving_files: &'static phf::Map<&'static str, &'static str>,
        special_path_options: SpecialPathOptions
    ) -> Box<dyn Any> {
        Box::new(Rc::new(Self::new(serving_directories, serving_files, special_path_options)) as Rc<dyn IWwwRootProviderServiceOptions>)
    }

    // create a new FileProviderControllerOptions struct as a service with default values.
    pub fn new_service_defaults() -> Box<dyn Any> {
        Box::new(Rc::new(Self::new_defaults()) as Rc<dyn IWwwRootProviderServiceOptions>)
    }
}

impl IWwwRootProviderServiceOptions for WwwRootProviderServiceOptions {
    fn get_file(&self, path: String) -> Option<String> {
        for (serving_file_alias, serving_file_path) in self.serving_files.entries() {
            if serving_file_alias == &path.as_str() {
                let full_path = Path::new(&serving_file_path);
                if full_path.exists() && full_path.is_file() {
                    return Some(serving_file_path.to_string());
                } else {
                    break; // requested files was found in known files to serve but not found on disk
                }
            }
        }

        for serving_directory in self.serving_directories.iter() {
            let path_string = format!("{}{}", serving_directory, if path.starts_with("/") { &path[1..] } else { path.as_str() });
            let full_path = Path::new(&path_string);
            if full_path.exists() && full_path.is_file() {
                return Some(path_string);
            }
        }

        return None;
    }

    fn get_mapped_paths(&self, recursive: bool) -> HashMap<String, String> {
        let project_paths = self.special_path_options.get_special_paths();
        project_paths.iter().flat_map(|project_path|
            self.serving_directories
            .iter()
            .map(|path| {
                let parent_dir = format!("{}/{}", project_path, path);
                let mut glob_path = String::new();
                glob_path.push_str(&parent_dir);
                glob_path.push_str(if recursive { "**/*" } else { "*" });

                glob(&glob_path)
                    .expect("Failed to read glob pattern")
                    .map(|x| x.expect("Failed to read glob pattern entry"))
                    .map(|x| x.to_str().expect("x.to_str()").to_string())
                    .map(|x| (x[parent_dir.len() - 1..].to_string(), x))
                    .collect::<Vec<(String, String)>>()
            })
            .flatten()
            .chain(
                self.serving_files.entries().map(|x| (x.0.to_string(), x.1.to_string()))
            )
            .collect::<Vec<(String, String)>>()
        )
        .collect()
    }
    
    fn list(&self, path: &str) -> std::io::Result<Vec<String>> {

        // Ok(std::fs::read_dir(path)?.into_iter()
        //     .filter_map(|x| x.ok())
        //     .filter_map(|x| x.path().to_str().map(|x| x.to_string()))
        //     .collect()
        // )

        let paths = self.get_mapped_paths(true);
        Ok(paths.keys().into_iter()
            .filter(|x| x.starts_with(path))
            .cloned().collect::<Vec<String>>())
    }
}
