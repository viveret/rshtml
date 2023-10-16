use std::cell::RefCell;

use crate::view::rusthtml::rusthtml_token::RustHtmlToken;



// this is used to peek at the next token in a RustHtml token stream.
pub trait IPeekableRustHtmlToken {
    fn peek(self: &Self) -> Option<&RustHtmlToken>;
    fn next(self: &Self) -> Option<&RustHtmlToken>;
}

pub struct VecPeekableRustHtmlToken {
    data: Vec<RustHtmlToken>,
    index: RefCell<usize>,
    peek_index: RefCell<usize>,
}

impl <'a> VecPeekableRustHtmlToken {
    pub fn new(data: Vec<RustHtmlToken>) -> Self {
        Self {
            data,
            index: RefCell::new(0),
            peek_index: RefCell::new(0),
        }
    }
}

impl <'a> IPeekableRustHtmlToken for VecPeekableRustHtmlToken {
    fn peek(self: &Self) -> Option<&RustHtmlToken> {
        self.data.get(*self.peek_index.borrow())
    }

    fn next(self: &Self) -> Option<&RustHtmlToken> {
        let token = self.peek();
        if token.is_some() {
            *self.index.borrow_mut() += 1;
            *self.peek_index.borrow_mut() += 1;
        }
        token
    }
}
