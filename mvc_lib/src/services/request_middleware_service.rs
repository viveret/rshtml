use std::error::Error;
use std::rc::Rc;
use std::result::Result;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::ResponseContext;

use crate::services::service_collection::IServiceCollection;

pub enum MiddlewareResult {
    OkContinue,
    OkBreak,
}

pub trait IRequestMiddlewareService {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>);
    fn handle_request(self: &Self, request_ctx: Rc<dyn IRequestContext>, response_ctx: Rc<ResponseContext>, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn Error>>;
}
