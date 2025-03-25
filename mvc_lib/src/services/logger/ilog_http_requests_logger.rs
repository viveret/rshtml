use http::HeaderMap;



pub trait ILogHttpRequestsLogger {
    fn log_request_info(&self, version: http::version::Version, method: &http::method::Method, path: &String);
    fn log_request_headers(&self, path: &String, headers: &HeaderMap);
    fn log_request_cookies(&self, path: &String, headers: &HeaderMap);
}