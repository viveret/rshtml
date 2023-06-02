use std::rc::Rc;

use crate::core::{type_info::TypeInfo, itcp_stream_wrapper::ITcpStreamWrapper};

use super::http_body_content::{ContentType};



// // transforms a stream of bytes from one format to another
pub trait IHttpBodyStreamFormat {
    fn type_info(self: &Self) -> Box<TypeInfo>;
    fn matches_content_type(self: &Self, content_type: &ContentType) -> bool;
    fn decode(self: &Self, stream: Rc<dyn ITcpStreamWrapper>, content_type: &ContentType) -> Rc<dyn ITcpStreamWrapper>;
    fn encode(self: &Self, stream: Rc<dyn ITcpStreamWrapper>, content_type: &ContentType) -> Rc<dyn ITcpStreamWrapper>;
}