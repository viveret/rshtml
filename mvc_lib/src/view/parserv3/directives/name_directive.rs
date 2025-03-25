use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Ident;

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::parserv3::core::rusthtml_directive_result::RustHtmlDirectiveResultV3;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use super::irusthtml_directive::IRustHtmlDirective;


// The "name" directive is used to label the view for compilation into a Rust class.
// This directive is required for all views. The name must be unique and must be a valid Rust identifier.
pub struct NameDirective {}

impl NameDirective {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_service() -> Rc<dyn IRustHtmlDirective> {
        Rc::new(NameDirective::new())
    }
}

impl IRustHtmlDirective for NameDirective {
    fn matches(&self, name: &String) -> bool {
        name == "name"
    }

    fn execute_new_v3(&self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        if let Ok(param_value) = parser.get_rust_parser().parse_string_with_quotes(false, identifier, it) {
            context.insert_params(identifier.to_string().clone(), param_value);
            Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, None))
        } else {
            Err(RustHtmlError::from_string(format!("The \"name\" directive must be followed by a valid Rust identifier.")))
        }
    }
}