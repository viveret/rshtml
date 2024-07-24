use std::cell::RefCell;

use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use super::ipeekable_tokentree::IPeekableTokenTree;
use super::ipeekable::IPeekable;

#[derive(Debug)]
pub struct StreamPeekableTokenTree {
    stream: TokenStream,
    it: RefCell<proc_macro2::token_stream::IntoIter>,
    peeked: RefCell<Vec<TokenTree>>,
}

impl StreamPeekableTokenTree {
    pub fn new(stream: TokenStream) -> Self {
        Self {
            stream: stream.clone(),
            it: RefCell::new(stream.into_iter()),
            peeked: RefCell::new(vec![]),
        }
    }
}

impl IPeekable<TokenTree> for StreamPeekableTokenTree {
    fn peek(&self) -> Option<TokenTree> {
        self.peek_nth(0)
    }

    fn next(&self) -> Option<TokenTree> {
        // check if peeked any
        let mut peeked = self.peeked.borrow_mut();
        if !peeked.is_empty() {
            return Some(peeked.remove(0));
        }
        match self.it.borrow_mut().next() {
            Some(token) => Some(token),
            None => None,
        }
    }

    fn to_vec(&self) -> Vec<TokenTree> {
        self.stream.clone().into_iter().collect()
    }
    
    fn peek_nth(&self, i: usize) -> Option<TokenTree> {
        let mut peeked = self.peeked.borrow_mut();
        while peeked.len() <= i {
            match self.it.borrow_mut().next() {
                Some(token) => peeked.push(token),
                None => break,
            }
        }
        peeked.get(i).cloned()
    }
}

impl IPeekableTokenTree for StreamPeekableTokenTree {
    fn to_stream(&self) -> TokenStream {
        self.stream.clone()
    }
}