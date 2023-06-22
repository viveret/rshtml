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

    // get a header from the response.
    // name: the name of the header.
    // returns: the value of the header.
    fn get_header(self: &Self, name: &str) -> Option<String>;

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
