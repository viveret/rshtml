use std::rc::Rc;

use core_lib::asyncly::cancellation_token::CancellationToken;
use mvc_lib::view::rusthtml::parser_parts::{rusthtmlparser_rust::{RustHtmlParserRust, IRustHtmlParserRust}, peekable_tokentree::{StreamPeekableTokenTree, VecPeekableTokenTree}};
use proc_macro2::{Ident, TokenTree};


#[test]
pub fn test_rusthtmlparser_rust_constructor_works() {
    let _ = RustHtmlParserRust::new();
}

#[test]
pub fn test_rusthtmlparser_rust_parse_string_with_quotes_works() {
    let ident = Ident::new("test", proc_macro2::Span::call_site());

    let parser = RustHtmlParserRust::new();
    let it = Rc::new(VecPeekableTokenTree::new(vec![]));

    let result = parser.parse_string_with_quotes(true, &ident, it).unwrap();
    assert_eq!(result, "test".to_string());
}

#[test]
pub fn test_rusthtmlparser_rust_parse_type_identifier_works() {
    let ident = Ident::new("test", proc_macro2::Span::call_site());
    let token = TokenTree::Ident(ident.clone());

    let parser = RustHtmlParserRust::new();
    let it = Rc::new(VecPeekableTokenTree::new(vec![token]));
    let ct = Rc::new(CancellationToken::new());

    let result = parser.parse_type_identifier(it, ct).unwrap();

    let token_actual = result.next().unwrap();
    match token_actual {
        TokenTree::Ident(ident_actual) => {
            assert_eq!(ident_actual, ident);
        },
        _ => {
            panic!("Expected Ident");
        }
    }
}

#[test]
pub fn test_rusthtmlparser_rust_parse_identifier_expression_works() {
    let ident = Ident::new("test", proc_macro2::Span::call_site());
    let first_token = TokenTree::Ident(ident.clone());

    let parser = RustHtmlParserRust::new();
    let it = Rc::new(VecPeekableTokenTree::new(vec![first_token.clone()]));
    let ct = Rc::new(CancellationToken::new());

    let result = parser.parse_rust_identifier_expression(true, &first_token, true, it, ct).unwrap();

    let token_actual = result.next().unwrap();
    match token_actual {
        TokenTree::Ident(ident_actual) => {
            assert_eq!(ident_actual, ident);
        },
        _ => {
            panic!("Expected Ident");
        }
    }
}

#[test]
pub fn test_rusthtmlparser_rust_expect_punct_works() {
    let token = proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone);
    let parser = RustHtmlParserRust::new();
    let it = Rc::new(VecPeekableTokenTree::new(vec![TokenTree::Punct(token)]));
    let result = parser.expect_punct(',', it).unwrap();
    assert_eq!(result.1.as_char(), ',');
}