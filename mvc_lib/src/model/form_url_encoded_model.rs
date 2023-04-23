use crate::core::query_string::QueryString;

pub struct FormUrlEncodedModel(QueryString);
impl FormUrlEncodedModel {
    pub fn parse_body(body_content: &Vec<u8>) -> Self {
        let body_content = std::str::from_utf8(body_content).unwrap();
        Self(QueryString::parse(body_content))
    }
}