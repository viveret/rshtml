use std::sync::Arc;
use std::io::Result;

use super::icompiled_view::ICompiledView;



pub trait ICompiledViewCache: Send + Sync {
    fn get_count_of_compiled_views(&self) -> usize;
    fn get_count_of_compiled_views_in_file(&self, path: String) -> usize;
    fn get_compiled_views_in_file(&self, path: String) -> Vec<Arc<dyn ICompiledView>>;
    fn get_compiled_view_in_file_at_index(&self, path: String, index: usize) -> Option<Arc<dyn ICompiledView>>;
    fn get_compiled_view_by_hash(&self, hash: u64) -> Option<Arc<dyn ICompiledView>>;

    fn add_compiled_view(&self, view: Arc<dyn ICompiledView>) -> Result<()>;
}