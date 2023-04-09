
pub trait ILogHttpRequestsOptions {
    fn get_log_request(self: &Self) -> bool;
    fn get_log_response(self: &Self) -> bool;
    fn get_log_request_headers(self: &Self) -> bool;
    fn get_log_response_headers(self: &Self) -> bool;
    fn get_log_request_cookies(self: &Self) -> bool;
    fn get_log_response_cookies(self: &Self) -> bool;
}

pub struct LogHttpRequestsOptions {
    pub log_request: bool,
    pub log_response: bool,

    pub log_request_headers: bool,
    pub log_response_headers: bool,

    pub log_request_cookies: bool,
    pub log_response_cookies: bool,
}

impl ILogHttpRequestsOptions for LogHttpRequestsOptions {
    fn get_log_request(self: &Self) -> bool {
        self.log_request
    }

    fn get_log_response(self: &Self) -> bool {
        self.log_response
    }

    fn get_log_request_headers(self: &Self) -> bool {
        self.log_request_headers
    }

    fn get_log_response_headers(self: &Self) -> bool {
        self.log_response_headers
    }

    fn get_log_request_cookies(self: &Self) -> bool {
        self.log_request_cookies
    }

    fn get_log_response_cookies(self: &Self) -> bool {
        self.log_response_cookies
    }
}