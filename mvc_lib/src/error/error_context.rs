use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::IResponseContext;

use super::ierror_context::IErrorContext;



pub struct ErrorContext<'a> {
    error: Rc<dyn Error>,
    handled: RefCell<bool>,
    request_context: &'a dyn IRequestContext,
    response_context: &'a dyn IResponseContext,
}

impl<'a> ErrorContext<'a> {
    pub fn new(
        error: Rc<dyn Error>,
        request_context: &'a dyn IRequestContext,
        response_context: &'a dyn IResponseContext,
    ) -> Self {
        Self {
            error,
            handled: RefCell::new(false),
            request_context,
            response_context,
        }
    }

    pub fn set_handled(&self) {
        self.handled.replace(true);
    }

    pub fn is_handled(&self) -> bool {
        *self.handled.borrow()
    }
}

impl<'a> IErrorContext for ErrorContext<'a> {
    fn get_error(self: &Self) -> Rc<dyn Error> {
        self.error.clone()
    }

    fn get_request_context(self: &Self) -> &dyn IRequestContext {
        self.request_context
    }

    fn get_response_context(self: &Self) -> &dyn IResponseContext {
        self.response_context
    }
}