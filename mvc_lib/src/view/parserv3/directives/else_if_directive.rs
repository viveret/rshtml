use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{ Ident, TokenTree };

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::RustHtmlDirectiveResultV3;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

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
    
    fn execute_new_v3(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        todo!("execute_new_v3 elseif directive")
    }
}