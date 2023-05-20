use std::any::Any;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use glob::glob;

// this trait abstracts the file provider controller options.
pub trait IFileProviderControllerOptions {
    // get the file path for a given path.
    // path: the path to get the file path for.
    fn get_file(self: &Self, path: String) -> Option<String>;

    // get the mapped paths with the alias as the key and the path as the value.
    // recursive: whether to get the paths recursively.
    fn get_mapped_paths(self: &Self, recursive: bool) -> HashMap<String, String>;
}

// this struct implements IFileProviderControllerOptions.
#[derive(Debug, Clone)]
pub struct FileProviderControllerOptions {
    // the directories to serve files from.
    pub serving_directories: &'static [&'static str],
    // the files to serve mapped to their aliases.
    pub serving_files: &'static phf::Map<&'static str, &'static str>,
}

impl FileProviderControllerOptions {
    // create a new FileProviderControllerOptions struct from a list of directories to serve files from and a list of files to serve mapped to their aliases.
    // serving_directories: the directories to serve files from.
    // serving_files: the files to serve mapped to their aliases.
    // returns: a new FileProviderControllerOptions struct.
    pub fn new(
        serving_directories: &'static [&'static str],
        serving_files: &'static phf::Map<&'static str, &'static str>
    ) -> Self {
        Self {
            serving_directories: serving_directories,
            serving_files: serving_files,
        }
    }

    // create a new FileProviderControllerOptions struct with default values.
    pub fn new_defaults() -> Self {
        static _EMPTY: phf::Map<&'static str, &'static str> = phf::Map::new();
        Self { serving_directories: &["wwwroot/"], serving_files: &_EMPTY }
    }

    // create a new FileProviderControllerOptions struct as a service from a list of directories to serve files from and a list of files to serve mapped to their aliases.
    // serving_directories: the directories to serve files from.
    // serving_files: the files to serve mapped to their aliases.
    // returns: a new FileProviderControllerOptions struct as a service.
    pub fn new_service(
        serving_directories: &'static [&'static str],
        serving_files: &'static phf::Map<&'static str, &'static str>
    ) -> Box<dyn Any> {
        Box::new(Rc::new(Self::new(serving_directories, serving_files)) as Rc<dyn IFileProviderControllerOptions>)
    }

    // create a new FileProviderControllerOptions struct as a service with default values.
    pub fn new_service_defaults() -> Box<dyn Any> {
        Box::new(Rc::new(Self::new_defaults()) as Rc<dyn IFileProviderControllerOptions>)
    }
}

impl IFileProviderControllerOptions for FileProviderControllerOptions {
    fn get_file(self: &Self, path: String) -> Option<String> {
        for (serving_file_alias, serving_file_path) in self.serving_files.entries() {
            // println!("FileProviderControllerOptions comparing serving_file_alias {} to path {}", serving_file_alias, path);
            if serving_file_alias == &path.as_str() {
                let full_path = Path::new(&serving_file_path);
                // println!("FileProviderControllerOptions full_path: {}", serving_file_path);
                if full_path.exists() && full_path.is_file() {
                    // println!("full_path.exists() && full_path.is_file()");
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

    fn get_mapped_paths(self: &Self, recursive: bool) -> HashMap<String, String> {
        let all_paths = self.serving_directories
            .iter()
            .map(|path| {
                let cwd = std::env::current_dir().unwrap();
                let parent_dir = format!("{}/{}", cwd.to_str().unwrap(), path);
                let mut glob_path = String::new();
                glob_path.push_str(&parent_dir);
                glob_path.push_str(if recursive { "**/*" } else { "*" });

                // println!("glob_path: {}", glob_path);

                glob(&glob_path)
                    .expect("Failed to read glob pattern")
                    .map(|x| x.unwrap().to_str().unwrap().to_string())
                    .map(|x| (x[parent_dir.len() - 1..].to_string(), x))
                    .collect::<Vec<(String, String)>>()
            })
            .flatten()
            .chain(
                self.serving_files.entries().map(|x| (x.0.to_string(), x.1.to_string()))
            )
            .collect();

        all_paths
    }
}
