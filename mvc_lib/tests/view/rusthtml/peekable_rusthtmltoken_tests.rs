// maybe no tests for this

use mvc_lib::view::rusthtml::peekable_rusthtmltoken::{PeekableRustHtmlToken, IPeekableRustHtmlToken};

#[test]
pub fn peekable_rusthtmltoken_peek_works() {
    let tokens = vec![];
    let it = PeekableRustHtmlToken::new(&tokens);
    it.peek().unwrap();
}

#[test]
pub fn peekable_rusthtmltoken_next_works() {
    let tokens = vec![];
    let it = PeekableRustHtmlToken::new(&tokens);
    it.next().unwrap();
}