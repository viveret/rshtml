use std::rc::Rc;

use proc_macro2::Ident;
use proc_macro2::TokenTree;

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parsers::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::irusthtml_directive::IRustHtmlDirective;


// The "html" directive is used to render raw html from a string.
pub struct HtmlDirective {}

impl HtmlDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for HtmlDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "rawhtml"
    }

    fn execute(self: &Self, _context: Rc<dyn IRustHtmlParserContext>, _identifier: &Ident, _ident_token: &TokenTree, _parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, _it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}