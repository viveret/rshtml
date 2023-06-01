use std::any::Any;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;

use super::view_model_result::ViewModelResult;


// this trait is used to determine if a given IViewModelBinder can bind a given request.
// it is used by the ModelBinderResolverMiddleware between the RequestDecoderMiddleware and the ControllerMiddleware.
pub trait IViewModelBinder {
    // get the type info for the view model binder.
    fn type_info(self: &Self) -> Box<TypeInfo>;

    // whether or not this IViewModelBinder can bind the given request.'
    // request_context: the request context to check if this IViewModelBinder can bind.
    // returns: true if this IViewModelBinder can bind the given request, otherwise false.
    fn matches(self: &Self, request_context: &dyn IRequestContext) -> bool;

    // bind and validate the view model for the given request context.
    // request_context: the request context to bind and validate the view model for.
    // returns: the result of the binding and validation.
    fn bind_view_model(self: &Self, request_context: &dyn IRequestContext) -> ViewModelResult<Rc<dyn Any>>;
}