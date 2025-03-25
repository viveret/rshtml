use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Ident;

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::RustHtmlDirectiveResultV3;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
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
    fn matches(&self, name: &String) -> bool {
        name == "rawhtml"
    }
    
    fn execute_new_v3(&self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        todo!("execute_new_v3 html directive")
    }
}