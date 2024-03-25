use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenTree, TokenStream};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::irusthtmlparser_version_agnostic::IRustHtmlParserVersionAgnostic;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::rusthtml::parser_parts::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;

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

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, _identifier: &Ident, _ident_token: &TokenTree, parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, _ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // expecting type identifier
        if let Ok(type_ident_tokens) = parser.get_old_parser().parse_type_identifier(it) {
            let inner_tokenstream = proc_macro2::TokenStream::from(TokenStream::from_iter(type_ident_tokens)); // .to_splice().to_vec()
            context.push_use_statements(TokenStream::from(quote::quote! { use #inner_tokenstream; }));
            Ok(RustHtmlDirectiveResult::OkContinue)
        } else {
            Err(RustHtmlError::from_str("Error parsing use directive"))
        }
    }
    
    fn execute_new(self: &Self, context: Rc<dyn IRustHtmlParserContext>, _identifier: &Ident, _ident_token: &RustHtmlToken, parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        if let Ok(type_ident_tokens) = parser.get_rust_parser().parse_type_identifier(it, ct.clone()) {
            match parser.get_converter_out().convert_rusthtmltokens_to_plain_rust(type_ident_tokens, context.clone(), ct) {
                Ok(type_ident_rust_out) => {
                    let inner_tokenstream = proc_macro2::TokenStream::from(TokenStream::from_iter(type_ident_rust_out));
                    context.push_use_statements(TokenStream::from(quote::quote! { use #inner_tokenstream; }));
                    Ok(RustHtmlDirectiveResult::OkContinue)
                },
                Err(RustHtmlError(err)) => Err(RustHtmlError::from_string(err.to_string()))
            }
        } else {
            Err(RustHtmlError::from_str("Error parsing use directive"))
        }
    }
    
    fn execute_old(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &TokenTree, parser: Rc<crate::view::rusthtml::rusthtml_parser::RustHtmlParser>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        match parser.parser.parse_type_identifier(it.clone()) {
            Ok(type_ident_tokens) => {
                let inner_tokenstream = proc_macro2::TokenStream::from(TokenStream::from_iter(type_ident_tokens));
                context.push_use_statements(TokenStream::from(quote::quote! { use #inner_tokenstream; }));
                Ok(RustHtmlDirectiveResult::OkContinue)
            },
            Err(RustHtmlError(err)) => Err(RustHtmlError::from_string(err.to_string()))
        }
    }
}