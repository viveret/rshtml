use std::{borrow::Cow, rc::Rc};

use proc_macro::{Ident, TokenTree};

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
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

    fn execute(self: &Self, identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // expecting group
        if let Some(group_token) = it.next() {
            match group_token {
                TokenTree::Group(group) => {
                    parser.get_context().set_struct_section(Some(group.stream()));
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
}