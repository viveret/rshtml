use crate::core::query_string::QueryString;


// this struct is used to parse the body content of a form url encoded request.
// it is used by the FormUrlEncodedBinder.
pub struct FormUrlEncodedModel(QueryString);
impl FormUrlEncodedModel {
    // parse the body content of a form url encoded request.
    // body_content: the body content to parse.
    // returns: the parsed body content.
    pub fn parse_body(body_content: &Vec<u8>) -> Self {
        let body_content = std::str::from_utf8(body_content).unwrap();
        Self(QueryString::parse(body_content))
    }
}