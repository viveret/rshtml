use std::cell::RefCell;
use std::rc::Rc;

use http::{StatusCode, Version};
use http::{ HeaderName, HeaderValue, HeaderMap };

use crate::http::ihttp_body_stream_format::IHttpBodyStreamFormat;

use super::connection_context::{IConnectionContext, IHttpConnectionContext};
use super::irequest_context::IRequestContext;


// this trait represents a HTTP response and its context.
pub trait IResponseContext {
    // add a header to the response.
    // name: the name of the header.
    // value: the value of the header.
    fn add_header_string(self: &Self, name: String, value: String);

    // add a header to the response.
    // name: the name of the header.
    // value: the value of the header.
    fn add_header_str(self: &Self, name: &str, value: &str);

    // get the status message of the response.
    fn status_message(self: &Self) -> String;

    // get the headers of the response.
    fn get_headers(&self) -> HeaderMap;

    // get the status code of the response.
    fn get_status_code(&self) -> StatusCode;

    // set the status code of the response.
    fn set_status_code(&self, status_code: StatusCode);

    // get the request context of the response.
    fn get_request_context(&self) -> &dyn IRequestContext;

    // get the connection context of the response, same as the one for the request context.
    fn get_connection_context(&self) -> &dyn IHttpConnectionContext;

    // use an encoder for the response body.
    fn use_encoder(self: &Self, encoder: Rc<dyn IHttpBodyStreamFormat>);
}

// this trait represents a HTTP response and its context.
pub struct ResponseContext<'a> {
    // // the HTTP version of the response
    // pub http_version: Version,
    // // the status code of the response
    // pub status_code: RefCell<StatusCode>,
    // // the headers of the response
    // pub headers: RefCell<HeaderMap>,

    // the request context of the response.
    pub request_context: &'a dyn IRequestContext,
    // the connection context of the response, same as the one for the request context.
    pub connection_context: &'a dyn IHttpConnectionContext,
    // the encoders to use for the response body.
    encoders: RefCell<Vec<Rc<dyn IHttpBodyStreamFormat>>>,
}

impl <'a> ResponseContext<'a> {
    // create a new response context.
    // http_version: the HTTP version of the response.
    // status_code: the status code of the response.
    // returns: the new response context.
    pub fn new(
        request_context: &'a dyn IRequestContext,
    ) -> Self {
        let http_context = request_context.get_connection_context();

        Self {
            // http_version: http_version,
            // status_code: RefCell::new(status_code),
            // headers: RefCell::new(HeaderMap::new()),
            request_context: request_context,
            connection_context: http_context,
            encoders: RefCell::new(Vec::new()),
        }
    }
}

impl <'a> IResponseContext for ResponseContext<'a> {
    fn add_header_string(self: &Self, name: String, value: String) {
        self.connection_context.add_header_string(name, value);
    }

    fn add_header_str(self: &Self, name: &str, value: &str) {
        self.connection_context.add_header_str(name, value);
    }

    fn status_message(self: &Self) -> String {
        self.connection_context.get_pending_status_message()
    }

    fn get_headers(&self) -> HeaderMap {
        self.connection_context.get_pending_headers()
    }

    fn set_status_code(&self, status_code: StatusCode) {
        self.connection_context.set_pending_status_code(status_code);
    }

    fn get_status_code(&self) -> StatusCode {
        self.connection_context.get_pending_status_code()
    }

    fn get_request_context(&self) -> &dyn IRequestContext {
        self.request_context
    }

    fn get_connection_context(&self) -> &dyn IHttpConnectionContext {
        self.connection_context
    }

    fn use_encoder(self: &Self, encoder: Rc<dyn IHttpBodyStreamFormat>) {
        self.encoders.borrow_mut().push(encoder);
    }
}