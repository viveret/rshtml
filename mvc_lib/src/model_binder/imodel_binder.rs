use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;

use super::imodel::AnyIModel;
use super::model_validation_result::ModelValidationResult;


// this trait is used to determine if a given IModelBinder can bind a given request.
// it is used by the ModelBinderResolverMiddleware between the RequestDecoderMiddleware and the ControllerMiddleware.
pub trait IModelBinder {
    // get the type info for the view model binder.
    fn type_info(self: &Self) -> Box<TypeInfo>;

    // whether or not this IModelBinder can bind the given request.'
    // request_context: the request context to check if this IModelBinder can bind.
    // returns: true if this IModelBinder can bind the given request, otherwise false.
    fn matches(self: &Self, request_context: &dyn IRequestContext) -> bool;

    // bind and validate the model for the given request context.
    // request_context: the request context to bind and validate the model for.
    // returns: the result of the binding and validation.
    fn bind_model(self: &Self, request_context: &dyn IRequestContext) -> ModelValidationResult<AnyIModel>;
}