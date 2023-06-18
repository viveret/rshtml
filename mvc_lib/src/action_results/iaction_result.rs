use std::any::Any;
use std::rc::Rc;

use http::StatusCode;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::IResponseContext;

use crate::services::service_collection::IServiceCollection;

// this trait is used to convert an IActionResult to a dyn Any
pub trait IActionResultToAny {
    // convert an IActionResult to a dyn Any
    fn as_any(&self) -> &dyn Any;

    // convert an IActionResult to a string
    fn to_string(&self) -> String;
}

impl<T: 'static + std::fmt::Debug> IActionResultToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub trait IActionResult: IActionResultToAny {
    // get the status code of the action result
    fn get_statuscode(self: &Self) -> StatusCode;

    // configure the response based on the action result
    fn configure_response(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<(), Rc<dyn std::error::Error>>;
}