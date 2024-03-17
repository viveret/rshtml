use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenTree};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::rusthtml::parser_parts::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::irusthtml_directive::IRustHtmlDirective;


// the "struct" directive is used to define a struct in a view.
// It is similar to the "struct" keyword in Rust, except that it combined with other data in the view class.
pub struct StructSectionDirective {}

impl StructSectionDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for StructSectionDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "struct"
    }

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, _identifier: &Ident, _ident_token: &TokenTree, _parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, _ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // expecting group
        if let Some(group_token) = it.next() {
            match group_token {
                TokenTree::Group(group) => {
                    context.set_struct_section(Some(group.stream()));
                    Ok(RustHtmlDirectiveResult::OkContinue)
                },
                _ => {
                    Err(RustHtmlError::from_string(format!("unexpected token after struct directive: {:?}", group_token)))
                }
            }
        } else {
            Err(RustHtmlError::from_string(format!("unexpected end of input after struct directive")))
        }
    }
    
    fn execute_new(self: &Self, _context: Rc<dyn IRustHtmlParserContext>, _identifier: &Ident, _ident_token: &RustHtmlToken, _parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, _it: Rc<dyn IPeekableRustHtmlToken>, _ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        todo!("execute_new struct directive")
    }
}