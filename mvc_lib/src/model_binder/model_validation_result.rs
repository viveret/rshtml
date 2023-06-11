use std::error::Error;
use std::rc::Rc;

use super::imodel::IModel;


// this enum represents the result of a model validation operation.
#[derive(Clone, Debug)]
pub enum ModelValidationResult<T: IModel> {
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

impl <T> std::fmt::Display for ModelValidationResult<T> where T: IModel {
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

impl <T> ModelValidationResult<T> where T: IModel {
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

    pub fn downcast<U: 'static + IModel + Clone>(self: Self) -> ModelValidationResult<U> {
        match self {
            ModelValidationResult::OkNone => ModelValidationResult::OkNone,
            ModelValidationResult::Ok(model) => {
                match downcast_inner_value::<T, U>(model) {
                    Ok(downcasted_model) => ModelValidationResult::Ok(downcasted_model),
                    Err(error) => error,
                }
            },
            ModelValidationResult::ModelError(model, error) => {
                match downcast_inner_value::<T, U>(model) {
                    Ok(downcasted_model) => ModelValidationResult::ModelError(downcasted_model, error),
                    Err(error) => error,
                }
            },
            ModelValidationResult::PropertyError(model, property_name, error) => {
                match downcast_inner_value::<T, U>(model) {
                    Ok(downcasted_model) => ModelValidationResult::PropertyError(downcasted_model, property_name, error),
                    Err(error) => error,
                }
            },
            ModelValidationResult::OtherError(error) => ModelValidationResult::OtherError(error),
        }
    }
}

fn downcast_inner_value<T, U: 'static + IModel + Clone>(model: T) -> Result<U, ModelValidationResult<U>> where T: IModel {
    match model.get_underlying_value().downcast_ref::<U>() {
        Some(downcasted_model) => Ok(downcasted_model.clone()),
        None => Err(ModelValidationResult::OtherError(Rc::new(std::io::Error::new(std::io::ErrorKind::Other, "Could not downcast model.")))),
    }
}