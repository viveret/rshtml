use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Ident;

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::vec_peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::{RustHtmlDirectiveResult, RustHtmlDirectiveResultV3};
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};

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
/*
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
     */
    fn execute_new_v3(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        let mut output = vec![ident_token.clone()];
        
        loop {
            if let Some(token) = it.next() {
                match token {
                    _ => {
                        output.push(token.clone());
                    },
                    RustHtmlToken::Group(delimiter, stream, group) => {
                        match delimiter {
                            proc_macro2::Delimiter::Brace => {
                                // recurse
                                match parser.get_converter_middle().convert(stream.clone(), context.clone(), ct.clone()) {
                                    Ok(new_input) => {
                                        output.push(RustHtmlToken::Group(delimiter, new_input, group.clone()));
                                        break;
                                    },
                                    Err(e) => {
                                        return Err(e);
                                    }
                                }
                            },
                            _ => {
                                output.push(RustHtmlToken::Group(delimiter, stream.clone(), group.clone()));
                            },
                        }
                    },
                }
                // println!("for_directive: {} -> {:?}", token.to_string(), output.last());
            } else {
                break;
            }
        }

        let output_stream = Rc::new(VecPeekableRustHtmlToken::new(output));
        Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, Some(output_stream)))
    }
}