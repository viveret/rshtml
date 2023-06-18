use std::rc::Rc;
use std::error::Error;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::IResponseContext;


// context for an error that is being handled by an error handler.
pub trait IErrorContext {
    fn get_error(self: &Self) -> Rc<dyn Error>;
    fn get_request_context(self: &Self) -> Option<&dyn IRequestContext>;
    fn get_response_context(self: &Self) -> Option<&dyn IResponseContext>;
}