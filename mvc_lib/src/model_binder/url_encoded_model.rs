use std::rc::Rc;

use crate::http::http_body_content::{ContentType, IBodyContent};
use crate::core::query_string::QueryString;

use super::iviewmodel::IViewModel;


// this struct is used to parse the body content of a form url encoded request.
// it is used by the FormUrlEncodedBinder.
pub struct UrlEncodedModel(QueryString);
impl UrlEncodedModel {
    // parse the body content of a form url encoded request.
    // body_content: the body content to parse.
    // returns: the parsed body content.
    pub fn parse_body(content_type: ContentType, body_bytes: &Vec<u8>) -> Self {
        let body_content = std::str::from_utf8(body_bytes).unwrap();
        Self(QueryString::parse(body_content))
    }

    pub fn new(body: Rc<dyn IBodyContent>) -> Self {
        let generic = body.data().downcast_ref::<Vec<u8>>().unwrap();
        Self::parse_body(body.get_content_type(), generic)
    }
}

impl IViewModel for UrlEncodedModel {

}

impl IBodyContent for UrlEncodedModel {
    fn get_content_type(self: &Self) -> ContentType {
        ContentType {
            mime_type: "application/x-www-form-urlencoded".to_string(),
            options: "".to_string(),
        }
    }

    fn get_content_length(self: &Self) -> usize {
        self.0.to_string().len()
    }

    fn get_self_type(self: &Self) -> ContentType {
        ContentType {
            mime_type: "application/x-www-form-urlencoded".to_string(),
            options: "".to_string(),
        }
    }

    fn data(self: &Self) -> &dyn std::any::Any {
        &self.0
    }

    fn to_string(self: &Self) -> String {
        format!("UrlEncodedModel: {:?}", self.0)
    }
}