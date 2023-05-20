use std::any::Any;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;

use super::view_model_result::ViewModelResult;


// this trait is used to determine if a given IViewModelBinder can bind a given content type and to bind the view model.
// it is used by the ModelBinderResolverMiddleware.
pub trait IViewModelBinder {
    // whether or not this IViewModelBinder can bind the given content type.
    // content_type: the content type to check.
    // returns: true if this IViewModelBinder can bind the given content type, otherwise false.
    fn matches_content_type(self: &Self, content_type: &str) -> bool;

    // bind and validate the view model for the given request context.
    // request_context: the request context to bind and validate the view model for.
    // returns: the result of the binding and validation.
    fn bind_view_model(self: &Self, request_context: Rc<dyn IRequestContext>) -> ViewModelResult<Box<dyn Any>>;
}