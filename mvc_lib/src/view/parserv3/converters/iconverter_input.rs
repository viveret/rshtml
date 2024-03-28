use std::rc::Rc;

use crate::view::rusthtml::parser_parts::{peekable_rusthtmltoken::IPeekableRustHtmlToken, peekable_tokentree::IPeekableTokenTree};

pub trait IConverterInput {
    fn convert(&self, input: Rc<dyn IPeekableTokenTree>) -> Rc<dyn IPeekableRustHtmlToken>;
}

pub struct ConverterInput {
}

impl ConverterInput {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl IConverterInput for ConverterInput {
    fn convert(&self, input: Rc<dyn IPeekableTokenTree>) -> Rc<dyn IPeekableRustHtmlToken> {
        todo!("Implement IConverterInput for ConverterInput")
    }
}