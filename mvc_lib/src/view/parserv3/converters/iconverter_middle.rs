use std::rc::Rc;

use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;

pub trait IConverterMiddle {
    fn convert(&self, input: Rc<dyn IPeekableRustHtmlToken>) -> Rc<dyn IPeekableRustHtmlToken>;
}

pub struct ConverterMiddle {
}

impl ConverterMiddle {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl IConverterMiddle for ConverterMiddle {
    fn convert(&self, input: Rc<dyn IPeekableRustHtmlToken>) -> Rc<dyn IPeekableRustHtmlToken> {
        todo!("Implement IConverterMiddle for ConverterMiddle")
    }
}