use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::io;

use super::icompiled_view::ICompiledView;
use super::icompiled_view_cache::ICompiledViewCache;

pub struct MemoryCompiledViewCache {
    items: Arc<Mutex<HashMap<u64, Arc<dyn ICompiledView>>>>,
    file_path_index: Arc<Mutex<HashMap<String, Vec<u64>>>>,
}

impl MemoryCompiledViewCache {
    // Creates a new empty MemoryCompiledViewCache
    pub fn new() -> Self {
        MemoryCompiledViewCache {
            items: Arc::new(Mutex::new(HashMap::new())),
            file_path_index: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl ICompiledViewCache for MemoryCompiledViewCache {
    fn get_count_of_compiled_views(&self) -> usize {
        let items = self.items.lock().unwrap();
        items.len()
    }

    fn get_count_of_compiled_views_in_file(&self, path: String) -> usize {
        let file_path_index = self.file_path_index.lock().unwrap();
        file_path_index.get(&path).map_or(0, |v| v.len())
    }

    fn get_compiled_views_in_file(&self, path: String) -> Vec<Arc<dyn ICompiledView>> {
        let file_path_index = self.file_path_index.lock().unwrap();
        let items = self.items.lock().unwrap();
        
        file_path_index.get(&path)
            .map(|hashes| {
                hashes.iter()
                    .filter_map(|hash| items.get(hash).cloned())
                    .collect()
            })
            .unwrap_or_else(Vec::new)
    }

    fn get_compiled_view_in_file_at_index(&self, path: String, index: usize) -> Option<Arc<dyn ICompiledView>> {
        let file_path_index = self.file_path_index.lock().unwrap();
        let items = self.items.lock().unwrap();
        
        file_path_index.get(&path)
            .and_then(|hashes| hashes.get(index))
            .and_then(|hash| items.get(hash))
            .cloned()
    }

    fn add_compiled_view(&self, view: Arc<dyn ICompiledView>) -> io::Result<()> {
        let file_path = view.get_file_path();
        let hash = view.get_hash();

        let mut items = self.items.lock().unwrap();
        let mut file_path_index = self.file_path_index.lock().unwrap();

        // Check for duplicates
        if items.contains_key(&hash) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("CompiledView hash {} already exists in cache", hash),
            ));
        }

        // Insert into main hash map
        items.insert(hash, view.clone());

        // Update file path index
        file_path_index.entry(file_path)
            .or_insert_with(Vec::new)
            .push(hash);

        Ok(())
    }
    
    fn get_compiled_view_by_hash(&self, hash: u64) -> Option<Arc<dyn ICompiledView>> {
        let items = self.items.lock().unwrap();
        items.get(&hash).cloned()
    }
}