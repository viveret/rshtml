use std::cell::RefCell;
use std::iter::Peekable;
use std::slice::Iter;

use super::rusthtml_token::RustHtmlToken;


// this is used to peek at the next token in a RustHtml token stream.
pub trait IPeekableRustHtmlToken {
    fn peek(self: &Self) -> Option<&RustHtmlToken>;
    fn next(self: &Self) -> Option<&RustHtmlToken>;
}

pub struct PeekableRustHtmlToken<'a> {
    it: RefCell<Peekable<Iter<'a, RustHtmlToken>>>,
}

impl <'a> PeekableRustHtmlToken<'a> {
    pub fn new(iter: &'a Vec<RustHtmlToken>) -> Self {
        Self {
            it: RefCell::new(iter.iter().peekable()),
        }
    }
}

impl <'a> IPeekableRustHtmlToken for PeekableRustHtmlToken<'a> {
    fn peek(self: &Self) -> Option<&RustHtmlToken> {
        self.it.borrow_mut().peek().cloned()
    }

    fn next(self: &Self) -> Option<&RustHtmlToken> {
        self.it.borrow_mut().next()
    }
}
