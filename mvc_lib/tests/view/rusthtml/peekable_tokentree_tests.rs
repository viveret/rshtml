use mvc_lib::view::rusthtml::parsers::peekable_tokentree::IPeekableTokenTree;
use mvc_lib::view::rusthtml::parsers::peekable_tokentree::StreamPeekableTokenTree;
use proc_macro2::TokenStream;



#[test]
pub fn peekable_tokentree_peek_empty_works() {
    let it = StreamPeekableTokenTree::new(TokenStream::new());
    assert_eq!(true, it.peek().is_none());
}

#[test]
pub fn peekable_tokentree_peek_basic_works() {
    let it = StreamPeekableTokenTree::new(quote::quote! { fn foobar() {} });
    assert_eq!(true, it.peek().is_some());
}

#[test]
pub fn peekable_tokentree_next_empty_works() {
    let it = StreamPeekableTokenTree::new(TokenStream::new());
    assert_eq!(true, it.next().is_none());
}

#[test]
pub fn peekable_tokentree_next_basic_works() {
    let it = StreamPeekableTokenTree::new(quote::quote! { fn foobar() {} });
    assert_eq!(true, it.next().is_some());
}