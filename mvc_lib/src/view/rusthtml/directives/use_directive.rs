use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenTree, TokenStream};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parsers::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::parsers::peekable_tokentree::IPeekableTokenTree;
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

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &TokenTree, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // expecting type identifier
        if let Ok(type_ident_tokens) = parser.get_rust_parser().parse_type_identifier(it, ct) {
            let inner_tokenstream = proc_macro2::TokenStream::from(TokenStream::from_iter(type_ident_tokens.to_splice().to_vec()));
            context.mut_use_statements().push(TokenStream::from(quote::quote! { use #inner_tokenstream; }));
            Ok(RustHtmlDirectiveResult::OkContinue)
        } else {
            Err(RustHtmlError::from_str("Error parsing use directive"))
        }
    }
}