use std::backtrace;
use std::cell::RefCell;
use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro::TokenTree;
use proc_macro2::{Delimiter, Ident};

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::parserv3::IParserV3;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

pub trait IParserV3RustParser {
    fn parse_type_identifier(&self, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_var_identifier(&self, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_var_literal(&self, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_var(&self, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_string_with_quotes(&self, peek_or_next: bool, identifier: &Ident, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<String, RustHtmlError>;
    fn parse_expression(&self, it: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    fn set_parser(&self, parser: Rc<dyn IParserV3>);
    fn get_parser(&self) -> Rc<dyn IParserV3>;
}

pub struct ParserV3RustParser {
    parser: RefCell<Option<Rc<dyn IParserV3>>>
}

impl ParserV3RustParser {
    pub fn new() -> Self {
        Self { parser: RefCell::new(None) }
    }
}

impl IParserV3RustParser for ParserV3RustParser {
    fn set_parser(&self, parser: Rc<dyn IParserV3>) {
        self.parser.replace(Some(parser));
    }

    fn get_parser(&self) -> Rc<dyn IParserV3> {
        self.parser.borrow().as_ref().unwrap().clone()
    }

    fn parse_type_identifier(&self, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut tokens = vec![];
        let mut type_template_stack = 0;
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
                                '<' => {
                                    type_template_stack += 1;
                                    tokens.push(RustHtmlToken::ReservedChar(c.clone(), p.clone()));
                                    it.next();
                                },
                                '>' => {
                                    // error
                                    if type_template_stack == 0 {
                                        return Err(RustHtmlError::from_str("unexpected '>' while parsing type identifier"));
                                    }

                                    type_template_stack -= 1;
                                    tokens.push(RustHtmlToken::ReservedChar(c.clone(), p.clone()));
                                    it.next();

                                    // end of type identifier
                                    if type_template_stack == 0 {
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
    
    fn parse_var_identifier(&self, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut tokens = vec![];
        // only identifier then puncts allowed
        while let Some(token) = it.peek() {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_cancellationtoken(ct));
            }

            // look for identifier
            if let RustHtmlToken::Identifier(i) = token {
                tokens.push(it.next().unwrap());
            } else {
                break;
            }

            // look for puncts
            // todo / bugfix / fixme: this might cause problems for </> (closing tags)
            while let Some(RustHtmlToken::ReservedChar(c, p)) = it.peek() {
                if ct.is_cancelled() {
                    return Err(RustHtmlError::from_cancellationtoken(ct));
                }

                if c != '.'  && c != ':' {
                    break;
                }

                tokens.push(it.next().unwrap());
            }
        }

        if tokens.is_empty() {
            Err(RustHtmlError::from_str("Expected identifier or punct, received non identifier or punct"))
        } else {
            Ok(tokens)
        }
    }
    
    fn parse_var_literal(&self, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        if let Some(RustHtmlToken::Literal(l, s)) = it.peek() {
            Ok(vec![it.next().unwrap()])
        } else {
            Err(RustHtmlError::from_str("Expected literal, received non literal"))
        }
    }
    
    fn parse_var(&self, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        // prefix puncts
        let mut prefix_puncts = vec![];
        while let Some(RustHtmlToken::ReservedChar(c, p)) = it.peek() {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_cancellationtoken(ct));
            }
            prefix_puncts.push(it.next().unwrap());
        }

        // check if literal or var
        let tokens_result = if let Some(token) = it.peek() {
            match token {
                RustHtmlToken::Identifier(i) => {
                    self.parse_var_identifier(it, ct)
                },
                RustHtmlToken::Literal(l, s) => {
                    self.parse_var_literal(it, ct)
                },
                _ => {
                    return Err(RustHtmlError::from_string(format!("Expected identifier or literal, not {:?}", token)))
                }
            }
        } else {
            return Err(RustHtmlError::from_string(format!("Expected identifier or literal, received nothing")))
        };

        match tokens_result {
            Ok(tokens) => {
                // prepend prefix puncts
                let all_tokens = vec![prefix_puncts, tokens].concat();
                Ok(all_tokens)
            },
            Err(e) => {
                Err(e)
            }
        }
    }
    
    fn parse_expression(&self, it: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        // must be literal, identifier, or () group, and end on literal, ident, or () group
        if let Some(token) = it.next() {
            match &token {
                RustHtmlToken::Group(d, s, g) => {
                    Ok(vec![token.clone()])
                },
                RustHtmlToken::Identifier(i) => {
                    // println!("parse_expression token: {}", token.to_string());

                    let mut tokens = vec![token];
                    let mut last_token_was_ident = false;
                    while let Some(token) = it.peek() {
                        if ct.is_cancelled() {
                            return Err(RustHtmlError::from_cancellationtoken(ct));
                        }

                        // println!("parse_expression token: {}", token.to_string());

                        let mut break_after_add = false;
                        let mut overwrite_token_to_add = None;

                        match &token {
                            RustHtmlToken::Identifier(i) => {
                                if last_token_was_ident {
                                    break;
                                } else {
                                    last_token_was_ident = true;
                                }
                            },
                            RustHtmlToken::ReservedChar(c, p) => {
                                match c {
                                    '.' | ':' | '!' => {
                                        last_token_was_ident = false;
                                    },
                                    '>' | '<' | '@' => {
                                        break;
                                    },
                                    _ => {
                                        let bt = backtrace::Backtrace::capture();
                                        return Err(RustHtmlError::from_string(format!("parse_expression unexpected char {}\n{}", c, bt.to_string())))
                                    }
                                }
                            },
                            RustHtmlToken::Group(d, s, g) => {
                                last_token_was_ident = false;
                                if *d == Delimiter::Bracket {
                                } else if *d == Delimiter::Parenthesis || *d == Delimiter::Brace {
                                    // need to ensure inner parts are converted
                                    let inner_result = self.get_parser().get_converter_middle().convert(s.clone(), context.clone(), ct.clone())?;
                                    overwrite_token_to_add = Some(RustHtmlToken::Group(*d, inner_result, None));
                                } else {
                                    return Err(RustHtmlError::from_string(format!("parse_expression invalid group delimiter {:?}", d)))
                                }
                            },
                            RustHtmlToken::Literal(l, s) => {
                                if let Some(literal) = l {
                                    last_token_was_ident = false;
                                } else if let Some(string) = s {
                                    last_token_was_ident = false;
                                } else {
                                    panic!("wtf");
                                }
                            },
                            _ => {
                                return Err(RustHtmlError::from_string(format!("parse_expression unexpected token {:?}", token)))
                            }
                        }

                        // println!("token: {:?}", token);
                        if let Some(t) = overwrite_token_to_add {
                            tokens.push(t);
                        } else {
                            tokens.push(token);
                        }
                        it.next();

                        if break_after_add {
                            break;
                        }
                    }
                    Ok(tokens)
                },
                RustHtmlToken::Literal(l, s) => {
                    Ok(vec![token.clone()])
                },
                _ => Err(RustHtmlError::from_string(format!("invalid expression: {:?}", token)))
            }
        } else {
            Err(RustHtmlError::from_str("expression ended early"))
        }
    }
}