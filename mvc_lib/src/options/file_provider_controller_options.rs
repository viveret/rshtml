use std::any::Any;
use std::path::Path;
use std::rc::Rc;

pub trait IFileProviderControllerOptions {
    fn get_file(self: &Self, path: String) -> Option<String>;
}

#[derive(Debug, Clone)]
pub struct FileProviderControllerOptions {
    pub serving_directories: &'static [&'static str],
    pub serving_files: &'static phf::Map<&'static str, &'static str>,
}

impl FileProviderControllerOptions {
    pub fn new(
        serving_directories: &'static [&'static str],
        serving_files: &'static phf::Map<&'static str, &'static str>
    ) -> Self {
        Self {
            serving_directories: serving_directories,
            serving_files: serving_files,
        }
    }

    pub fn new_defaults() -> Self {
        static _EMPTY: phf::Map<&'static str, &'static str> = phf::Map::new();
        Self { serving_directories: &["wwwroot/"], serving_files: &_EMPTY }
    }

    pub fn new_service(
        serving_directories: &'static [&'static str],
        serving_files: &'static phf::Map<&'static str, &'static str>
    ) -> Box<dyn Any> {
        Box::new(Rc::new(Self::new(serving_directories, serving_files)) as Rc<dyn IFileProviderControllerOptions>)
    }

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
}
