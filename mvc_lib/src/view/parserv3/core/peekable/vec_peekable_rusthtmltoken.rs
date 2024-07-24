use std::cell::RefCell;

use super::ipeekable::IPeekable;
use super::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;

use crate::view::rusthtml::rusthtml_token::RustHtmlToken;


#[derive(Debug)]
pub struct VecPeekableRustHtmlToken {
    tokens: RefCell<Vec<RustHtmlToken>>,
    index: RefCell<usize>,
}

impl VecPeekableRustHtmlToken {
    pub fn new(tokens: Vec<RustHtmlToken>) -> Self {
        Self {
            tokens: RefCell::new(tokens),
            index: RefCell::new(0),
        }
    }
}

impl IPeekable<RustHtmlToken> for VecPeekableRustHtmlToken {
    fn peek(&self) -> Option<RustHtmlToken> {
        self.peek_nth(0)
    }

    fn peek_nth(&self, i: usize) -> Option<RustHtmlToken> {
        match self.tokens.borrow().get(*self.index.borrow() + i) {
            Some(token) => Some(token.clone()),
            None => None,
        }
    }

    fn next(&self) -> Option<RustHtmlToken> {
        let mut index = self.index.borrow_mut();
        if *index < self.tokens.borrow().len() {
            *index += 1;
            Some(self.tokens.borrow()[*index - 1].clone())
        } else {
            None
        }
    }
    
    fn to_vec(&self) -> Vec<RustHtmlToken> {
        self.tokens.borrow().clone()
    }
}

impl IPeekableRustHtmlToken for VecPeekableRustHtmlToken {
}