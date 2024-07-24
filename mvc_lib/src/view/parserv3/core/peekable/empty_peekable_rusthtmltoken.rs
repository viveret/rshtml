use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::ipeekable::IPeekable;

#[derive(Debug)]
pub struct EmptyPeekableRustHtmlToken {
}

impl EmptyPeekableRustHtmlToken {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl IPeekable<RustHtmlToken> for EmptyPeekableRustHtmlToken {
    fn peek(&self) -> Option<RustHtmlToken> {
        None
    }

    fn peek_nth(&self, _i: usize) -> Option<RustHtmlToken> {
        None
    }

    fn next(&self) -> Option<RustHtmlToken> {
        None
    }
    
    fn to_vec(&self) -> Vec<RustHtmlToken> {
        Vec::new()
    }
}

impl IPeekableRustHtmlToken for EmptyPeekableRustHtmlToken {
}