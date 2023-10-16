// maybe no tests for this

use mvc_lib::view::rusthtml::parsers::peekable_rusthtmltoken::{VecPeekableRustHtmlToken, IPeekableRustHtmlToken};
use mvc_lib::view::rusthtml::rusthtml_token::RustHtmlToken;
use proc_macro2::Ident;

#[test]
pub fn peekable_rusthtmltoken_peek_empty_works() {
    let tokens = vec![];
    let it = VecPeekableRustHtmlToken::new(tokens);
    assert_eq!(true, it.peek().is_none());
}

#[test]
pub fn peekable_rusthtmltoken_peek_basic_works() {
    let tokens = vec![
        RustHtmlToken::Identifier(Ident::new("test", proc_macro2::Span::call_site())),
    ];
    let it = VecPeekableRustHtmlToken::new(tokens);
    assert_eq!(true, it.peek().is_some());
}

#[test]
pub fn peekable_rusthtmltoken_next_empty_works() {
    let tokens = vec![];
    let it = VecPeekableRustHtmlToken::new(tokens);
    assert_eq!(true, it.next().is_none());
}

#[test]
pub fn peekable_rusthtmltoken_next_basic_works() {
    let tokens = vec![
        RustHtmlToken::Identifier(Ident::new("test", proc_macro2::Span::call_site())),
    ];
    let it = VecPeekableRustHtmlToken::new(tokens);
    assert_eq!(true, it.next().is_some());
}