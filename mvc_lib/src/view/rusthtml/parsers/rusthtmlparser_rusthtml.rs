use std::{rc::Rc, cell::RefCell};

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{TokenTree, Ident};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::parsers::peekable_tokentree::IPeekableTokenTree;

use super::rusthtmlparser_all::IRustHtmlParserAll;
use super::rusthtmlparser_all::IRustHtmlParserAssignSharedParts;



pub trait IRustHtmlParserRustOrHtml: IRustHtmlParserAssignSharedParts {
    fn parse_rust_or_html(self: &Self, it: Rc<dyn IPeekableTokenTree>, cancellation_token: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn convert_vec(&self, tokens: &Vec<TokenTree>) -> Vec<RustHtmlToken>;

    fn peek_path_str(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &TokenTree, it: Rc<dyn IPeekableTokenTree>) -> Result<String, RustHtmlError>;
    fn next_path_str(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &TokenTree, it: Rc<dyn IPeekableTokenTree>) -> Result<String, RustHtmlError>;
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
    fn parse_rust_or_html(self: &Self, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        if ct.is_cancelled() {
            return Err(RustHtmlError::from_str("RustHtmlParserRustOrHtml: cancellation_token is cancelled"));
        }
        
        if let Some(shared_parser) = self.shared_parser.borrow().as_ref() {
            match shared_parser.get_rust_or_html_parser().parse_rust_or_html(it, ct) {
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

    fn peek_path_str(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>,identifier: &Ident, ident_token: &TokenTree, it: Rc<dyn IPeekableTokenTree>) -> Result<String, RustHtmlError> {
        let mut path = std::path::PathBuf::new();
        let cwd = std::env::current_dir().unwrap();
        path.push(cwd);
        
        // do match instead
        match self.shared_parser.borrow().as_ref().unwrap().get_rust_parser().parse_string_with_quotes(true, identifier, it) {
            Ok(relative_path) => {
                path.push(relative_path.clone());
            },
            Err(RustHtmlError(err)) => {
                return Err(RustHtmlError::from_string(err.into_owned()));
            }
        }

        Ok(path.to_str().unwrap().to_string())
    }

    fn next_path_str(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &TokenTree, it: Rc<dyn IPeekableTokenTree>) -> Result<String, RustHtmlError> {
        let mut path = std::path::PathBuf::new();
        let cwd = std::env::current_dir().unwrap();
        path.push(cwd);
        match self.shared_parser.borrow().as_ref().unwrap().get_rust_parser().parse_string_with_quotes(false, identifier, it) {
            Ok(relative_path) => {
                path.push(relative_path.clone());
            },
            Err(RustHtmlError(err)) => {
                return Err(RustHtmlError::from_string(err.into_owned()));
            }
        }

        Ok(path.to_str().unwrap().to_string())
    }
}

impl IRustHtmlParserAssignSharedParts for RustHtmlParserRustOrHtml {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        *self.shared_parser.borrow_mut() = Some(parser.clone());
    }
}
