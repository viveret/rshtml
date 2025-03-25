use http::HeaderMap;
use uuid::Uuid;



pub trait ILogHttpRequestsLogger {
    fn log_request_info(&self, id: &Uuid, version: http::version::Version, method: &http::method::Method, path: &String);
    fn log_request_headers(&self, id: &Uuid, path: &String, headers: &HeaderMap);
    fn log_request_cookies(&self, id: &Uuid, path: &String, headers: &HeaderMap);
    fn log_response_info(&self, id: &Uuid, status_code: http::status::StatusCode, path: &String);
    fn log_response_headers(&self, id: &Uuid, path: &String, headers: &HeaderMap);
    fn log_response_cookies(&self, id: &Uuid, path: &String, headers: &HeaderMap);
}