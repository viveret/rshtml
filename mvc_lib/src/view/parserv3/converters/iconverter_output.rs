use std::rc::Rc;

use proc_macro2::TokenTree;

use crate::view::rusthtml::{parser_parts::{peekable_rusthtmltoken::IPeekableRustHtmlToken, peekable_tokentree::{IPeekableTokenTree, StreamPeekableTokenTree, VecPeekableTokenTree}}, rusthtml_error::RustHtmlError};

pub trait IConverterOutput {
    fn convert(&self, input: Rc<dyn IPeekableRustHtmlToken>) -> Result<Rc<dyn IPeekableTokenTree>, RustHtmlError>;
}

pub struct ConverterOutput {
}

impl ConverterOutput {
    pub fn new() -> Self {
        Self {
        }
    }
    
    fn convert_token(&self, token: &crate::view::rusthtml::rusthtml_token::RustHtmlToken) -> Result<TokenTree, RustHtmlError> {
        todo!()
    }
}

impl IConverterOutput for ConverterOutput {
    fn convert(&self, input: Rc<dyn IPeekableRustHtmlToken>) -> Result<Rc<dyn IPeekableTokenTree>, RustHtmlError> {
        let mut output: Vec<TokenTree> = vec![];
        loop {
            let token = input.peek();
            if token.is_none() {
                break;
            }
            let token = token.unwrap();
            output.push(self.convert_token(token)?);
            input.next();
        }
        Ok(Rc::new(VecPeekableTokenTree::new(output)))
    }
}