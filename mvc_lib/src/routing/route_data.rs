use std::collections::HashMap;

// this is a struct that holds the route data (action parameters, name, controller name, area name, etc.)
#[derive(Clone, Debug)]
pub struct RouteData {
    // this is a map of the route data
    pub map: HashMap<String, String>,
}

impl RouteData {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}