use std::any::Any;

use crate::contexts::response_context::IResponseContext;

use super::{model_validation_result::ModelValidationResult, imodel::AnyIModel};


// this trait is used to determine if a given IViewModelEncoder can encode a given content type and to encode the view model.
pub trait IViewModelEncoder {
    // whether or not this IViewModelEncoder can encode the given content type.
    // content_type: the content type to check.
    // returns: true if this IViewModelEncoder can encode the given content type, otherwise false.
    fn matches_content_type(self: &Self, content_type: &str) -> bool;

    // encodes the view model for the given response context.
    // model: the view model to encode.
    // response_context: the response context to encode the view model for.
    // returns: the encoded view model.
    fn encode_model(self: &Self, model: Box<dyn Any>, response_context: &dyn IResponseContext) -> ModelValidationResult<AnyIModel>;
}