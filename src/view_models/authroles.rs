use mvc_lib::auth::auth_role_json_file_dbset::JsonAuthRole;

pub struct IndexViewModel {
    pub roles: Vec<JsonAuthRole>
}

impl IndexViewModel {
    pub fn new(roles: Vec<JsonAuthRole>) -> Self {
        Self { roles: roles }
    }
}

pub struct ViewModelValidationResult {
    pub has_errors: bool,
    pub message: String,
}

impl ViewModelValidationResult {
    pub fn new(has_errors: bool, message: String) -> Self {
        Self {
            has_errors: has_errors,
            message: message
        }
    }
}






pub struct AddViewModel {
    pub role: String,
    pub validation_result: Option<Box<ViewModelValidationResult>>
}

impl AddViewModel {
    pub fn new(role: String, validation_result: Option<Box<ViewModelValidationResult>>) -> Self {
        Self { role: role, validation_result: validation_result }
    }

    pub fn new_ok(role: String, message: &str) -> Self {
        Self::new(role, Some(Box::new(ViewModelValidationResult::new(false, message.to_string()))))
    }
    
    pub fn new_error(role: String, message: &str) -> Self {
        Self::new(role, Some(Box::new(ViewModelValidationResult::new(true, message.to_string()))))
    }
}