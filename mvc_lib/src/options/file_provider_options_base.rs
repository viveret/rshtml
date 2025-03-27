use std::collections::HashMap;
use std::path::Path;

use glob::glob;

use super::special_path_options::SpecialPathOptions;

pub trait IFileProviderOptionsBase {
    // get the file path for a given path.
    // path: the path to get the file path for.
    fn get_file(&self, path: String) -> Option<String>;

    // get the mapped paths with the alias as the key and the path as the value.
    // recursive: whether to get the paths recursively.
    fn get_mapped_paths(&self, recursive: bool) -> HashMap<String, String>;

    // list entries in a directory
    fn list(&self, path: &str) -> std::io::Result<Vec<String>>;

    fn resolve_path(&self, path: &str) -> Option<String>;
}

#[derive(Debug, Clone)]
pub struct FileProviderOptionsBase {
    // the directories to serve files from.
    pub directories: &'static [&'static str],
    // the files to serve mapped to their aliases.
    pub files: &'static phf::Map<&'static str, &'static str>,
    // special directories to auto map
    pub special_path_options: SpecialPathOptions,
}

impl IFileProviderOptionsBase for FileProviderOptionsBase {
    fn get_file(&self, path: String) -> Option<String> {
        for (file_alias, file_path) in self.files.entries() {
            if file_alias == &path.as_str() {
                let full_path = Path::new(&file_path);
                if full_path.exists() && full_path.is_file() {
                    return Some(file_path.to_string());
                } else {
                    break; // requested files was found in known files to serve but not found on disk
                }
            }
        }

        for directory in self.directories.iter() {
            let path_string = format!("{}{}", directory, if path.starts_with("/") { &path[1..] } else { path.as_str() });
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
            self.directories
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
                    .map(|x| (x[parent_dir.len()..].to_string(), x))
                    .collect::<Vec<(String, String)>>()
            })
            .flatten()
            .chain(
                self.files.entries().map(|x| (x.0.to_string(), x.1.to_string()))
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
    
    fn resolve_path(&self, path: &str) -> Option<String> {
        let paths = self.get_mapped_paths(true);
        paths.get(path).cloned()
    }
}