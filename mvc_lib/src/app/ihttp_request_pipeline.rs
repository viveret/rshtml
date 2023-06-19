use std::rc::Rc;
use std::error::Error;

use crate::services::service_collection::IServiceCollection;
use crate::contexts::ihttpconnection_context::IHttpConnectionContext;



// this is a trait for a class that can process an HTTP request and return an HTTP response.
// the way requests are processed is by using a pipeline of middleware services.
pub trait IHttpRequestPipeline {
    fn process_request(self: &Self, connection_context: &dyn IHttpConnectionContext, services: &dyn IServiceCollection) -> Result<(), Rc<dyn Error>>;
}