use std::{borrow::Cow, rc::Rc};

use proc_macro::{Ident, TokenTree};

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;

use super::irusthtml_directive::IRustHtmlDirective;


// The "md" directive is used to render markdown from a string.
pub struct MarkdownDirective {}

impl MarkdownDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for MarkdownDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "md" || name == "markdown"
    }

    fn execute(self: &Self, identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}