use std::error::Error;
use std::rc::Rc;

#[derive(Clone)]
pub enum ViewModelResult<T> {
    OkNone,
    Ok(T),
    ModelError(T, Rc<dyn Error>),
    PropertyError(T, String, Rc<dyn Error>),
}