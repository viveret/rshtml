use std::collections::HashMap;

pub struct ControllerActionRoutePattern {
    pub raw: String,
    pub parts: Vec<String>,
    pub captures: HashMap<String, String>,
}

impl ControllerActionRoutePattern {
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

    pub fn gen_url(self: &Self, routeValues: &Vec<(String, String)>) -> String {
        let mut result = String::new();
        result.push_str("/");
        result.push_str(self.parts.join("/").as_str());
        if routeValues.len() > 0 {
            result.push_str("?");
            let mut first = true;
            for (key, value) in routeValues {
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