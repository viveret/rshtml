use std::rc::Rc;

use proc_macro2::{ Ident, TokenTree };

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;

use super::irusthtml_directive::IRustHtmlDirective;


// The "else if" directive is used to render a section of the view if the previous "if" or "else if" directive evaluated to false.
pub struct ElseIfDirective {}

impl ElseIfDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for ElseIfDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "else if"
    }

    fn execute(self: &Self, _identifier: &Ident, _ident_token: &TokenTree, _parser: Rc<dyn IRustToRustHtmlConverter>, _output: &mut Vec<RustHtmlToken>, _it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}