use std::cell::RefCell;

use http::{StatusCode, Version};
use http::{ HeaderName, HeaderValue, HeaderMap };


pub struct ResponseContext {
    pub http_version: Version,
    pub status_code: RefCell<StatusCode>,
    pub headers: RefCell<HeaderMap>,
    pub body: RefCell<Vec<u8>>,
}

impl ResponseContext {
    pub fn new(http_version: Version, status_code: StatusCode) -> Self {
        Self {
            http_version: http_version,
            status_code: RefCell::new(status_code),
            headers: RefCell::new(HeaderMap::new()),
            body: RefCell::new(vec![]),
        }
    }

    pub fn to_bytes(self: &Self) -> Vec<u8> {
        vec![]
    }

    pub fn add_header_string(self: &Self, name: String, value: String) {
        self.headers.borrow_mut().insert(HeaderName::from_bytes(name.as_bytes()).unwrap(), HeaderValue::from_bytes(value.as_bytes()).unwrap());
    }

    pub fn add_header_str(self: &Self, name: &str, value: &str) {
        self.headers.borrow_mut().insert(HeaderName::from_bytes(name.as_bytes()).unwrap(), HeaderValue::from_bytes(value.as_bytes()).unwrap());
    }

    pub fn status_message(self: &Self) -> String {
        match self.http_version {
            Version::HTTP_10 | Version::HTTP_11 => self.status_code.borrow().canonical_reason().unwrap_or("").to_string(),
            _ => "".into()
        }
    }

    pub fn get_headers(&self) -> &HeaderMap {
        todo!()
    }
}