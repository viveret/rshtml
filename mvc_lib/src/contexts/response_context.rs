use http::{StatusCode, Version};

pub struct ResponseHeader {
    pub name: String,
    pub value: Box<Vec<u8>>,
}

impl ResponseHeader {
    pub fn new(name: String, value: Box<Vec<u8>>) -> Self {
        Self { name: name, value: value }
    }

    pub fn from_httparse_header(name: &str, value: &[u8]) -> Self {
        Self::new(name.to_string(), Box::new(value.to_vec()))
    }
}

pub struct ResponseContext {
    pub http_version: Version,
    pub status_code: StatusCode,
    pub headers: Vec<ResponseHeader>,
    pub body: Vec<u8>,
}

impl ResponseContext {
    pub fn new(http_version: Version, status_code: StatusCode) -> Self {
        Self { http_version: http_version, status_code: status_code, headers: vec![], body: vec![] }
    }

    pub fn to_bytes(self: &Self) -> Vec<u8> {
        vec![]
    }

    pub fn add_header_string(self: &mut Self, name: String, value: String) {
        self.headers.push(ResponseHeader::new(name, Box::new(value.as_bytes().to_vec())));
    }

    pub fn add_header_str(self: &mut Self, name: &str, value: &str) {
        self.headers.push(ResponseHeader::from_httparse_header(name, value.as_bytes()));
    }

    pub fn status_message(self: &Self) -> String {
        match self.http_version {
            Version::HTTP_10 | Version::HTTP_11 => self.status_code.canonical_reason().unwrap_or("").to_string(),
            _ => "".into()
        }
    }
}