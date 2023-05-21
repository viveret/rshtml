use std::rc::Rc;

use proc_macro::Ident;

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;

use super::irusthtml_directive::IRustHtmlDirective;



// the "while" directive is used to create a while loop in the view.
// It will loop over the contents of the directive until the condition is false.
pub struct WhileDirective {}

impl WhileDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for WhileDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "while"
    }

    fn execute(self: &Self, identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        output.push(RustHtmlToken::Identifier(identifier.clone()));
        // read until we reach the loop body {}
        
        if let Ok(_) = parser.parse_for_or_while_loop_preamble(output, it, parser.get_context().get_is_raw_tokenstream()) {
            Ok(RustHtmlDirectiveResult::OkContinue)
        } else {
            return Err(RustHtmlError::from_str("Error parsing while preamble"));
        }
    }
}