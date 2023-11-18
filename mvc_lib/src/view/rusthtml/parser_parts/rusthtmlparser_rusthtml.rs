use std::{rc::Rc, cell::RefCell};

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Delimiter;
use proc_macro2::{TokenTree, Ident};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;

use super::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use super::rusthtmlparser_all::IRustHtmlParserAll;
use super::rusthtmlparser_all::IRustHtmlParserAssignSharedParts;



pub trait IRustHtmlParserRustOrHtml: IRustHtmlParserAssignSharedParts {
    fn preprocess_rusthtmltokens(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn postprocess_rusthtmltokens(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn preprocess_tokenstream(self: &Self, tokens: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn postprocess_tokenstream(self: &Self, tokens: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError>;
    
    fn parse_rust_or_html(self: &Self, it: Rc<dyn IPeekableTokenTree>, cancellation_token: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn convert_vec(&self, tokens: &Vec<TokenTree>, ct: Rc<dyn ICancellationToken>) -> Vec<RustHtmlToken>;

    fn peek_path_str(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<String, RustHtmlError>;
    fn next_path_str(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<String, RustHtmlError>;

    fn get_opening_delim(self: &Self, delim: Delimiter) -> &'static str;
    fn get_closing_delim(self: &Self, delim: Delimiter) -> &'static str;
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

    fn convert_vec(&self, tokens: &Vec<TokenTree>, ct: Rc<dyn ICancellationToken>) -> Vec<RustHtmlToken> {
        tokens.iter().map(|x| RustHtmlToken::from(x)).collect::<Vec<RustHtmlToken>>()
    }

    fn peek_path_str(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>,identifier: &Ident, ident_token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<String, RustHtmlError> {
        let mut path = std::path::PathBuf::new();
        let cwd = std::env::current_dir().expect("couldn't get current working directory");
        path.push(cwd);
        
        // do match instead
        match self.shared_parser.borrow().as_ref()
                    .expect("shared_parser was None")
                    .get_rust_parser()
                    .parse_string_with_quotes(true, identifier, it) {
            Ok(relative_path) => {
                path.push(relative_path.clone());
            },
            Err(RustHtmlError(err)) => {
                return Err(RustHtmlError::from_string(err.into_owned()));
            }
        }

        Ok(path.to_str().expect("couldn't awd").to_string())
    }

    fn next_path_str(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<String, RustHtmlError> {
        let mut path = std::path::PathBuf::new();
        let cwd = std::env::current_dir().expect("couldn't get current working directory");
        path.push(cwd);
        match self.shared_parser.borrow().as_ref()
                    .expect("shared_parser was None")
                    .get_rust_parser()
                    .parse_string_with_quotes(true, identifier, it) {
            Ok(relative_path) => {
                path.push(relative_path.clone());
            },
            Err(RustHtmlError(err)) => {
                return Err(RustHtmlError::from_string(err.into_owned()));
            }
        }

        Ok(path.to_str().expect("couldn't awd").to_string())
    }

    fn preprocess_rusthtmltokens(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!()
    }

    fn postprocess_rusthtmltokens(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!()
    }

    fn preprocess_tokenstream(self: &Self, tokens: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError> {
        todo!()
    }

    fn postprocess_tokenstream(self: &Self, tokens: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError> {
        todo!()
    }

    fn get_opening_delim(self: &Self, delimiter: Delimiter) -> &'static str {
        match delimiter {
            Delimiter::Brace => "{",
            Delimiter::Bracket => "[",
            Delimiter::Parenthesis => "(",
            Delimiter::None => "",
        }
    }

    fn get_closing_delim(self: &Self, delimiter: Delimiter) -> &'static str {
        match delimiter {
            Delimiter::Brace => "}",
            Delimiter::Bracket => "]",
            Delimiter::Parenthesis => ")",
            Delimiter::None => "",
        }
    }
}

impl IRustHtmlParserAssignSharedParts for RustHtmlParserRustOrHtml {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        *self.shared_parser.borrow_mut() = Some(parser.clone());
    }
}
