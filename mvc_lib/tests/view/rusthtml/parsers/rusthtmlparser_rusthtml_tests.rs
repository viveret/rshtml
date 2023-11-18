use std::rc::Rc;

use core_lib::asyncly::cancellation_token::CancellationToken;
use mvc_lib::view::rusthtml::parser_parts::peekable_tokentree::StreamPeekableTokenTree;
use mvc_lib::view::rusthtml::parser_parts::rusthtmlparser_all::{RustHtmlParserAll, IRustHtmlParserAll};
use mvc_lib::view::rusthtml::parser_parts::rusthtmlparser_rusthtml::RustHtmlParserRustOrHtml;
use mvc_lib::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use proc_macro2::{Ident, TokenTree};


#[test]
pub fn test_rusthtmlparser_rusthtml_constructor() {
    let _ = RustHtmlParserRustOrHtml::new();
}

#[test]
pub fn test_rusthtmlparser_rusthtml_parse_rust_or_html() {
    let parser = RustHtmlParserAll::new_default();

    let input = quote::quote! {
        let x = 1;
    };
    let it = Rc::new(StreamPeekableTokenTree::new(input));
    let ct = Rc::new(CancellationToken::new());
    
    let test_parser = parser.get_rust_or_html_parser();
    let tokens = test_parser.parse_rust_or_html(it, ct).unwrap();

    assert_eq!(tokens.len(), 4);
}

#[test]
pub fn test_rusthtmlparser_rusthtml_convert_vec() {
    let parser = RustHtmlParserAll::new_default();

    let input = quote::quote! {
        let x = 1;
    };
    let input_tokens = input.clone().into_iter().collect::<Vec<_>>();
    let ct = Rc::new(CancellationToken::new());

    let test_parser = parser.get_rust_or_html_parser();
    let tokens = test_parser.convert_vec(&input_tokens, ct);

    assert_eq!(tokens.len(), 4);
}

#[test]
pub fn test_rusthtmlparser_rusthtml_peek_path_str() {
    let parser = RustHtmlParserAll::new_default();

    let input = quote::quote! {
        let x = 1;
    };
    let it = Rc::new(StreamPeekableTokenTree::new(input));
    let ctx = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let test_parser = parser.get_rust_or_html_parser();

    let ident = Ident::new("x", proc_macro2::Span::call_site());
    let ident_token = TokenTree::Ident(ident.clone());
    let tokens = test_parser.peek_path_str(ctx, &ident, &ident_token, it).unwrap();

    assert_eq!(tokens.len(), 4);
}

#[test]
pub fn test_rusthtmlparser_rusthtml_next_path_str() {
    let parser = RustHtmlParserAll::new_default();

    let input = quote::quote! {
        let x = 1;
    };
    let it = Rc::new(StreamPeekableTokenTree::new(input));
    let ctx = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let test_parser = parser.get_rust_or_html_parser();

    let ident = Ident::new("x", proc_macro2::Span::call_site());
    let ident_token = TokenTree::Ident(ident.clone());
    let tokens = test_parser.next_path_str(ctx, &ident, &ident_token, it).unwrap();

    assert_eq!(tokens.len(), 4);
}