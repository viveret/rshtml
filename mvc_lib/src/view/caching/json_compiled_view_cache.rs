use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use super::icompiled_view_cache::ICompiledViewCache;
use super::icompiled_view::{CompiledView, ICompiledView};

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
struct JsonCache {
    views: Vec<CompiledView>,
}

pub struct JsonCompiledViewCache {
    file_path: PathBuf,
    cache: Arc<Mutex<JsonCache>>,
}

impl JsonCompiledViewCache {
    // Creates a new cache that will store data in the specified JSON file
    pub fn new(file_path: impl Into<PathBuf>) -> Result<Self> {
        let file_path = file_path.into();
        let cache = if file_path.exists() {
            let mut file = File::open(&file_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            serde_json::from_str(&contents).map_err(|e| {
                Error::new(ErrorKind::InvalidData, e)
            })?
        } else {
            JsonCache::default()
        };

        Ok(Self { file_path, cache: Arc::new(Mutex::new(cache)) })
    }

    // Saves the cache to disk
    fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.cache().clone())?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    // Converts an ICompiledView to JsonView
    fn view_to_json(view: &Arc<dyn ICompiledView>) -> CompiledView {
        CompiledView {
            file_path: view.get_file_path(),
            index_in_file: view.get_index_in_file(),
            hash: view.get_hash(),
            source_code: view.try_get_source_code(),
            output_code: view.try_get_output_code(),
        }
    }

    // Converts a JsonView to a trait object (requires a concrete implementation)
    fn json_to_view(json: &CompiledView) -> Arc<dyn ICompiledView> {
        Arc::new(CompiledView::new(
            json.file_path.clone(),
            json.index_in_file,
            json.hash,
            json.source_code.clone(),
            json.output_code.clone(),
        ))
    }
    
    fn cache(&self) -> std::sync::MutexGuard<'_, JsonCache> {
        self.cache.lock().unwrap()
    }
}

impl ICompiledViewCache for JsonCompiledViewCache {
    fn get_count_of_compiled_views(&self) -> usize {
        self.cache().views.len()
    }

    fn get_count_of_compiled_views_in_file(&self, path: String) -> usize {
        self.cache().views
            .iter()
            .filter(|v| v.file_path == path)
            .count()
    }

    fn get_compiled_views_in_file(&self, path: String) -> Vec<Arc<dyn ICompiledView>> {
        self.cache().views
            .iter()
            .filter(|v| v.file_path == path)
            .map(|v| Self::json_to_view(v))
            .collect()
    }

    fn get_compiled_view_in_file_at_index(&self, path: String, index: usize) -> Option<Arc<dyn ICompiledView>> {
        self.cache().views
            .iter()
            .filter(|v| v.file_path == path)
            .nth(index)
            .map(|v| Self::json_to_view(v))
    }

    fn add_compiled_view(&self, view: Arc<dyn ICompiledView>) -> Result<()> {
        // Check for duplicates
        if self.cache().views.iter().any(|v| 
            v.file_path == view.get_file_path() && 
            v.hash == view.get_hash()
            // v.index_in_file == view.get_index_in_file()
        ) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("CompiledView at {} {} {} already exists in cache", view.get_file_path(), view.get_index_in_file(), view.get_hash()),
            ));
        }

        // Add to cache
        self.cache().views.push(Self::view_to_json(&view));

        // Save to disk
        self.save()
    }
    
    fn get_compiled_view_by_hash(&self, hash: u64) -> Option<Arc<dyn ICompiledView>> {
        self.cache().views
            .iter()
            .filter(|v| v.hash == hash)
            .map(|v| Self::json_to_view(v))
            .nth(0)
    }
}