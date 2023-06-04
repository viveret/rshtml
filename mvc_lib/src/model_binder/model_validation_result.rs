use std::error::Error;
use std::rc::Rc;

use as_any::Downcast;


// this enum represents the result of a model validation operation.
#[derive(Clone, Debug)]
pub enum ModelValidationResult<T: 'static + Downcast + Clone> {
    // this value indicates that the model was successfully validated.
    OkNone,
    // this value indicates that the model was successfully validated and the validated model is returned.
    Ok(T),
    // this value indicates that the model was not successfully validated and the error is returned.
    ModelError(T, Rc<dyn Error>),
    // this value indicates that a property of the model was not successfully validated and the error is returned.
    PropertyError(T, String, Rc<dyn Error>),
    // this value indicates something else went wrong and the model could not be validated.
    OtherError(Rc<dyn Error>),
}

impl <T> std::fmt::Display for ModelValidationResult<T> where T: 'static + Downcast + Clone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelValidationResult::OkNone => write!(f, "OkNone"),
            ModelValidationResult::Ok(_) => write!(f, "Ok"),
            ModelValidationResult::ModelError(_, _) => write!(f, "ModelError"),
            ModelValidationResult::PropertyError(_, _, _) => write!(f, "PropertyError"),
            ModelValidationResult::OtherError(_) => write!(f, "OtherError"),
        }
    }
}

impl <T> ModelValidationResult<T> where T: 'static + Downcast + Clone {
    // get whether or not the model was successfully validated.
    pub fn is_ok(self: &Self) -> bool {
        match self {
            ModelValidationResult::OkNone => true,
            ModelValidationResult::Ok(_) => true,
            ModelValidationResult::ModelError(_, _) => false,
            ModelValidationResult::PropertyError(_, _, _) => false,
            ModelValidationResult::OtherError(_) => false,
        }
    }

    // get whether or not the model was not successfully validated.
    pub fn is_err(self: &Self) -> bool {
        !self.is_ok()
    }

    pub fn downcast<U: 'static + Downcast + Clone>(self: Self) -> ModelValidationResult<U> {
        match self {
            ModelValidationResult::OkNone => ModelValidationResult::OkNone,
            ModelValidationResult::Ok(model) => {
                match model.downcast_ref::<U>() {
                    None => ModelValidationResult::OtherError(Rc::new(std::io::Error::new(std::io::ErrorKind::Other, "Could not downcast model."))),
                    Some(downcasted_model) => ModelValidationResult::Ok(downcasted_model.clone()),
                }
            },
            ModelValidationResult::ModelError(model, error) => ModelValidationResult::ModelError(model.downcast_ref::<U>().unwrap().clone(), error),
            ModelValidationResult::PropertyError(model, property_name, error) => ModelValidationResult::PropertyError(model.downcast_ref::<U>().unwrap().clone(), property_name, error),
            ModelValidationResult::OtherError(error) => ModelValidationResult::OtherError(error),
        }
    }
}