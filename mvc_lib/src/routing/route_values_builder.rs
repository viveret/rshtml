use std::collections::HashMap;



pub struct RouteValuesBuilder {
    pub route_values: HashMap<String, String>,
}

impl RouteValuesBuilder {
    pub fn new() -> Self {
        Self {
            route_values: HashMap::new(),
        }
    }

    pub fn new_id(id: &str) -> Self {
        let mut route_values = HashMap::new();
        route_values.insert("id".to_string(), id.to_string());
        Self {
            route_values: route_values,
        }
    }

    pub fn new_default(id: &str) -> Self {
        let mut route_values = HashMap::new();
        route_values.insert("_".to_string(), id.to_string());
        Self {
            route_values: route_values,
        }
    }

    pub fn build(self: &Self) -> HashMap<String, String> {
        self.route_values.clone()
    }

    pub fn build_id(id: &str) -> HashMap<String, String> {
        let mut route_values = HashMap::new();
        route_values.insert("id".to_string(), id.to_string());
        route_values
    }

    pub fn build_default(id: &str) -> HashMap<String, String> {
        let mut route_values = HashMap::new();
        route_values.insert("_".to_string(), id.to_string());
        route_values
    }

    pub fn build_area(id: &str) -> HashMap<String, String> {
        let mut route_values = HashMap::new();
        route_values.insert("..".to_string(), id.to_string());
        route_values
    }
}