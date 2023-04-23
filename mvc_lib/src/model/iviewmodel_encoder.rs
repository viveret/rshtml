use std::{rc::Rc, any::Any};

use crate::contexts::response_context::ResponseContext;

use super::view_model_result::ViewModelResult;

pub trait IViewModelEncoder {
    fn matches_content_type(self: &Self, content_type: &str) -> bool;

    fn encode_model(self: &Self, model: Box<dyn Any>, response_context: Rc<ResponseContext>) -> ViewModelResult<Vec<u8>>;
}