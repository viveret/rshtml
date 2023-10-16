use std::rc::Rc;
use std::borrow::Cow;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Ident;
use proc_macro2::TokenTree;

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parsers::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::parsers::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
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

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &TokenTree, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        if let Ok(param_value) = parser.get_rust_parser().parse_string_with_quotes(false, identifier, it) {
            context.mut_params().insert(identifier.to_string().clone(), param_value);
            Ok(RustHtmlDirectiveResult::OkBreak)
        } else {
            return Err(RustHtmlError(Cow::Owned(format!("The \"name\" directive must be followed by a valid Rust identifier."))));
        }
    }
}