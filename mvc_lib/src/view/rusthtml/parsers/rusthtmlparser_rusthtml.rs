use std::{rc::Rc, cell::RefCell};

use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::rusthtmlparser_all::{IRustHtmlParserAssignSharedParts, IRustHtmlParserAll};



pub trait IRustHtmlParserRustOrHtml: IRustHtmlParserAssignSharedParts {
    fn parse_rust_or_html(self: &Self, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
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

    pub fn get_parser(self: &Self) -> Rc<dyn IRustHtmlParserRustOrHtml> {
        self.shared_parser.borrow().as_ref().expect("self.shared_parser").clone()
    }
}

impl IRustHtmlParserRustOrHtml for RustHtmlParserRustOrHtml {
    fn parse_rust_or_html(self: &Self, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        if let Some(shared_parser) = self.get_parser() {
            match shared_parser.get_rust_or_html_parser().parse_rust_or_html(it, is_raw_tokenstream) {
                Ok(tokens) => Ok(tokens),
                Err(RustHtmlError(err)) => Err(RustHtmlError::from_string(err.into_owned())),
            }
        } else {
            Err(RustHtmlError::from_str("RustHtmlParserRustOrHtml: shared_parser is None"))
        }
    }
}

impl IRustHtmlParserAssignSharedParts for RustHtmlParserRustOrHtml {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        *self.shared_parser.borrow_mut() = Some(parser.clone());
    }
}
