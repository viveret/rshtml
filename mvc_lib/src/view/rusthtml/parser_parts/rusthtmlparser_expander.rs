use std::cell::RefCell;
use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{TokenTree, Punct, Delimiter, Group, Ident, TokenStream, Literal};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;

use super::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use super::rusthtmlparser_all::{IRustHtmlParserAssignSharedParts, IRustHtmlParserAll};


pub trait IRustHtmlParserExpander: IRustHtmlParserAssignSharedParts {
    fn expand_rust(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn expand_rshtml(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
}

pub struct RustHtmlParserExpander {
    parser: RefCell<Option<Rc<dyn IRustHtmlParserAll>>>,
}

impl RustHtmlParserExpander {
    pub fn new() -> Self {
        Self {
            parser: RefCell::new(None),
        }
    }

    fn expand_rshtmltoken(&self, token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>{
        todo!()
    }

    fn get_parser(&self) -> Rc<dyn IRustHtmlParserAll> {
        self.parser.borrow().as_ref().expect("self.parser was None").clone()
    }
}

impl IRustHtmlParserAssignSharedParts for RustHtmlParserExpander {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        *self.parser.borrow_mut() = Some(parser);
    }
}

impl IRustHtmlParserExpander for RustHtmlParserExpander {
    fn expand_rust(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        loop {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_str("parse_rust cancelled"));
            }

            let next_token = it.peek();
            if let Some(ref token) = next_token {
                match token {
                    TokenTree::Ident(ident) => {
                        // consume the token from the stream and add the token to the output
                        let converter = self.get_parser().get_converter();
                        match converter.convert_ident(&ident) {
                            Ok(ident) => {
                                output.push(ident);
                            },
                            Err(RustHtmlError(err)) => {
                                return Err(RustHtmlError::from_string(err.into_owned()));
                            }
                        }
                        // let tokens_result = parser.parse_rust_identifier_expression(true, token, false, it.clone(), ct.clone());
                        // match tokens_result {
                        //     Ok(tokens) => {
                        //         output.extend_from_slice(&self.expand_rust(ctx.clone(), tokens, ct.clone())?);
                        //     },
                        //     Err(RustHtmlError(err)) => {
                        //         return Err(RustHtmlError::from_string(err.into_owned()));
                        //     }
                        // }
                    },
                    TokenTree::Punct(punct) => {
                        // consume the token from the stream
                        it.next(); // this should be conditional in case we peek a terminal token
                        // for example, <div>hello</div> should not consume the </div> token
                        let converter = self.get_parser().get_converter();
                        match converter.convert_punct(&punct) {
                            Ok(punct) => {
                                output.push(punct);
                            },
                            Err(RustHtmlError(err)) => {
                                return Err(RustHtmlError::from_string(err.into_owned()));
                            }
                        }
                    },
                    TokenTree::Literal(literal) => {
                        // consume the token from the stream
                        it.next();
                        let converter = self.get_parser().get_converter();
                        match converter.convert_literal(&literal, ct.clone()) {
                            Ok(literal) => {
                                output.push(literal);
                            },
                            Err(RustHtmlError(err)) => {
                                return Err(RustHtmlError::from_string(err.into_owned()));
                            }
                        }
                    },
                    TokenTree::Group(group) => {
                        // consume the token from the stream
                        it.next();
                        let converter = self.get_parser().get_converter();
                        match converter.convert_group(&group, false, ct.clone()) {
                            Ok(group) => {
                                output.push(group);
                            },
                            Err(RustHtmlError(err)) => {
                                return Err(RustHtmlError::from_string(err.into_owned()));
                            }
                        }
                    },
                }
            } else {
                break;
            }
        }
        Ok(output)
    }

    fn expand_rshtml(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut output = vec![];

        loop {
            let next_token = it.peek();
            match next_token {
                Some(token) => {
                    let tokens = self.expand_rshtmltoken(token, it.clone(), ct.clone())?;
                    output.extend_from_slice(&tokens);
                },
                None => {
                    break;
                }
            }
        }

        Ok(output)
    }
}