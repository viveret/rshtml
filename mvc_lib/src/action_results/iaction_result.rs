use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use http::StatusCode;

use crate::contexts::request_context::RequestContext;
use crate::contexts::response_context::ResponseContext;
use crate::contexts::controller_context::ControllerContext;

use crate::services::service_collection::IServiceCollection;


pub trait IActionResultToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> IActionResultToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait IActionResult: IActionResultToAny {
    fn get_statuscode(self: &Self) -> StatusCode;

    fn configure_response(self: &Self, controller_ctx: Rc<RefCell<ControllerContext>>, response_ctx: Rc<RefCell<ResponseContext>>, request_ctx: Rc<RequestContext>, services: &dyn IServiceCollection);
}