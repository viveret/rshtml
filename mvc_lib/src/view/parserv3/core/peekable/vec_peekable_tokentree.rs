use std::cell::RefCell;

use proc_macro2::TokenTree;

use super::ipeekable::IPeekable;
use super::ipeekable_tokentree::IPeekableTokenTree;

#[derive(Debug)]
pub struct VecPeekableTokenTree {
    tokens: Vec<TokenTree>,
    index: RefCell<usize>,
}

impl VecPeekableTokenTree {
    pub fn new(tokens: Vec<TokenTree>) -> Self {
        Self {
            tokens,
            index: RefCell::new(0),
        }
    }
}

impl IPeekable<TokenTree> for VecPeekableTokenTree {
    fn to_vec(&self) -> Vec<TokenTree> {
        self.tokens.clone()
    }
    
    fn peek(&self) -> Option<TokenTree> {
        self.peek_nth(0)
    }
    
    fn next(&self) -> Option<TokenTree> {
        let mut index = self.index.borrow_mut();
        if *index < self.tokens.len() {
            *index += 1;
            Some(self.tokens[*index - 1].clone())
        } else {
            None
        }
    }
    
    fn peek_nth(&self, i: usize) -> Option<TokenTree> {
        let index = *self.index.borrow() + i;
        if index < self.tokens.len() {
            Some(self.tokens[index].clone())
        } else {
            None
        }
    }
}

impl IPeekableTokenTree for VecPeekableTokenTree {
    fn to_stream(&self) -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::from_iter(self.tokens.iter().cloned())
    }
}