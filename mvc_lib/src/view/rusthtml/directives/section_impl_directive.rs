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



// The "impl" directive is used to implement a trait required by the view definition.
// it is similar to the "impl" keyword in Rust, except that the trait must be referenced by an "@implements" directive.
pub struct ImplSectionDirective {}

impl ImplSectionDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for ImplSectionDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "impl"
    }

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, _identifier: &Ident, _ident_token: &TokenTree, _parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, _ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // expecting group
        match it.next() {
            Some(group_token) => {
                match group_token {
                    TokenTree::Group(group) => {
                        context.set_impl_section(Some(group.stream()));
                        Ok(RustHtmlDirectiveResult::OkContinue)
                    },
                    _ => {
                        Err(RustHtmlError::from_string(format!("unexpected token after impl directive: {:?}", group_token)))
                    }
                }
            },
            None => {
                Err(RustHtmlError::from_string(format!("unexpected end of input after impl directive")))
            }
        }
    }
    
    fn execute_new(self: &Self, _context: Rc<dyn IRustHtmlParserContext>, _identifier: &Ident, _ident_token: &RustHtmlToken, _parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, _it: Rc<dyn IPeekableRustHtmlToken>, _ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        todo!("execute_new impl directive")
    }
}