use std::{cell::RefCell, rc::Rc};

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

    // to splice
    fn to_splice(self: &Self) -> &[TokenTree];

    // to stream
    fn to_stream(self: &Self) -> TokenStream;

    fn enable_log_next(&self, msg: &str);
    fn disable_log_next(&self);
}

#[derive(Clone)]
pub struct StreamPeekableTokenTree {
    it: RefCell<proc_macro2::token_stream::IntoIter>,
    n_peeked: RefCell<Vec<TokenTree>>,
    // whether or not the next token is logged
    log_next_enabled: RefCell<bool>,
    // what to put in the logged message for the next token
    log_next_msg: RefCell<String>,
    // is raw
    is_raw: bool,
}
impl StreamPeekableTokenTree {
    pub fn new(stream: TokenStream) -> Self {
        Self {
            it: RefCell::new(stream.into_iter()),
            n_peeked: RefCell::new(vec![]),
            log_next_enabled: RefCell::new(false),
            log_next_msg: RefCell::new(String::new()),
            is_raw: false,
        }
    }

    pub fn new_raw(stream: TokenStream) -> Self {
        Self {
            it: RefCell::new(stream.into_iter()),
            n_peeked: RefCell::new(vec![]),
            log_next_enabled: RefCell::new(false),
            log_next_msg: RefCell::new(String::new()),
            is_raw: true,
        }
    }

    pub fn rc(self: &Self) -> Rc<dyn IPeekableTokenTree> {
        Rc::new(self.clone())
    }
}

impl IPeekableTokenTree for StreamPeekableTokenTree {
    fn peek(self: &Self) -> Option<TokenTree> {
        self.peek_nth(0)
    }

    fn next(self: &Self) -> Option<TokenTree> {
        let mut n_peeked = self.n_peeked.borrow_mut();
        let token = if n_peeked.len() > 0 {
            Some(n_peeked.remove(0))
        } else {
            self.it.borrow_mut().next()
        };

        if *self.log_next_enabled.borrow() {
            let msg = self.log_next_msg.borrow();
            println!("{}: {:?}", msg, token);
        }
        token
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

    fn enable_log_next(&self, msg: &str) {
        self.log_next_enabled.replace(true);
        self.log_next_msg.replace(msg.to_string());
    }

    fn disable_log_next(&self) {
        self.log_next_enabled.replace(false);
    }

    fn to_splice(self: &Self) -> &[TokenTree] {
        unimplemented!("to_splice not implemented for StreamPeekableTokenTree")
    }

    fn to_stream(self: &Self) -> TokenStream {
        let mut stream = TokenStream::new();
        for token in self.n_peeked.borrow().iter() {
            stream.extend(std::iter::once(token.clone()));
        }
        stream
    }
}


pub struct VecPeekableTokenTree {
    tokens: Vec<TokenTree>,
    index: RefCell<usize>,
    // whether or not the next token is logged
    log_next_enabled: RefCell<bool>,
    // what to put in the logged message for the next token
    log_next_msg: RefCell<String>,
}
impl VecPeekableTokenTree {
    pub fn new(tokens: Vec<TokenTree>) -> Self {
        Self {
            tokens,
            index: RefCell::new(0),
            log_next_enabled: RefCell::new(false),
            log_next_msg: RefCell::new(String::new()),
        }
    }
}
impl IPeekableTokenTree for VecPeekableTokenTree {
    fn peek(self: &Self) -> Option<TokenTree> {
        self.peek_nth(0)
    }

    fn next(self: &Self) -> Option<TokenTree> {
        let token = self.peek_nth(0);
        if token.is_some() {
            *self.index.borrow_mut() += 1;
        }

        if *self.log_next_enabled.borrow() {
            let msg = self.log_next_msg.borrow();
            println!("{}: {:?}", msg, token);
        }
        token
    }

    fn peek_nth(self: &Self, i: usize) -> Option<TokenTree> {
        self.tokens.get(*self.index.borrow() + i).cloned()
    }

    fn to_string(self: &Self) -> String {
        let mut s = String::new();
        for token in self.tokens.iter() {
            s.push_str(&token.to_string());
        }
        s
    }

    fn enable_log_next(&self, msg: &str) {
        self.log_next_enabled.replace(true);
        self.log_next_msg.replace(msg.to_string());
    }

    fn disable_log_next(&self) {
        self.log_next_enabled.replace(false);
    }

    fn to_splice(self: &Self) -> &[TokenTree] {
        &self.tokens[*self.index.borrow()..]
    }

    fn to_stream(self: &Self) -> TokenStream {
        let mut stream = TokenStream::new();
        for token in self.tokens.iter() {
            stream.extend(std::iter::once(token.clone()));
        }
        stream
    }
}


impl From<TokenStream> for StreamPeekableTokenTree {
    fn from(stream: TokenStream) -> Self {
        Self::new(stream)
    }
}