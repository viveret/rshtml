use std::any::Any;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;

use super::view_model_result::ViewModelResult;

pub trait IViewModelDecoder {
    fn matches_content_type(self: &Self, content_type: &str) -> bool;

    fn decode_model(self: &Self, request_context: Rc<dyn IRequestContext>) -> ViewModelResult<Rc<dyn Any>>;
}