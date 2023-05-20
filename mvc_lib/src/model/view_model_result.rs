use std::error::Error;
use std::rc::Rc;


// this enum represents the result of a view model validation operation.
#[derive(Clone)]
pub enum ViewModelResult<T> {
    // this value indicates that the view model was successfully validated.
    OkNone,
    // this value indicates that the view model was successfully validated and the validated view model is returned.
    Ok(T),
    // this value indicates that the view model was not successfully validated and the error is returned.
    ModelError(T, Rc<dyn Error>),
    // this value indicates that a property of the view model was not successfully validated and the error is returned.
    PropertyError(T, String, Rc<dyn Error>),
}