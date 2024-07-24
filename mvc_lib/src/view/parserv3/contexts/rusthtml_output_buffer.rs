use std::rc::Rc;
use std::cell::RefCell;

use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

pub struct RustHtmlTokenVec(Vec<RustHtmlToken>);

#[derive(Clone)]
pub struct RustHtmlTokenBuffer(Rc<RefCell<RustHtmlTokenVec>>);

pub struct RustHtmlTokenBufferBuffer(RefCell<Vec<RustHtmlTokenBuffer>>);

impl RustHtmlTokenBuffer {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(RustHtmlTokenVec(Vec::new()))))
    }

    pub fn push(&self, token: RustHtmlToken) {
        self.0.borrow_mut().0.push(token);
    }

    pub fn pop(&self) -> Option<RustHtmlToken> {
        self.0.borrow_mut().0.pop()
    }

    pub fn extend(&self, tokens: &[RustHtmlToken]) {
        self.0.borrow_mut().0.extend_from_slice(tokens);
    }

    pub fn extend_buffer(&self, buffer: &RustHtmlTokenBuffer) {
        self.0.borrow_mut().0.extend(buffer.0.borrow().0.iter().cloned());
    }

    pub fn first(&self) -> Option<RustHtmlToken> {
        self.0.borrow().0.first().cloned()
    }

    pub fn last(&self) -> Option<RustHtmlToken> {
        self.0.borrow().0.last().cloned()
    }

    pub fn to_vec(&self) -> Vec<RustHtmlToken> {
        self.0.borrow().0.clone()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<RustHtmlTokenVec> {
        self.0.borrow_mut()
    }

    pub fn remove(&self, index: usize) -> Option<RustHtmlToken> {
        Some(self.0.borrow_mut().0.remove(index))
    }
}

impl RustHtmlTokenBufferBuffer {
    pub fn new() -> Self {
        Self(RefCell::new(Vec::new()))
    }

    pub fn push(&self, buffer: RustHtmlTokenBuffer) {
        self.0.borrow_mut().push(buffer);
    }

    pub fn pop(&self) -> Option<RustHtmlTokenBuffer> {
        self.0.borrow_mut().pop()
    }

    pub fn extend(&self, buffers: &[RustHtmlTokenBuffer]) {
        self.0.borrow_mut().extend_from_slice(buffers);
    }

    pub fn first(&self) -> Option<RustHtmlTokenBuffer> {
        self.0.borrow().first().cloned()
    }

    pub fn last(&self) -> Option<RustHtmlTokenBuffer> {
        self.0.borrow().last().cloned()
    }

    pub fn to_vec(&self) -> Vec<RustHtmlTokenBuffer> {
        self.0.borrow().iter().cloned().collect()
    }
}