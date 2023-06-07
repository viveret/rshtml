use mvc_lib::{auth::auth_role_json_file_dbset::JsonAuthRole, model_binder::imodel::IModel};


// this is the view model for a validation result
pub struct ViewModelValidationResult {
    // whether or not there are errors
    pub has_errors: bool,
    // the message to display to the user if there are errors
    pub message: String,
}

impl ViewModelValidationResult {
    // create a new instance of the view model validation result
    // has_errors: whether or not there are errors
    // message: the message to display to the user if there are errors
    pub fn new(has_errors: bool, message: String) -> Self {
        Self {
            has_errors: has_errors,
            message: message
        }
    }
}





// this is the view model for the add role view
pub struct AddViewModel {
    // the role to add
    pub role: String,
    // the validation result
    pub validation_result: Option<Box<ViewModelValidationResult>>
}

impl AddViewModel {
    // create a new instance of the view model
    // role: the role to add
    // validation_result: the validation result
    pub fn new(role: String, validation_result: Option<Box<ViewModelValidationResult>>) -> Self {
        Self { role: role, validation_result: validation_result }
    }

    // create a new instance of the view model with an error
    // role: the role to add
    // message: the message to display to the user
    pub fn new_ok(role: String, message: &str) -> Self {
        Self::new(role, Some(Box::new(ViewModelValidationResult::new(false, message.to_string()))))
    }
    
    // create a new instance of the view model with an error
    // role: the role to add
    // message: the message to display to the user
    pub fn new_error(role: String, message: &str) -> Self {
        Self::new(role, Some(Box::new(ViewModelValidationResult::new(true, message.to_string()))))
    }
}

impl IModel for AddViewModel {
    fn get_properties(&self) -> std::collections::HashMap<String, Box<dyn std::any::Any>> {
        todo!()
    }

    fn get_property(&self, name: &str) -> Option<Box<dyn std::any::Any>> {
        todo!()
    }

    fn get_attributes(&self) -> Vec<Box<dyn std::any::Any>> {
        todo!()
    }

    fn get_attribute(&self, typeinfo: &mvc_lib::core::type_info::TypeInfo) -> Option<Box<dyn std::any::Any>> {
        todo!()
    }

    fn get_type_info(&self) -> Box<mvc_lib::core::type_info::TypeInfo> {
        todo!()
    }

    fn get_underlying_value(&self) -> &dyn std::any::Any {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }
}