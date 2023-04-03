use std::collections::HashMap;


#[derive(Clone, Debug)]
pub struct RouteData {
    pub map: HashMap<String, String>,
}

impl RouteData {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}