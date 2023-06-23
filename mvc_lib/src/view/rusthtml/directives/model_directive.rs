use std::rc::Rc;

use proc_macro2::Ident;

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;

use super::irusthtml_directive::IRustHtmlDirective;


// The "model" directive is used to assign a model type for the view.
pub struct ModelDirective {}

impl ModelDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for ModelDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "model"
    }

    fn execute(self: &Self, identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, _output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // expecting type identifier
        if let Ok(type_ident) = parser.parse_type_identifier(it) {
            parser.get_context().set_model_type(Some(type_ident));
            Ok(RustHtmlDirectiveResult::OkContinue)
        } else {
            Err(RustHtmlError::from_string(format!("Expected type identifier after \"{}\" directive.", identifier.to_string())))
        }
    }
}