use std::rc::Rc;
use std::borrow::Cow;

use proc_macro::Ident;

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{irust_to_rusthtml_converter::IRustToRustHtmlConverter, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;

use super::irusthtml_directive::IRustHtmlDirective;


// The "name" directive is used to label the view for compilation into a Rust class.
// This directive is required for all views. The name must be unique and must be a valid Rust identifier.
pub struct NameDirective {}

impl NameDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for NameDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "name"
    }

    fn execute(self: &Self, identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, _output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        if let Ok(param_value) = parser.parse_string_with_quotes(identifier.clone(), it) {
            parser.get_context().mut_params().insert(identifier.to_string().clone(), param_value);
            Ok(RustHtmlDirectiveResult::OkBreak)
        } else {
            return Err(RustHtmlError(Cow::Owned(format!("The \"name\" directive must be followed by a valid Rust identifier."))));
        }
    }
}