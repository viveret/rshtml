use super::special_path_options::SpecialPathOptions;


#[derive(Clone)]
pub struct AppContentProviderServiceOptions {
    pub special_path_options: SpecialPathOptions
}

impl AppContentProviderServiceOptions {
    pub fn new() -> Self {
        Self {
            special_path_options: SpecialPathOptions::new()
        }
    }
}

