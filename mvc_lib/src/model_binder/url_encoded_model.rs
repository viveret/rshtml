use std::rc::Rc;

use crate::http::http_body_content::{ContentType, IBodyContent};
use crate::core::query_string::QueryString;

use super::imodel::IModel;


// this struct is used to parse the body content of a form url encoded request.
// it is used by the FormUrlEncodedBinder.
pub struct UrlEncodedModel(pub QueryString);
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

impl IModel for UrlEncodedModel {
    fn get_properties(&self) -> std::collections::HashMap<String, Box<dyn std::any::Any>> {
        todo!()
    }

    fn get_property(&self, name: &str) -> Option<Box<dyn std::any::Any>> {
        todo!()
    }

    fn get_attributes(&self) -> Vec<Box<dyn std::any::Any>> {
        todo!()
    }

    fn get_attribute(&self, typeinfo: &crate::core::type_info::TypeInfo) -> Option<Box<dyn std::any::Any>> {
        todo!()
    }

    fn get_type_info(&self) -> Box<crate::core::type_info::TypeInfo> {
        todo!()
    }

    fn get_underlying_value(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }
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