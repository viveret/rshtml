use std::cell::RefCell;

use http::{StatusCode, Version};
use http::{ HeaderName, HeaderValue, HeaderMap };

// this trait represents a HTTP response and its context.
pub struct ResponseContext {
    // the HTTP version of the response
    pub http_version: Version,
    // the status code of the response
    pub status_code: RefCell<StatusCode>,
    // the headers of the response
    pub headers: RefCell<HeaderMap>,
    // the body of the response
    pub body: RefCell<Vec<u8>>,
}

impl ResponseContext {
    // create a new response context.
    // http_version: the HTTP version of the response.
    // status_code: the status code of the response.
    // returns: the new response context.
    pub fn new(http_version: Version, status_code: StatusCode) -> Self {
        Self {
            http_version: http_version,
            status_code: RefCell::new(status_code),
            headers: RefCell::new(HeaderMap::new()),
            body: RefCell::new(vec![]),
        }
    }

    // return the response as bytes.
    pub fn to_bytes(self: &Self) -> Vec<u8> {
        vec![]
    }

    // add a header to the response.
    // name: the name of the header.
    // value: the value of the header.
    pub fn add_header_string(self: &Self, name: String, value: String) {
        self.headers.borrow_mut().insert(HeaderName::from_bytes(name.as_bytes()).unwrap(), HeaderValue::from_bytes(value.as_bytes()).unwrap());
    }

    // add a header to the response.
    // name: the name of the header.
    // value: the value of the header.
    pub fn add_header_str(self: &Self, name: &str, value: &str) {
        self.headers.borrow_mut().insert(HeaderName::from_bytes(name.as_bytes()).unwrap(), HeaderValue::from_bytes(value.as_bytes()).unwrap());
    }

    // get the status message of the response.
    pub fn status_message(self: &Self) -> String {
        match self.http_version {
            Version::HTTP_10 | Version::HTTP_11 => self.status_code.borrow().canonical_reason().unwrap_or("").to_string(),
            _ => "".into()
        }
    }

    // get the headers of the response.
    pub fn get_headers(&self) -> &HeaderMap {
        todo!()
    }
}