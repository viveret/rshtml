use std::collections::HashMap;

pub struct QueryString {
    pub entries: HashMap<String, Vec<String>>
}

impl QueryString {
    pub fn parse(query: &str) -> Self {
        // println!("query: {}", query);
        Self {
            entries: querystring::querify(query)
                        .iter()
                        .map(|kvp| (
                            kvp.0.to_string(),
                            kvp.1.split('&')
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>())
                        ).collect::<HashMap<_,_>>()
        }
    }

    pub fn get(self: &Self, key: &str) -> Option<String> {
        match self.entries.get(key) {
            Some(v) => Some(v.join("\n")),
            None => None,
        }
    }

    pub fn to_string(self: &Self) -> String {
        let mut result = String::new();
        for (key, values) in &self.entries {
            for value in values {
                result.push_str(&format!("{}={}&", key, value));
            }
        }
        result.pop();
        result
    }
}
