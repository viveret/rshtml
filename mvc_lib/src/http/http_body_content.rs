use crate::contexts::ihttpconnection_context::IHttpConnectionContext;



#[derive(Clone, Debug)]
pub struct ContentType {
    pub mime_type: String,
    pub options: String,
}

impl ContentType {
    pub fn parse(mime: &str) -> Self {
        Self {
            mime_type: mime.to_string(),
            options: "".to_string(),
        }
    }

    fn new(mime: &str) -> ContentType {
        Self {
            mime_type: mime.to_string(),
            options: "".to_string(),
        }
    }
}

pub trait IBodyContent {
    fn get_http_context(self: &Self) -> &dyn IHttpConnectionContext;
    
    fn get_self_type(self: &Self) -> ContentType;
    fn get_content_type(self: &Self) -> ContentType;
    fn get_content_length(self: &Self) -> usize;

    // returns a string representation of the body content for debugging.
    // do not use this for decoding the body content, instead use the get_body_raw method.
    fn to_string(self: &Self) -> String;
}
