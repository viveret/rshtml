use std::error::Error;
use std::fmt;


// this struct represents an error that occurs when a HTTP request is made.
#[derive(Debug)]
pub struct RequestError(pub String);
impl Error for RequestError {}
impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

