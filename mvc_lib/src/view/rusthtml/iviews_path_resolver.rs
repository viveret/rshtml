

pub trait IViewsPathResolver {
    fn get_view_paths(&self, view_name: &str) -> Vec<String>;
}