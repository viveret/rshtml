use std::any::Any;
use std::error::Error;
use std::rc::Rc;

use crate::core::type_info::TypeInfo;

use crate::contexts::request_context::RequestContext;
use crate::contexts::response_context::ResponseContext;

use crate::services::request_middleware_service::MiddlewareResult;
use crate::services::service_collection::IServiceCollection;



pub trait IControllerActionFeature {
    fn get_type_info(self: &Self) -> TypeInfo;
    fn get_name(self: &Self) -> String;
    fn to_string(self: &Self) -> String;

    fn invoke(self: &Self, request_context: Rc<RequestContext>, response_ctx: Rc<ResponseContext>, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn Error>>;


    fn as_any(&self) -> &dyn Any;
}