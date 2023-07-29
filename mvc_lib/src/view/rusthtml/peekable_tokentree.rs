use std::cell::RefCell;

use proc_macro2::{TokenTree, TokenStream};


// this is used to peek at the next token in a Rust token stream.
pub trait IPeekableTokenTree {
    // peek at the next token. if there is no next token, return None. this does not advance the current iterator but the inner iterator could be advanced in order to fetch the next token.
    fn peek(self: &Self) -> Option<TokenTree>;

    // peek at the nth token. if there is no nth token, return None. this does not advance the current iterator but could advance the inner iterator in order to fetch the next N tokens.
    fn peek_nth(self: &Self, i: usize) -> Option<TokenTree>;

    // get the next token. if there is no next token, return None. this advances the current iterator regardless if peek() has not been called. the inner iterator could be advanced if peek has not been called.
    fn next(self: &Self) -> Option<TokenTree>;

    // to string
    fn to_string(self: &Self) -> String;
}

pub struct PeekableTokenTree {
    it: RefCell<proc_macro2::token_stream::IntoIter>,
    n_peeked: RefCell<Vec<TokenTree>>,
}
impl PeekableTokenTree {
    pub fn new(stream: TokenStream) -> Self {
        Self {
            it: RefCell::new(stream.into_iter()),
            n_peeked: RefCell::new(vec![]),
        }
    }

    pub fn from_vec(rusthtml: &[TokenTree]) -> Self {
        Self::new(rusthtml.iter().cloned().collect())
    }
}

impl IPeekableTokenTree for PeekableTokenTree {
    fn peek(self: &Self) -> Option<TokenTree> {
        self.peek_nth(0)
    }

    fn next(self: &Self) -> Option<TokenTree> {
        let mut n_peeked = self.n_peeked.borrow_mut();
        if n_peeked.len() > 0 {
            Some(n_peeked.remove(0))
        } else {
            self.it.borrow_mut().next()
        }
    }

    fn peek_nth(self: &Self, i: usize) -> Option<TokenTree> {
        let mut n_peeked = self.n_peeked.borrow_mut();
        while n_peeked.len() <= i {
            if let Some(token) = self.it.borrow_mut().next() {
                n_peeked.push(token);
            } else {
                break;
            }
        }

        match n_peeked.len() {
            0 => None,
            _ => n_peeked.get(i).cloned(),
        }
    }

    fn to_string(self: &Self) -> String {
        let mut s = String::new();
        for token in self.n_peeked.borrow().iter() {
            s.push_str(&token.to_string());
        }
        s
    }
}
