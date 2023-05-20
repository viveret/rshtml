// this file contains the view models for the learn controller

// this is the view model for the index view
pub struct IndexViewModel {
    // this is a list of all the learn docs
    pub learn_docs: Vec<String>,
}

impl IndexViewModel {
    // create a new instance of the view model
    pub fn new(learn_docs: Vec<String>) -> Self {
        Self { learn_docs: learn_docs }
    }
}

// this is the view model for the details view
pub struct DetailsViewModel {
    // this is the path to the learn doc
    pub path: String,
}

impl DetailsViewModel {
    // create a new instance of the view model
    pub fn new(path: String) -> Self {
        Self { path: path }
    }
}
