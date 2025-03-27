use std::rc::Rc;
use std::io;
use std::sync::Arc;

use super::icompiled_view_cache::ICompiledViewCache;
use super::icompiled_view::ICompiledView;

pub struct LayeredCompiledViewCache {
    primary: Arc<dyn ICompiledViewCache>,
    fallback: Arc<dyn ICompiledViewCache>,
}

impl LayeredCompiledViewCache {
    pub fn new(
        primary: Arc<dyn ICompiledViewCache>,
        fallback: Arc<dyn ICompiledViewCache>,
    ) -> Self {
        Self { primary, fallback }
    }
}

impl ICompiledViewCache for LayeredCompiledViewCache {
    fn get_count_of_compiled_views(&self) -> usize {
        // Return the count from primary, including any items it has cached
        self.primary.get_count_of_compiled_views()
    }

    fn get_count_of_compiled_views_in_file(&self, path: String) -> usize {
        // Try primary first
        let primary_count = self.primary.get_count_of_compiled_views_in_file(path.clone());
        if primary_count > 0 {
            return primary_count;
        }

        // If not found in primary, check fallback
        let fallback_count = self.fallback.get_count_of_compiled_views_in_file(path);
        if fallback_count > 0 {
            // Optionally: could populate primary here if desired
            return fallback_count;
        }

        0
    }

    fn get_compiled_views_in_file(&self, path: String) -> Vec<Arc<dyn ICompiledView>> {
        // Try primary first
        let primary_views = self.primary.get_compiled_views_in_file(path.clone());
        if !primary_views.is_empty() {
            return primary_views;
        }

        // If not found in primary, check fallback
        let fallback_views = self.fallback.get_compiled_views_in_file(path.clone());
        if !fallback_views.is_empty() {
            // Read-through: add to primary cache
            for view in &fallback_views {
                let _ = self.primary.add_compiled_view(view.clone());
            }
            return fallback_views;
        }

        Vec::new()
    }

    fn get_compiled_view_in_file_at_index(&self, path: String, index: usize) -> Option<Arc<dyn ICompiledView>> {
        // Try primary first
        if let Some(view) = self.primary.get_compiled_view_in_file_at_index(path.clone(), index) {
            return Some(view);
        }

        // If not found in primary, check fallback
        if let Some(view) = self.fallback.get_compiled_view_in_file_at_index(path.clone(), index) {
            // Read-through: add to primary cache
            let _ = self.primary.add_compiled_view(view.clone());
            return Some(view);
        }

        None
    }

    fn add_compiled_view(&self, view: Arc<dyn ICompiledView>) -> io::Result<()> {
        // Write to both caches
        let primary_result = self.primary.add_compiled_view(view.clone());
        let fallback_result = self.fallback.add_compiled_view(view);

        // Return first error if any occurred
        primary_result.and(fallback_result)
    }
    
    fn get_compiled_view_by_hash(&self, hash: u64) -> Option<Arc<dyn ICompiledView>> {
        // Try primary first
        if let Some(view) = self.primary.get_compiled_view_by_hash(hash) {
            return Some(view);
        }

        // If not found in primary, check fallback
        if let Some(view) = self.fallback.get_compiled_view_by_hash(hash) {
            // Read-through: add to primary cache
            let _ = self.primary.add_compiled_view(view.clone());
            return Some(view);
        }

        None
    }
}