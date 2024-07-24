use proc_macro2::{TokenStream, TokenTree};

use super::ipeekable_tokentree::IPeekableTokenTree;
use super::ipeekable::IPeekable;


#[derive(Debug)]
pub struct EmptyPeekableTokenTree {
}

impl EmptyPeekableTokenTree {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl IPeekable<TokenTree> for EmptyPeekableTokenTree {
    fn peek(&self) -> Option<TokenTree> {
        None
    }

    fn next(&self) -> Option<TokenTree> {
        None
    }
    
    fn to_vec(&self) -> Vec<TokenTree> {
        Vec::new()
    }
    
    fn peek_nth(&self, i: usize) -> Option<TokenTree> {
        None
    }
}

impl IPeekableTokenTree for EmptyPeekableTokenTree {
    fn to_stream(&self) -> TokenStream {
        TokenStream::new()
    }
}