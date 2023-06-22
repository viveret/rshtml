use std::rc::Rc;

use proc_macro::{Ident, TokenStream};

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;

use super::irusthtml_directive::IRustHtmlDirective;


// The "use" directive is used to import a namespace or type into the view. it is similar to the "use" keyword in Rust.
pub struct UseDirective {}

impl UseDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for UseDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "use"
    }

    fn execute(self: &Self, _: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, _: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // expecting type identifier
        if let Ok(type_ident_tokens) = parser.parse_type_identifier(it) {
            let inner_tokenstream = proc_macro2::TokenStream::from(TokenStream::from_iter(type_ident_tokens));
            parser.get_context().mut_use_statements().push(TokenStream::from(quote::quote! { use #inner_tokenstream; }));
            Ok(RustHtmlDirectiveResult::OkContinue)
        } else {
            Err(RustHtmlError::from_str("Error parsing use directive"))
        }
    }
}