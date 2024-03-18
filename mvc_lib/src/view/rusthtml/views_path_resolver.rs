use super::iviews_path_resolver::IViewsPathResolver;


pub struct RegularViewsPathResolver {
    pub views_path_dir: String,
}

impl RegularViewsPathResolver {
    pub fn new(views_path_dir: String) -> Self {
        Self {
            views_path_dir,
        }
    }
}

impl IViewsPathResolver for RegularViewsPathResolver {
    fn get_view_paths(&self, view_name: &str) -> Vec<String> {
        // list of different prefixes to try
        let prefixes = vec![
            "src/views/",
            // folder,
            "src/views/shared/",
            ""
        ];

        // try each prefix
        prefixes.iter().map(
            |prefix| {
                let mut path_buf = std::path::PathBuf::new();
                path_buf.push(&self.views_path_dir);
                path_buf.push(prefix);
                path_buf.push(view_name);
                path_buf.to_str().expect("could not call to_str on path_buf in get_view_paths").to_string()
            }
        ).collect()
    }
}
pub struct DirsViewsPathResolver {
    pub views_path_dir: Vec<String>,
}

impl DirsViewsPathResolver {
    pub fn new(views_path_dir: Vec<String>) -> Self {
        Self {
            views_path_dir,
        }
    }
}

impl IViewsPathResolver for DirsViewsPathResolver {
    fn get_view_paths(&self, view_name: &str) -> Vec<String> {
        self.views_path_dir.iter().map(|dir| {
            let mut view_path = dir.clone();
            view_path.push_str(view_name);
            view_path
        }).collect()
    }
}