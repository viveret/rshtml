pub struct IndexViewModel {
    pub learn_docs: Vec<String>,
}

impl IndexViewModel {
    pub fn new(learn_docs: Vec<String>) -> Self {
        Self { learn_docs: learn_docs }
    }
}

pub struct DetailsViewModel {
    pub path: String,
}

impl DetailsViewModel {
    pub fn new(path: String) -> Self {
        Self { path: path }
    }
}
