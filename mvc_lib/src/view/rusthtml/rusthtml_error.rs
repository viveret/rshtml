use std::borrow::Cow;
use std::error::Error;
use std::fmt;


// this struct is used to represent an error that occurs while parsing RustHTML.
#[derive(Debug, Clone)]
pub struct RustHtmlError<'a>(pub Cow<'a, str>);
impl <'a> Error for RustHtmlError<'a> {}
impl <'a> fmt::Display for RustHtmlError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl <'a> RustHtmlError<'a> {
    pub fn from_str(s: &'a str) -> RustHtmlError<'a> {
        return Self(Cow::Borrowed(s));
    }
    
    pub fn from_string(s: String) -> RustHtmlError<'a> {
        return Self(Cow::Owned(s));
    }
}