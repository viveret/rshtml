use std::rc::Rc;
use std::error::Error;

use super::ierror_context::IErrorContext;


pub trait IErrorHandler {
    fn handle_error(self: &Self, error_context: &dyn IErrorContext) -> Result<bool, Rc<dyn Error>>;
}