use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenTree, TokenStream};

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::vec_peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::{RustHtmlDirectiveResult, RustHtmlDirectiveResultV3};
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};

use super::irusthtml_directive::IRustHtmlDirective;


// The "use" directive is used to import a namespace or type into the view. it is similar to the "use" keyword in Rust.
pub struct UseDirective {}

impl UseDirective {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_service() -> Rc<dyn IRustHtmlDirective> {
        Rc::new(UseDirective::new())
    }
}

impl IRustHtmlDirective for UseDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "use"
    }
    
    fn execute_new_v3(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, _: &RustHtmlToken, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        match parser.get_rust_parser().parse_type_identifier(it.clone(), ct.clone()) {
            Ok(type_ident_tokens) => {
                let type_ident_tokens_stream = Rc::new(VecPeekableRustHtmlToken::new(type_ident_tokens));
                match parser.get_converter_out().convert(type_ident_tokens_stream, ct) {
                    Ok(type_ident_rust_out) => {
                        let inner_tokenstream = proc_macro2::TokenStream::from(TokenStream::from_iter(type_ident_rust_out.to_stream()));
                        context.push_use_statements(TokenStream::from(quote::quote! { #identifier #inner_tokenstream; }));
                        Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, None))
                    },
                    Err(RustHtmlError(err)) => Err(RustHtmlError::from_string(err.to_string()))
                }
            },
            Err(RustHtmlError(err)) => Err(RustHtmlError::from_string(err.to_string()))
        }

    }
}