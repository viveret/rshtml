use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use http::StatusCode;
use http::HeaderMap;

use crate::action_results::iaction_result::IActionResult;
use crate::http::ihttp_body_stream_format::IHttpBodyStreamFormat;
use crate::services::service_collection::IServiceCollection;

use super::ihttpconnection_context::IHttpConnectionContext;
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

    // invoke the action result for the controller context by setting the status code of the response and then
    // configuring the response with the action result, and finally writing the response body.
    fn invoke_action_result(self: &Self, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<(), Rc<dyn Error>>;

    // set the status code of the response.
    fn set_status_code(&self, status_code: StatusCode);

    // get the request context of the response.
    fn get_request_context(&self) -> &dyn IRequestContext;

    // get the connection context of the response, same as the one for the request context.
    fn get_connection_context(&self) -> &dyn IHttpConnectionContext;

    // use an encoder for the response body.
    fn use_encoder(self: &Self, encoder: Rc<dyn IHttpBodyStreamFormat>);

    // get the action result for the controller context.
    fn get_action_result(self: &Self) -> Option<Rc<dyn IActionResult>>;

    // set the action result for the controller context.
    fn set_action_result(self: &Self, action_result: Option<Rc<dyn IActionResult>>);

    fn get_has_started_writing(self: &Self) -> bool;

    fn set_result_500_if_not_started_writing(self: &Self);


    // get the context data of the request
    fn get_str(self: &Self, key: &str) -> Option<String>;
    // get the context data of the request
    fn get_string(self: &Self, key: String) -> Option<String>;
    // insert a value into the context data of the request
    fn insert_str(self: &mut Self, key: &str, value: String);
    // insert a value into the context data of the request
    fn insert_string(self: &mut Self, key: String, value: String);
    // remove a value from the context data of the request
    fn remove_str(self: &mut Self, key: &str);
    // remove a value from the context data of the request
    fn remove_string(self: &mut Self, key: String);
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
    // the action result to use for the response.
    pub action_result: RefCell<Option<Rc<dyn IActionResult>>>,
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
            action_result: RefCell::new(None),
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

    fn get_action_result(self: &Self) -> Option<Rc<dyn IActionResult>> {
        match self.action_result.borrow().clone() {
            Some(action_result) => Some(action_result),
            None => None,
        }
    }

    fn set_action_result(self: &Self, action_result: Option<Rc<dyn IActionResult>>) {
        if let Some(action_result) = action_result.as_ref() {
            self.set_status_code(action_result.get_statuscode());
        }
        self.action_result.replace(action_result);
    }

    fn get_str(self: &Self, key: &str) -> Option<String> {
        None
    }

    fn get_string(self: &Self, key: String) -> Option<String> {
        None
    }

    fn insert_str(self: &mut Self, key: &str, value: String) {
    }

    fn insert_string(self: &mut Self, key: String, value: String) {
    }

    fn remove_str(self: &mut Self, key: &str) {
    }

    fn remove_string(self: &mut Self, key: String) {
    }

    fn get_has_started_writing(self: &Self) -> bool {
        self.connection_context.get_has_started_writing()
    }

    fn set_result_500_if_not_started_writing(self: &Self) {
        if !self.get_has_started_writing() && self.get_action_result().is_none() {
            println!("Writing 500 because no response was written to the client.");
            self.set_action_result(Some(Rc::new(crate::action_results::http_result::InternalServerErrorResult::new("No response was written to the client.".to_string()))));
        }
    }

    fn invoke_action_result(self: &Self, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<(), Rc<dyn Error>> {
        if let Some(action_result) = self.get_action_result() {
            self.set_status_code(action_result.get_statuscode());
            action_result.configure_response(self, request_context, services)?;
            Ok(())
        } else {
            Err(Rc::new(std::io::Error::new(std::io::ErrorKind::NotFound, "No action result was set for the response.")))
        }
    }
}