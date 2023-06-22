use std::rc::Rc;

use proc_macro::Ident;

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;

use super::irusthtml_directive::IRustHtmlDirective;


// The "lang" directive is used to set the RustHTML language version of the view.
pub struct LangDirective {}

impl LangDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for LangDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "lang" || name == "lang_ver" || name == "lang_version"
    }

    fn execute(self: &Self, _identifier: &Ident, _parser: Rc<dyn IRustToRustHtmlConverter>, _output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, _it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}