
// this trait abstracts the options for logging HTTP requests.
pub trait ILogHttpRequestsOptions {
    // whether to log the request
    fn get_log_request(&self) -> bool;
    // whether to log the response
    fn get_log_response(&self) -> bool;
    // whether to log the request headers
    fn get_log_request_headers(&self) -> bool;
    // whether to log the response headers
    fn get_log_response_headers(&self) -> bool;
    // whether to log the request cookies
    fn get_log_request_cookies(&self) -> bool;
    // whether to log the response cookies
    fn get_log_response_cookies(&self) -> bool;
}

// this struct implements ILogHttpRequestsOptions.
pub struct LogHttpRequestsOptions {
    pub log_request: bool,
    pub log_response: bool,

    pub log_request_headers: bool,
    pub log_response_headers: bool,

    pub log_request_cookies: bool,
    pub log_response_cookies: bool,
}

impl ILogHttpRequestsOptions for LogHttpRequestsOptions {
    fn get_log_request(&self) -> bool {
        self.log_request
    }

    fn get_log_response(&self) -> bool {
        self.log_response
    }

    fn get_log_request_headers(&self) -> bool {
        self.log_request_headers
    }

    fn get_log_response_headers(&self) -> bool {
        self.log_response_headers
    }

    fn get_log_request_cookies(&self) -> bool {
        self.log_request_cookies
    }

    fn get_log_response_cookies(&self) -> bool {
        self.log_response_cookies
    }
}



pub trait ILoggingOptions {

}

#[derive(Clone)]
pub struct LoggingOptions {

}

impl LoggingOptions {

}

impl ILoggingOptions for LoggingOptions {
    
}