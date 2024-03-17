use std::cell::RefCell;
use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, Punct};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::peekable_rusthtmltoken::{IPeekableRustHtmlToken, VecPeekableRustHtmlToken};
use super::rusthtmlparser_all::{IRustHtmlParserAssignSharedParts, IRustHtmlParserAll};


pub trait IRustHtmlParserRust: IRustHtmlParserAssignSharedParts {
    fn parse_string_with_quotes(self: &Self, peek_or_next: bool, identifier: &Ident, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<String, RustHtmlError>;
    
    fn parse_type_identifier(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>, cancellation_token: Rc<dyn ICancellationToken>) -> Result<Rc<dyn IPeekableRustHtmlToken>, RustHtmlError>;
    fn parse_rust_identifier_expression(self: &Self, add_first_ident: bool, identifier_token: &RustHtmlToken, last_token_was_ident: bool, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Rc<dyn IPeekableRustHtmlToken>, RustHtmlError>;
    // fn convert_rust_identifier_expression(self: &Self, tokens: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    //fn parse_rust_literal_expression(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    //fn parse_rust_group_expression(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_rust_string_or_ident(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_rust_string_or_ident_or_punct_or_group(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_rust_string_or_ident_or_punct_or_group_or_literal(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    // assert that the next token is a punct. if it is, return nothing. otherwise, return the unexpected token.
    // c: the punct to expect.
    // it: the iterator to use.
    // returns: nothing or the unexpected token.
    fn expect_punct(self: &Self, c: char, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<(RustHtmlToken, Punct), Option<RustHtmlToken>>;
}


pub struct RustHtmlParserRust {
    parser: RefCell<Option<Rc<dyn IRustHtmlParserAll>>>,
}

impl RustHtmlParserRust {
    pub fn new() -> Self {
        Self {
            parser: RefCell::new(None),
        }
    }
}

impl IRustHtmlParserAssignSharedParts for RustHtmlParserRust {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        *self.parser.borrow_mut() = Some(parser);
    }
}

impl IRustHtmlParserRust for RustHtmlParserRust {
    fn parse_type_identifier(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Rc<dyn IPeekableRustHtmlToken>, RustHtmlError> {
        let mut output = Vec::<RustHtmlToken>::new();
        loop {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_str("parse_type_identifier cancelled"));
            }
            let next_token = it.peek();
            if let Some(token) = next_token {
                match token {
                    RustHtmlToken::Identifier(_ident) => {
                        output.push(it.next().unwrap().clone());

                        // peek for next 3 punct tokens
                        // if it is a colon, then push it
                        let mut colons = Vec::<RustHtmlToken>::new();
                        for i in 0..3 {
                            if let Some(peek_colon) = it.peek_nth(i) {
                                match &peek_colon {
                                    RustHtmlToken::ReservedChar(c, _punct) => {
                                        match c {
                                            ':' => {
                                                colons.push(peek_colon.clone());
                                            },
                                            _ => break,
                                        }
                                    },
                                    _ => break,
                                }
                            } else {
                                break;
                            }
                        }

                        // if only one is colon, then break
                        // if none are colon, then break
                        // if two then push them to type_parts
                        // if more than two then error
                        match colons.len() {
                            0 => break,
                            1 => break,
                            2 => {
                                it.next();
                                it.next();
                                output.extend_from_slice(&colons);
                            },
                            _ => {
                                return Err(RustHtmlError::from_string(format!("unexpected colon count: {}", colons.len())));
                            }
                        }
                    },
                    RustHtmlToken::ReservedChar(c, _punct) => {
                        match c {
                            '<' => {
                                output.push(it.next().unwrap().clone());
                                let inner = self.parse_type_identifier(it.clone(), ct)?;
                                output.extend_from_slice(inner.to_splice());
                                
                                // assert that next token is '>'
                                match self.expect_punct('>', it) {
                                    Ok((t, _c)) => {
                                        output.push(t);
                                    },
                                    Err(None) => {
                                        return Err(RustHtmlError::from_string(format!("unexpected end of token stream")));
                                    },
                                    Err(Some(token)) => {
                                        return Err(RustHtmlError::from_string(format!("unexpected token in rusthtmlparser_rust: {:?}", token)));
                                    }
                                }
                                break;
                            },
                            ':' => {
                                output.push(it.next().unwrap().clone());
                            },
                            _ => {
                                return Err(RustHtmlError::from_string(format!("unexpected punct: {:?}", token)));
                            }
                        }
                    },
                    _ => {
                        output.push(it.next().unwrap().clone());
                    }
                }
            } else {
                break;
            }
        }
        Ok(Rc::new(VecPeekableRustHtmlToken::new(output)))
    }

    fn parse_rust_identifier_expression(self: &Self, add_first_ident: bool, identifier_token: &RustHtmlToken, last_token_was_ident: bool, it: Rc<dyn IPeekableRustHtmlToken>, _ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Rc<dyn IPeekableRustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        if add_first_ident {
            output.push(identifier_token.clone());
        }
        // this needs to be an argument
        let mut _last_token_was_ident = last_token_was_ident;
        loop {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_str("parse_rust_identifier_expression cancelled"));
            }

            let token_option = it.peek();
            if let Some(token) = token_option {
                match token {
                    RustHtmlToken::Identifier(_ident) => {
                        if _last_token_was_ident {
                            break;
                        } else {
                            output.push(it.next().unwrap().clone());
                            _last_token_was_ident = true;
                            continue;
                        }
                    },
                    RustHtmlToken::ReservedChar(c, _punct) => {
                        match c {
                            '.' | '?' | '!' | '_' | ':' | '&' => {
                                if _last_token_was_ident {
                                    output.push(it.next().unwrap().clone());
                                    _last_token_was_ident = false;
                                } else {
                                    break;
                                }
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
            } else {
                break;
            }

            _last_token_was_ident = false;
        }
        Ok(Rc::new(VecPeekableRustHtmlToken::new(output)))
    }

    fn parse_rust_string_or_ident(self: &Self, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!("parse_rust_string_or_ident")
    }

    fn parse_rust_string_or_ident_or_punct_or_group(self: &Self, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!("parse_rust_string_or_ident_or_punct_or_group")
    }

    fn parse_rust_string_or_ident_or_punct_or_group_or_literal(self: &Self, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!("parse_rust_string_or_ident_or_punct_or_group_or_literal")
    }

    fn expect_punct(self: &Self, c: char, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<(RustHtmlToken, Punct), Option<RustHtmlToken>> {
        if let Some(actual_c_token) = it.peek() {
            match actual_c_token { 
                RustHtmlToken::ReservedChar(actual_c, punct) => {
                    if *actual_c == c {
                        it.next();
                        Ok((actual_c_token.clone(), punct.clone()))
                    } else {
                        Err(Some(actual_c_token.clone()))
                    }
                },
                _ => Err(Some(actual_c_token.clone()))
            }
        } else {
            Err(None)
        }
    }

    fn parse_string_with_quotes(self: &Self, peek_or_next: bool, identifier: &Ident, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<String, RustHtmlError> {
        let r = if peek_or_next { it.peek() } else { it.next() };
        if let Some(expect_string_token) = r {
            match expect_string_token {
                RustHtmlToken::Literal(literal, _s) => Ok(snailquote::unescape(&literal.clone().unwrap().to_string()).unwrap()),
                _ => Err(RustHtmlError::from_string(format!("unexpected token after {} directive: {:?}", identifier, expect_string_token))),
            }
        } else {
            Err(RustHtmlError::from_string(format!("unexpected end of token stream after {} directive", identifier)))
        }
    }
}