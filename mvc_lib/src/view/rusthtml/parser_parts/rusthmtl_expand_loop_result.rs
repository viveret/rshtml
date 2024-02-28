use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

pub type RustHtmlExpandLoopResult = Result<bool, RustHtmlError<'static>>;