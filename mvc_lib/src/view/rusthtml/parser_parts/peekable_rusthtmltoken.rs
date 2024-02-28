use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::view::rusthtml::rusthtml_token::RustHtmlToken;



// this is used to peek at the next token in a RustHtml token stream.
pub trait IPeekableRustHtmlToken: Debug {
    fn peek(self: &Self) -> Option<&RustHtmlToken>;
    fn peek_nth(self: &Self, n: usize) -> Option<&RustHtmlToken>;
    fn next(self: &Self) -> Option<&RustHtmlToken>;
    fn to_string(self: &Self) -> String;
    fn to_splice(self: &Self) -> &[RustHtmlToken];
    fn to_stream(self: &Self) -> Rc<dyn IPeekableRustHtmlToken>;
}

#[derive(Clone, Debug)]
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

    fn peek_nth(self: &Self, n: usize) -> Option<&RustHtmlToken> {
        self.data.get(*self.peek_index.borrow() + n)
    }

    fn to_string(self: &Self) -> String {
        let mut s = String::new();
        for token in self.data.iter() {
            s.push_str(&token.to_string());
        }
        s
    }

    fn to_splice(self: &Self) -> &[RustHtmlToken] {
        &self.data
    }

    fn to_stream(self: &Self) -> Rc<dyn IPeekableRustHtmlToken> {
        let mut stream = vec![];
        for token in self.data.iter() {
            stream.extend(std::iter::once(token.clone()));
        }
        Rc::new(VecPeekableRustHtmlToken::new(stream))
    }
}
