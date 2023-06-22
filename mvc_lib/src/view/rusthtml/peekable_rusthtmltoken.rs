use std::cell::RefCell;
use std::iter::Peekable;
use std::slice::Iter;

use proc_macro2::{Ident, Punct, Literal};

use super::rusthtml_token::RustHtmlToken;


// this is used to peek at the next token in a RustHtml token stream.
pub trait IPeekableRustHtmlToken {
    fn peek(self: &Self) -> Option<&RustHtmlToken<Ident, Punct, Literal>>;
    fn next(self: &Self) -> Option<&RustHtmlToken<Ident, Punct, Literal>>;
}

pub struct PeekableRustHtmlToken<'a> {
    it: RefCell<Peekable<Iter<'a, RustHtmlToken<Ident, Punct, Literal>>>>,
}

impl <'a> PeekableRustHtmlToken<'a> {
    pub fn new(iter: &'a Vec<RustHtmlToken<Ident, Punct, Literal>>) -> Self {
        Self {
            it: RefCell::new(iter.iter().peekable()),
        }
    }
}

impl <'a> IPeekableRustHtmlToken for PeekableRustHtmlToken<'a> {
    fn peek(self: &Self) -> Option<&RustHtmlToken<Ident, Punct, Literal>> {
        self.it.borrow_mut().peek().cloned()
    }

    fn next(self: &Self) -> Option<&RustHtmlToken<Ident, Punct, Literal>> {
        self.it.borrow_mut().next()
    }
}
