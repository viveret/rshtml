use std::backtrace::Backtrace;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;


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

    pub fn from_cancellationtoken(ct: Rc<dyn ICancellationToken>) -> RustHtmlError {
        let bt = Backtrace::capture();
        return Self::from_string(format!("CancellationToken was cancelled at {}: {:?}", bt, ct.get_cancelled_reason()))
    }
}