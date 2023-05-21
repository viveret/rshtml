use std::rc::Rc;
use std::borrow::Cow;

use proc_macro::Ident;

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::irusthtml_directive::IRustHtmlDirective;


// The "if" directive is used to conditionally render a section of the view.
pub struct IfDirective {}

impl IfDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for IfDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "if"
    }

    fn execute(self: &Self, identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        output.push(RustHtmlToken::Identifier(identifier.clone()));
        if let Ok(_) = parser.parse_for_or_while_loop_preamble(output, it, parser.get_context().get_is_raw_tokenstream()) {
            Ok(RustHtmlDirectiveResult::OkContinue)
        } else {
            return Err(RustHtmlError::from_str("Error parsing if preamble"));
        }
    }
}