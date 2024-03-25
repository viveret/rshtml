use std::error::Error;
use std::fmt;


// this struct is used to represent an error that occurs while parsing RustHTML.
#[derive(Debug, Clone)]
pub struct RustHtmlError(pub String);
impl Error for RustHtmlError {}
impl fmt::Display for RustHtmlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl RustHtmlError {
    pub fn from_str(s: &str) -> RustHtmlError {
        return Self(s.to_string());
    }
    
    pub fn from_string(s: String) -> RustHtmlError {
        return Self(s);
    }
}