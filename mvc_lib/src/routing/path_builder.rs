use super::action_path::ActionPath;





pub struct ActionPathBuilder {
    pub path: String,
}

impl ActionPathBuilder {
    pub fn new() -> Self {
        Self {
            path: String::new()
        }
    }

    pub fn add(self: &mut Self, path_part: &str, required: bool) -> &mut Self {
        if path_part.is_empty() {
            if required {
                panic!("path_part is empty");
            } else {

            }
        } else if self.path.chars().last().unwrap_or_default() != '/' &&
                    path_part.chars().nth(0).unwrap_or_default() != '/' {
            self.path.push('/');
            self.path.push_str(path_part);
        } else {
            self.path.push_str(path_part);
        }
        self
    }

    pub fn add_optional(self: &mut Self, path_part: Option<&str>) -> &mut Self {
        if let Some(path_part) = path_part {
            self.add(path_part, false);
        }
        self
    }

    pub fn as_action_path(self: &mut Self) -> ActionPath {
        ActionPath(self.path.clone())
    }

    pub fn as_string(self: &mut Self) -> String {
        self.path.clone()
    }

    pub fn as_str(self: &mut Self) -> &str {
        self.path.as_str()
    }
}