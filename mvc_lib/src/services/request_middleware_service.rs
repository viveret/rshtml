use std::error::Error;
use std::rc::Rc;
use std::result::Result;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::ResponseContext;

use crate::services::service_collection::IServiceCollection;

// enum for the result of a middleware service.
pub enum MiddlewareResult {
    // the middleware service handled the request and the pipeline should continue.
    OkContinue,
    // the middleware service handled the request and the pipeline should stop.
    OkBreak,
}

// this is a trait for a class that can process an HTTP request and return an HTTP response.
// the way requests are processed is by using a pipeline of middleware services.
pub trait IRequestMiddlewareService {
    // sets the next middleware service in the pipeline.
    // next: the next middleware service.
    // returns: nothing.
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>);

    // handles the request.
    // request_ctx: the request context.
    // response_ctx: the response context.
    // services: the service collection.
    // returns: the result of the middleware service.
    fn handle_request(self: &Self, request_ctx: Rc<dyn IRequestContext>, response_ctx: Rc<ResponseContext>, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn Error>>;
}
