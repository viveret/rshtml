use super::ipeekable::IPeekable;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

pub trait IPeekableRustHtmlToken: IPeekable<RustHtmlToken> + std::fmt::Debug {
}