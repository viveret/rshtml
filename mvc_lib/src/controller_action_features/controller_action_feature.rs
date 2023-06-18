use std::any::Any;
use std::error::Error;
use std::rc::Rc;

use crate::core::type_info::TypeInfo;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::ResponseContext;

use crate::services::request_middleware_service::MiddlewareResult;
use crate::services::service_collection::IServiceCollection;


// this trait represents a controller action feature.
// a controller action feature is a feature that can be applied to a controller action.
// controller action features are used to add functionality to controller actions.
// some examples include:
// - authorization / authentication
// - logging
// - caching
// - metrics
// - rate limiting
pub trait IControllerActionFeature {
    // get the type info for the controller action feature.
    fn get_type_info(self: &Self) -> TypeInfo;
    // get the name of the controller action feature.
    fn get_name(self: &Self) -> String;
    // get the string representation of the controller action feature.
    fn to_string(self: &Self) -> String;

    // invoke the controller action feature on the controller action and return a middleware result.
    // request_context: the request context for the controller action.
    // response_context: the response context for the controller action.
    // services: the service collection for the controller action.
    // returns: the middleware result for the controller action or an error.
    fn invoke(self: &Self, request_context: Rc<dyn IRequestContext>, response_context: Rc<ResponseContext>, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Rc<dyn Error>>;

    // get the controller action feature as an Any for downcasting.
    fn as_any(&self) -> &dyn Any;
}