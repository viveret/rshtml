use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Delimiter;
use proc_macro2::Ident;
use proc_macro2::TokenTree;

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::irusthtmlparser_version_agnostic::IRustHtmlParserVersionAgnostic;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::rusthtml::parser_parts::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;

use super::irusthtml_directive::IRustHtmlDirective;


// the "for" directive is used to iterate over a collection and render a section of the view for each item in the collection.
pub struct ForDirective {}

impl ForDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for ForDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "for"
    }

    fn execute(self: &Self, _context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, _ident_token: &TokenTree, _parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, _ct: Rc<dyn ICancellationToken> ) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        output.push(RustHtmlToken::Identifier(identifier.clone()));
        
        loop {
            if let Some(token) = it.peek() {
                match &token {
                    TokenTree::Ident(ident) => {
                        output.push(RustHtmlToken::Identifier(ident.clone()));
                        it.next();
                    },
                    TokenTree::Literal(literal) => {
                        output.push(RustHtmlToken::Literal(Some(literal.clone()), None));
                        it.next();
                    },
                    TokenTree::Punct(punct) => {
                        output.push(RustHtmlToken::ReservedChar(punct.as_char(), punct.clone()));
                        it.next();
                    },
                    TokenTree::Group(_group) => {
                        unimplemented!("parse the for body");
                        // let delimiter = group.delimiter();
                        // match delimiter {
                        //     Delimiter::Brace => {
                        //         match parser.get_rust_parser().convert_group(group, false, ct) {
                        //             Ok(_) => {
                        //                 // println!("for_directive: {} -> {:?}", token.to_string(), output.last());
                        //                 it.next();
                        //                 break;
                        //             },
                        //             Err(RustHtmlError(e)) => {
                        //                 return Err(RustHtmlError::from_string(e.to_string()));
                        //             }
                        //         }
                        //     },
                        //     _ => {
                        //         output.push(RustHtmlToken::Group(delimiter, group.clone()));
                        //         it.next();
                        //     },
                        // }
                    },
                }
                // println!("for_directive: {} -> {:?}", token.to_string(), output.last());
            } else {
                break;
            }
        }

        Ok(RustHtmlDirectiveResult::OkContinue)
    }
    
    fn execute_new(self: &Self, _context: Rc<dyn IRustHtmlParserContext>, _identifier: &Ident, _ident_token: &RustHtmlToken, _parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, _it: Rc<dyn IPeekableRustHtmlToken>, _ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        todo!("execute_new for directive")
    }
    
    fn execute_old(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &TokenTree, parser: Rc<crate::view::rusthtml::rusthtml_parser::RustHtmlParser>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        output.push(RustHtmlToken::Identifier(identifier.clone()));
        
        loop {
            if let Some(token) = it.peek() {
                match &token {
                    TokenTree::Ident(ident) => {
                        output.push(RustHtmlToken::Identifier(ident.clone()));
                        it.next();
                    },
                    TokenTree::Literal(literal) => {
                        output.push(RustHtmlToken::Literal(Some(literal.clone()), None));
                        it.next();
                    },
                    TokenTree::Punct(punct) => {
                        output.push(RustHtmlToken::ReservedChar(punct.as_char(), punct.clone()));
                        it.next();
                    },
                    TokenTree::Group(group) => {
                        let delimiter = group.delimiter();
                        match delimiter {
                            Delimiter::Brace => {
                                let mut inner_output = Vec::new();
                                match parser.parser.convert_group_to_rusthtmltoken(group.clone(), false, true, &mut inner_output, false, ct.clone()) {
                                    Ok(_) => {
                                        // println!("for_directive: {} -> {:?}", token.to_string(), output.last());
                                        it.next();
                                        break;
                                    },
                                    Err(RustHtmlError(e)) => {
                                        return Err(RustHtmlError::from_string(e.to_string()));
                                    }
                                }
                            },
                            _ => {
                                let mut inner_output = Vec::new();
                                match parser.parser.convert_copy(token.clone(), &mut inner_output, ct.clone()) {
                                    Ok(_) => {
                                        let stream = Rc::new(VecPeekableRustHtmlToken::new(inner_output));
                                        output.push(RustHtmlToken::Group(delimiter, stream, Some(group.clone())));
                                        it.next();
                                    },
                                    Err(RustHtmlError(e)) => {
                                        return Err(RustHtmlError::from_string(e.to_string()));
                                    }
                                }
                            },
                        }
                    },
                }
                // println!("for_directive: {} -> {:?}", token.to_string(), output.last());
            } else {
                break;
            }
        }
        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}