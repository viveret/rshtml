
use std::rc::Rc;
use regex::Regex;


pub struct RequestHeader {
    pub name: Box<String>,
    pub value: Box<Vec<u8>>,
}

impl RequestHeader {
    pub fn new(name: Box<String>, value: Box<Vec<u8>>) -> Self {
        Self { name: name, value: value }
    }

    pub fn from_httparse_header(name: &str, value: &[u8]) -> Self {
        Self::new(Box::new(name.to_string()), Box::new(value.to_vec()))
    }
}

pub struct RequestContext {
    pub http_version: http::version::Version,
    pub method: Box<String>,
    pub path: Box<String>,
    pub headers: Box<Vec<RequestHeader>>,
    pub body: Vec<u8>,
}

impl RequestContext {
    pub fn new(http_version: http::version::Version, method: Box<String>, path: Box<String>,
                request_headers: Box<Vec<RequestHeader>>) -> Self {
        Self { http_version: http_version, method: method, path: path, headers: request_headers, body: vec![] }
    }

    pub fn parse(http_header: String, headers: Vec<String>, _request_bytes: Box<Vec<u8>>) -> Rc<RequestContext> {
        let re_method: Regex = Regex::new(r"^(GET|POST|PUT) ").unwrap();
        let re_version: Regex = Regex::new(r" HTTP/(\d\.\d)$").unwrap();
        let re_header: Regex = Regex::new(r"^([a-zA-Z0-9 _-]+): ").unwrap();

        println!("Incoming request: {}", http_header);
        let method_str = re_method.find(&http_header).expect("HTTP method not found").as_str().trim();
        let version_str = re_version.find(&http_header).expect("HTTP version not found").as_str().trim();
        let version = match version_str {
            "HTTP/0.9" => http::version::Version::HTTP_09,
            "HTTP/1.0" => http::version::Version::HTTP_10,
            "HTTP/1.1" => http::version::Version::HTTP_11,
            _ => panic!("Invalid HTTP version {}", version_str)
        };

        let path = &http_header[method_str.len() + 1 .. http_header.len() - version_str.len() - 1].trim();

        Rc::new(Self::new(
            version,
            Box::new(method_str.to_string()),
            Box::new(path.to_string()),
            Box::new(headers.iter().map(|x| {
                let name = re_header.find(x).expect(&format!("Invalid header format: {}", x)).as_str();
                let value = x[name.len()..].to_string();
                RequestHeader::from_httparse_header(name, &value.as_bytes().to_vec())
            }).collect())
        ))
    }
}