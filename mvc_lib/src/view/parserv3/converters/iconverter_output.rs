use std::rc::Rc;

use crate::view::rusthtml::parser_parts::{peekable_rusthtmltoken::IPeekableRustHtmlToken, peekable_tokentree::IPeekableTokenTree};

pub trait IConverterOutput {
    fn convert(&self, input: Rc<dyn IPeekableRustHtmlToken>) -> Rc<dyn IPeekableTokenTree>;
}

pub struct ConverterOutput {
}

impl ConverterOutput {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl IConverterOutput for ConverterOutput {
    fn convert(&self, input: Rc<dyn IPeekableRustHtmlToken>) -> Rc<dyn IPeekableTokenTree> {
        todo!("Implement IConverterOutput for ConverterOutput")
    }
}