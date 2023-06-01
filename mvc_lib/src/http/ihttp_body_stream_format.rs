use std::rc::Rc;

use crate::core::type_info::TypeInfo;

use super::http_body_content::{IBodyContent, ContentType};



// // transforms a stream of bytes from one format to another
pub trait IHttpBodyStreamFormat {
    fn type_info(self: &Self) -> Box<TypeInfo>;
    fn matches_content_type(self: &Self, content_type: ContentType) -> bool;
    fn decode(self: &Self, body: Rc<dyn IBodyContent>) -> Rc<dyn IBodyContent>;
}