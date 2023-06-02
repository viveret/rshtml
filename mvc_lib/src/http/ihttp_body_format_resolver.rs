use std::rc::Rc;

use crate::http::http_body_content::ContentType;

use super::ihttp_body_stream_format::IHttpBodyStreamFormat;


pub trait IHttpBodyFormatResolver {
    fn resolve(&self, content_type: &ContentType) -> Option<Rc<dyn IHttpBodyStreamFormat>>;
    // fn wrap(&self, content_type: ContentType, inner: BufferedTcpStream) -> Option<Rc<dyn ITcpStreamWrapper>>;
}