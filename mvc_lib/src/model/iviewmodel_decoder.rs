use std::any::Any;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;

use super::view_model_result::ViewModelResult;

// this trait is used to determine if a given IViewModelDecoder can decode a given content type and to decode the view model.
pub trait IViewModelDecoder {
    // whether or not this IViewModelDecoder can decode the given content type.
    // content_type: the content type to check.
    // returns: true if this IViewModelDecoder can decode the given content type, otherwise false.
    fn matches_content_type(self: &Self, content_type: &str) -> bool;

    // decodes the view model for the given request context.
    // request_context: the request context to decode the view model for.
    // returns: the decoded view model result.
    fn decode_model(self: &Self, request_context: Rc<dyn IRequestContext>) -> ViewModelResult<Rc<dyn Any>>;
}