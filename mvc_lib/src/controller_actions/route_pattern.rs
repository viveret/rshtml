use std::collections::HashMap;

// this struct represents a route pattern for a controller action.
// a route pattern is a string that looks like a url path, but with capture groups.
// capture groups are denoted by curly braces, and have a name and a type.
// the type is used to validate the captured value.
// for example, a route pattern might look like this:
// /users/{id:int}
// this route pattern would match the url /users/123, but not /users/abc.
// the route pattern would capture the value 123 and store it in the captures map.
// the captures map would have a key of "id" and a value of "123".
pub struct ControllerActionRoutePattern {
    // the raw string that was parsed to create this route pattern
    pub raw: String,
    // the parts of the route pattern, split by slashes
    pub parts: Vec<String>,
    // the capture groups of the route pattern, with the name of the capture group as the key and the type of the capture group as the value
    pub captures: HashMap<String, String>,
}

impl ControllerActionRoutePattern {
    pub fn to_string(self: &Self) -> String {
        self.raw.clone()
    }

    // parse a string into a route pattern.
    // s: the string to parse
    pub fn parse(s: &String) -> Self {
        let mut parts = Vec::new();
        let mut captures = HashMap::new();
        for route_dir in s.split('/') {
            // parse capture groups, like regex
            let mut route_dir_it = route_dir.chars();
            let mut str_match = String::new();
            loop {
                let c_option = route_dir_it.next();
                if let Some(c) = c_option {
                    match c {
                        '{' => {
                            if str_match.len() > 0 {
                                parts.push(str_match);
                                str_match = String::new();
                            }

                            // expect name:type
                            let mut param_name = String::new();
                            let mut param_type = String::new();
                            let mut parsing_name = true;
                            loop {
                                let name_c_option = route_dir_it.next();
                                if let Some(name_c) = name_c_option {
                                    match name_c {
                                        '}' => {
                                            break;
                                        },
                                        ':' => {
                                            if parsing_name {
                                                parsing_name = false;
                                            } else {
                                                panic!("Cannot use ':' more than once in route pattern param")
                                            }
                                        },
                                        _ => {
                                            if parsing_name {
                                                param_name.push(name_c);
                                            } else {
                                                param_type.push(name_c);
                                            }
                                        }
                                    }
                                } else {
                                    panic!("Expected next char");
                                }
                            }

                            parts.push(param_name.clone());
                            captures.insert(param_name, param_type);
                        },
                        _ => {
                            str_match.push(c);
                        }
                    }
                } else {
                    break;
                }
            }

            if str_match.len() > 0 {
                parts.push(str_match);
            }
        }
        Self { raw: s.clone(), parts: parts, captures: captures }
    }

    // generate a url from the route pattern and the route values.
    // the route values are used to fill in the capture groups.
    // for example, if the route pattern is /users/{id:int} and the route values are [("id", "123")], the generated url will be /users/123.
    // route_values: the route values to use to generate the url.
    // returns: the generated url.
    pub fn gen_url(self: &Self, route_values: &HashMap<String, String>) -> String {
        let mut result = String::new();
        result.push_str("/");
        result.push_str(self.parts.join("/").as_str());
        if route_values.len() > 0 {
            result.push_str("?");
            let mut first = true;
            for (key, value) in route_values {
                if first {
                    first = false;
                } else {
                    result.push_str("&");
                }
                result.push_str(&format!("{}={}", key, value));
            }
        }
        result
    }
}