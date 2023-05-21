use std::cell::RefCell;
use std::iter::Peekable;

use proc_macro::{TokenTree, TokenStream};


// this is used to peek at the next token in a Rust token stream.
pub trait IPeekableTokenTree {
    fn peek(self: &Self) -> Option<TokenTree>;
    fn next(self: &Self) -> Option<TokenTree>;
}

pub struct PeekableTokenTree {
    it: RefCell<Peekable<proc_macro::token_stream::IntoIter>>,
}
impl PeekableTokenTree {
    pub fn new(stream: TokenStream) -> Self {
        Self {
            it: RefCell::new(stream.into_iter().peekable()),
        }
    }
}

impl IPeekableTokenTree for PeekableTokenTree {
    fn peek(self: &Self) -> Option<TokenTree> {
        self.it.borrow_mut().peek().cloned()
    }

    fn next(self: &Self) -> Option<TokenTree> {
        self.it.borrow_mut().next()
    }
}
