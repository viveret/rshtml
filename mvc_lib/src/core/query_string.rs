use std::collections::HashMap;

// this struct holds the query string key/value pairs.
#[derive(Debug)]
pub struct QueryString {
    // the query string key/value pairs
    pub entries: HashMap<String, Vec<String>>
}

impl QueryString {
    // parse a query string into a QueryString struct.
    // query: the query string to parse.
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

    // get a value from the query string by key.
    // key: the key to get the value for.
    // returns: the value for the key if found, otherwise None.
    pub fn get(self: &Self, key: &str) -> Option<String> {
        match self.entries.get(key) {
            Some(v) => Some(v.join("\n")),
            None => None,
        }
    }

    // gets a string representation of the query string.
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
