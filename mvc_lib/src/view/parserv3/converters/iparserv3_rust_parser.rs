use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;

use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;

pub trait IParserV3RustParser {
    fn parse_type_identifier(&self, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
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
}