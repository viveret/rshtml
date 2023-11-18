use std::rc::Rc;

use core_lib::asyncly::cancellation_token::CancellationToken;
use mvc_lib::view::rusthtml::{parser_parts::{rusthtmlparser_expander::{RustHtmlParserExpander, IRustHtmlParserExpander}, peekable_tokentree::StreamPeekableTokenTree, rusthtmlparser_all::{RustHtmlParserAll, IRustHtmlParserAll}, peekable_rusthtmltoken::VecPeekableRustHtmlToken}, rusthtml_parser_context::RustHtmlParserContext};
use proc_macro2::TokenStream;


#[test]
pub fn test_rusthtmlparser_expander_expand_rust() {
    let parser = RustHtmlParserAll::new_default();

    let input = quote::quote! {
        let x = 1;
    };
    let it = Rc::new(StreamPeekableTokenTree::new(input));

    let expected = quote::quote! {
        let x = 1;
    };
    let ctx = Rc::new(RustHtmlParserContext::new(false, false, "Test".to_string()));
    let ct = Rc::new(CancellationToken::new());
    let actual_tokens = parser.get_expander().expand_rust(ctx, it, ct.clone()).unwrap();
    let actual_rust = parser.get_converter_out().convert_vec(actual_tokens, ct).unwrap();
    let actual = TokenStream::from_iter(actual_rust.into_iter());

    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
pub fn test_rusthtmlparser_expander_expand_rshtml() {
    let parser = RustHtmlParserAll::new_default();
    let ct = Rc::new(CancellationToken::new());
    let ctx = Rc::new(RustHtmlParserContext::new(false, false, "Test".to_string()));
    
    let input = quote::quote! {
        x = 1;
    };
    let it_rust = Rc::new(StreamPeekableTokenTree::new(input));
    let rshtml_vec = parser.get_converter().convert_rust(it_rust, ct.clone()).unwrap();
    let it = Rc::new(VecPeekableRustHtmlToken::new(rshtml_vec));

    let expected = quote::quote! {
        x = 1;
    };

    let actual_tokens = parser.get_expander().expand_rshtml(ctx, it, ct.clone()).unwrap();
    let actual_rust = parser.get_converter_out().convert_vec(actual_tokens, ct).unwrap();
    let actual = TokenStream::from_iter(actual_rust.into_iter());

    assert_eq!(expected.to_string(), actual.to_string());
}