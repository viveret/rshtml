use std::{rc::Rc, cell::RefCell};

use proc_macro2::{TokenTree, Ident};

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::rusthtmlparser_all::{IRustHtmlParserAssignSharedParts, IRustHtmlParserAll};



pub trait IRustHtmlParserRustOrHtml: IRustHtmlParserAssignSharedParts {
    fn parse_rust_or_html(self: &Self, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn convert_vec(&self, tokens: &Vec<TokenTree>) -> Vec<RustHtmlToken>;

    fn peek_path_str(self: &Self, identifier: &Ident, ident_token: &TokenTree, it: Rc<dyn IPeekableTokenTree>) -> Result<String, RustHtmlError>;
    fn next_path_str(self: &Self, identifier: &Ident, ident_token: &TokenTree, it: Rc<dyn IPeekableTokenTree>) -> Result<String, RustHtmlError>;
}

pub struct RustHtmlParserRustOrHtml {
    shared_parser: RefCell<Option<Rc<dyn IRustHtmlParserAll>>>,
}

impl RustHtmlParserRustOrHtml {
    pub fn new() -> Self {
        Self {
            shared_parser: RefCell::new(None),
        }
    }
}

impl IRustHtmlParserRustOrHtml for RustHtmlParserRustOrHtml {
    fn parse_rust_or_html(self: &Self, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        if let Some(shared_parser) = self.shared_parser.borrow().as_ref() {
            match shared_parser.get_rust_or_html_parser().parse_rust_or_html(it, is_raw_tokenstream) {
                Ok(tokens) => Ok(tokens),
                Err(RustHtmlError(err)) => Err(RustHtmlError::from_string(err.into_owned())),
            }
        } else {
            Err(RustHtmlError::from_str("RustHtmlParserRustOrHtml: shared_parser is None"))
        }
    }

    fn convert_vec(&self, tokens: &Vec<TokenTree>) -> Vec<RustHtmlToken> {
        tokens.iter().map(|x| RustHtmlToken::from(x)).collect::<Vec<RustHtmlToken>>()
    }

    fn peek_path_str(self: &Self, identifier: &Ident, ident_token: &TokenTree, it: Rc<dyn IPeekableTokenTree>) -> Result<String, RustHtmlError> {
        todo!("peek_path_str")
    }

    fn next_path_str(self: &Self, identifier: &Ident, ident_token: &TokenTree, it: Rc<dyn IPeekableTokenTree>) -> Result<String, RustHtmlError> {
        todo!("next_path_str")
    }
}

impl IRustHtmlParserAssignSharedParts for RustHtmlParserRustOrHtml {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        *self.shared_parser.borrow_mut() = Some(parser.clone());
    }
}
