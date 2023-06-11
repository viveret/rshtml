use std::rc::Rc;

use proc_macro::Ident;

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;

use super::irusthtml_directive::IRustHtmlDirective;


// The "rusthtmlfile_nocache" directive is used to include a RustHtml file without caching it.
pub struct RustHtmlFileNoCacheDirective {}

impl RustHtmlFileNoCacheDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for RustHtmlFileNoCacheDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "rusthtmlfile_nocache"
    }

    fn execute(self: &Self, _identifier: &Ident, _parser: Rc<dyn IRustToRustHtmlConverter>, _output: &mut Vec<RustHtmlToken>, _it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}