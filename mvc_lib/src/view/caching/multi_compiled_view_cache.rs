use std::rc::Rc;
use std::io;
use std::sync::Arc;

use super::icompiled_view_cache::ICompiledViewCache;
use super::icompiled_view::ICompiledView;

pub struct MultiCompiledViewCache {
    caches: Vec<Arc<dyn ICompiledViewCache>>,
}

impl MultiCompiledViewCache {
    // Creates a new multi cache with no caches
    pub fn empty() -> Self {
        Self { caches: vec![] }
    }

    // Creates a new multi cache with the given caches
    pub fn new(caches: Vec<Arc<dyn ICompiledViewCache>>) -> Self {
        Self { caches }
    }

    // Adds a new cache to the front of the stack
    pub fn push_front(&mut self, cache: Arc<dyn ICompiledViewCache>) {
        self.caches.insert(0, cache);
    }

    // Adds a new cache to the back of the stack
    pub fn push_back(&mut self, cache: Arc<dyn ICompiledViewCache>) {
        self.caches.push(cache);
    }
}

impl ICompiledViewCache for MultiCompiledViewCache {
    fn get_count_of_compiled_views(&self) -> usize {
        // Sum counts from all caches (may include duplicates)
        self.caches.iter().map(|c| c.get_count_of_compiled_views()).sum()
    }

    fn get_count_of_compiled_views_in_file(&self, path: String) -> usize {
        // Return the first non-zero count found, or zero if none
        for cache in &self.caches {
            let count = cache.get_count_of_compiled_views_in_file(path.clone());
            if count > 0 {
                return count;
            }
        }
        0
    }

    fn get_compiled_views_in_file(&self, path: String) -> Vec<Arc<dyn ICompiledView>> {
        // Return the first non-empty result found
        for cache in &self.caches {
            let views = cache.get_compiled_views_in_file(path.clone());
            if !views.is_empty() {
                return views;
            }
        }
        Vec::new()
    }

    fn get_compiled_view_in_file_at_index(&self, path: String, index: usize) -> Option<Arc<dyn ICompiledView>> {
        // Try each cache in order until we find a result
        for cache in &self.caches {
            if let Some(view) = cache.get_compiled_view_in_file_at_index(path.clone(), index) {
                return Some(view);
            }
        }
        None
    }

    fn add_compiled_view(&self, view: Arc<dyn ICompiledView>) -> io::Result<()> {
        // Write to all caches, collecting errors
        let mut errors = Vec::new();
        
        for cache in &self.caches {
            if let Err(e) = cache.add_compiled_view(view.clone()) {
                errors.push(e);
            }
        }

        if !errors.is_empty() {
            // Return the first error if any occurred
            Err(errors.remove(0))
        } else {
            Ok(())
        }
    }
    
    fn get_compiled_view_by_hash(&self, hash: u64) -> Option<Arc<dyn ICompiledView>> {
        // Try each cache in order until we find a result
        for cache in &self.caches {
            if let Some(view) = cache.get_compiled_view_by_hash(hash) {
                return Some(view);
            }
        }
        None
    }
}