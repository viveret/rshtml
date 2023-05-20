use std::any::Any;
use std::rc::Rc;

use http::StatusCode;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::ResponseContext;
use crate::contexts::controller_context::ControllerContext;

use crate::services::service_collection::IServiceCollection;

// this trait is used to convert an IActionResult to a dyn Any
pub trait IActionResultToAny: 'static {
    // convert an IActionResult to a dyn Any
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> IActionResultToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait IActionResult: IActionResultToAny {
    // get the status code of the action result
    fn get_statuscode(self: &Self) -> StatusCode;

    // configure the response based on the action result
    fn configure_response(self: &Self, controller_ctx: Rc<ControllerContext>, response_ctx: Rc<ResponseContext>, request_ctx: Rc<dyn IRequestContext>, services: &dyn IServiceCollection);
}