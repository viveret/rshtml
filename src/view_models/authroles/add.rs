use std::collections::HashMap;
use std::rc::Rc;

use mvc_lib::core::type_info::TypeInfo;

use mvc_lib::model_binder::imodel::IModel;
use core_macro_lib::{IModel, IHazAttributes, reflect_attributes, reflect_properties, reflect_methods};
use mvc_lib::model_binder::ihaz_attributes::IHazAttributes;
use mvc_lib::model_binder::imodel_attribute::IAttribute;
use mvc_lib::model_binder::imodel_property::IModelProperty;
use mvc_lib::model_binder::imodel_method::IModelMethod;
use mvc_lib::model_binder::reflected_attribute::ReflectedAttribute;
use mvc_lib::model_binder::reflected_property::ReflectedProperty;
use mvc_lib::model_binder::reflected_method::ReflectedMethod;


// this is the view model for a validation result
#[derive(Clone)]
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
#[derive(Clone, IHazAttributes, IModel)]
#[reflect_attributes]
#[reflect_properties]
pub struct AddViewModel {
    // the role to add
    pub role: String,
    // the validation result
    pub validation_result: Option<ViewModelValidationResult>
}

#[reflect_methods]
impl AddViewModel {
    // create a new instance of the view model
    // role: the role to add
    // validation_result: the validation result
    pub fn new(role: String, validation_result: Option<ViewModelValidationResult>) -> Self {
        Self { role: role, validation_result: validation_result }
    }

    // create a new instance of the view model with an error
    // role: the role to add
    // message: the message to display to the user
    pub fn new_ok(role: String, message: &str) -> Self {
        Self::new(role, Some(ViewModelValidationResult::new(false, message.to_string())))
    }
    
    // create a new instance of the view model with an error
    // role: the role to add
    // message: the message to display to the user
    pub fn new_error(role: String, message: &str) -> Self {
        Self::new(role, Some(ViewModelValidationResult::new(true, message.to_string())))
    }
}
