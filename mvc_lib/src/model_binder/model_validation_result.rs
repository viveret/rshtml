use std::error::Error;
use std::rc::Rc;


// this enum represents the result of a model validation operation.
#[derive(Clone, Debug)]
pub enum ModelValidationResult<T> {
    // this value indicates that the model was successfully validated.
    OkNone,
    // this value indicates that the model was successfully validated and the validated model is returned.
    Ok(T),
    // this value indicates that the model was not successfully validated and the error is returned.
    ModelError(T, Rc<dyn Error>),
    // this value indicates that a property of the model was not successfully validated and the error is returned.
    PropertyError(T, String, Rc<dyn Error>),
}

impl <T> std::fmt::Display for ModelValidationResult<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelValidationResult::OkNone => write!(f, "OkNone"),
            ModelValidationResult::Ok(_) => write!(f, "Ok"),
            ModelValidationResult::ModelError(_, _) => write!(f, "ModelError"),
            ModelValidationResult::PropertyError(_, _, _) => write!(f, "PropertyError"),
        }
    }
}