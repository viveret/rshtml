use std::collections::HashMap;

use super::special_path_options::SpecialPathOptions;
use super::file_provider_options_base::IFileProviderOptionsBase;
use super::file_provider_options_base::FileProviderOptionsBase;


pub trait IAppContentProviderServiceOptions: IFileProviderOptionsBase {

}


#[derive(Clone)]
pub struct AppContentProviderServiceOptions {
    pub base: FileProviderOptionsBase
}

impl AppContentProviderServiceOptions {
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
}

impl IAppContentProviderServiceOptions for AppContentProviderServiceOptions {
    
}

impl IFileProviderOptionsBase for AppContentProviderServiceOptions {
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