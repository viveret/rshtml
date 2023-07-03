use mvc_lib::view::rusthtml::peekable_tokentree::{PeekableTokenTree, IPeekableTokenTree};
use proc_macro2::TokenStream;



#[test]
pub fn peekable_tokentree_peek_works() {
    let it = PeekableTokenTree::new(TokenStream::new());
    it.peek().unwrap();
}

#[test]
pub fn peekable_tokentree_next_works() {
    let it = PeekableTokenTree::new(TokenStream::new());
    it.next().unwrap();
}