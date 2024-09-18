use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenTree};

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::{RustHtmlDirectiveResult, RustHtmlDirectiveResultV3};
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::irusthtml_directive::IRustHtmlDirective;



// The "functions" directive is used to define functions that can be used in the view.
// it is similar to the "functions" keyword in a Razor in C#.
pub struct FunctionsSectionDirective {}

impl FunctionsSectionDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for FunctionsSectionDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "functions"
    }
    
    fn execute_new_v3(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        if let Some(token) = it.next() {
            match token {
                RustHtmlToken::Group(d, s, g) => {
                    let s_out = parser.get_converter_out().convert(s, ct)?;
                    context.set_functions_section(Some(s_out.to_stream()));
                    Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, None))
                }
                _ => {
                    Err(RustHtmlError::from_string(format!("unexpected token after functions directive: {:?}", token)))
                }
            }
        } else {
            Err(RustHtmlError::from_string(format!("unexpected end of input after functions directive")))
        }
    }
}