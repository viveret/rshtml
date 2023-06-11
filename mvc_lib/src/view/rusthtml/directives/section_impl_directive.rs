use std::rc::Rc;

use proc_macro::{Ident, TokenTree};

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
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

    fn execute(self: &Self, _identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, _output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // expecting group
        match it.next() {
            Some(group_token) => {
                match group_token {
                    TokenTree::Group(group) => {
                        parser.get_context().set_impl_section(Some(group.stream()));
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
}