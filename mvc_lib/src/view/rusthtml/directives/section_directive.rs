use std::rc::Rc;

use proc_macro::Ident;

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::irusthtml_directive::IRustHtmlDirective;


// The "section" directive is used to label a section of the view.
pub struct SectionDirective {}

impl SectionDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for SectionDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "section"
    }

    fn execute(self: &Self, _identifier: &Ident, _parser: Rc<dyn IRustToRustHtmlConverter>, _output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, _it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}