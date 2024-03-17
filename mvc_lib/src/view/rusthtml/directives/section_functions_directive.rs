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

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, _identifier: &Ident, _ident_token: &TokenTree, _parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, _ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // expecting group
        match it.next() {
            Some(group_token) => {
                match group_token {
                    TokenTree::Group(group) => {
                        context.set_functions_section(Some(group.stream()));
                        Ok(RustHtmlDirectiveResult::OkContinue)
                    },
                    _ => {
                        Err(RustHtmlError::from_string(format!("unexpected token after functions directive: {:?}", group_token)))
                    }
                }
            },
            None => {
                Err(RustHtmlError::from_string(format!("unexpected end of input after functions directive")))
            }
        }
    }
    
    fn execute_new(self: &Self, _context: Rc<dyn IRustHtmlParserContext>, _identifier: &Ident, _ident_token: &RustHtmlToken, _parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, _it: Rc<dyn IPeekableRustHtmlToken>, _ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        todo!("execute_new functions directive")
    }
}