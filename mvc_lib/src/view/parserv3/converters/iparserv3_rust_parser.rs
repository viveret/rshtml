use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenTree};

use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;

pub trait IParserV3RustParser {
    fn parse_type_identifier(&self, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_string_with_quotes(&self, peek_or_next: bool, identifier: &Ident, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<String, RustHtmlError>;
}

pub struct ParserV3RustParser {

}
impl ParserV3RustParser {
    pub fn new() -> Self {
        Self {  }
    }
}
impl IParserV3RustParser for ParserV3RustParser {
    fn parse_type_identifier(&self, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut tokens = vec![];
        loop {
            match it.peek() {
                Some(token) => {
                    match token {
                        RustHtmlToken::Identifier(ident) => {
                            tokens.push(RustHtmlToken::Identifier(ident.clone()));
                            it.next();
                        },
                        RustHtmlToken::ReservedChar(c, p) => {
                            match c {
                                ':' => {
                                    tokens.push(RustHtmlToken::ReservedChar(c.clone(), p.clone()));
                                    it.next();
                                },
                                _ => {
                                    break;
                                }
                            }
                        },
                        _ => {
                            break;
                        }
                    }
                },
                None => {
                    break;
                }
            }
        }
        Ok(tokens)
    }
    
    fn parse_string_with_quotes(&self, peek_or_next: bool, identifier: &Ident, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<String, RustHtmlError> {
        let r = if peek_or_next { it.peek() } else { it.next() };
        if let Some(expect_string_token) = r {
            match expect_string_token {
                RustHtmlToken::Literal(literal, s) => {
                    let str = literal.clone().map(|l| l.to_string()).unwrap_or_else(|| s.clone().unwrap());
                    Ok(snailquote::unescape(&str).expect("snailquote::unescape failed"))
                },
                _ => Err(RustHtmlError::from_string(format!("unexpected token after {} directive: {:?}", identifier, expect_string_token))),
            }
        } else {
            Err(RustHtmlError::from_string(format!("unexpected end of token stream after {} directive", identifier)))
        }
    }
}