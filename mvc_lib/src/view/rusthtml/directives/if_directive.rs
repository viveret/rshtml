use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenTree, Delimiter};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::rusthtml::parser_parts::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::irusthtml_directive::IRustHtmlDirective;


// The "if" directive is used to conditionally render a section of the view.
pub struct IfDirective {}

impl IfDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for IfDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "if"
    }

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, _ident_token: &RustHtmlToken, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        output.push(RustHtmlToken::Identifier(identifier.clone()));
        
        loop {
            if let Some(token) = it.peek() {
                match token {
                    RustHtmlToken::Identifier(ident) => {
                        output.push(it.next().unwrap().clone());
                    },
                    RustHtmlToken::Literal(literal, s) => {
                        output.push(it.next().unwrap().clone());
                    },
                    RustHtmlToken::ReservedChar(c, punct) => {
                        output.push(it.next().unwrap().clone());
                        if punct.as_char() == ';' {
                            break;
                        }
                    },
                    RustHtmlToken::Group(delimiter, stream, group) => {
                        let delimiter = group.delimiter();
                        match delimiter {
                            Delimiter::Brace => {
                                match parser.get_converter().convert_group(&group, false, ct.clone()) {
                                    Ok(token) => {
                                        output.push(token);
                                        // let last = output.last().unwrap();
                                        // match last {
                                        //     RustHtmlToken::GroupParsed(delimiter, tokens) => {
                                        //         // let to_str = tokens.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(" ");
                                        //         // println!("if group: {:?}", to_str);
                                        //         output.push(RustHtmlToken::Group(delimiter.clone(), Group::new(delimiter.clone(), TokenStream::from_iter(tokens.iter().cloned()))));
                                        //     },
                                        //     _ => {}
                                        // }
                                        it.next();

                                        // need to check for else if and else
                                        if let Some(token) = it.peek() {
                                            match token {
                                                RustHtmlToken::Identifier(ident) => {
                                                    if ident.to_string() == "else" {
                                                        it.next();
                                                        output.push(RustHtmlToken::Identifier(ident.clone()));

                                                        if let Some(token) = it.peek() {
                                                            match token {
                                                                RustHtmlToken::Identifier(ident) => {
                                                                    if ident.to_string() == "if" {
                                                                        // else if
                                                                        output.push(RustHtmlToken::Identifier(ident.clone()));
                                                                        it.next();
                                                                        continue;
                                                                    }
                                                                },
                                                                _ => {}
                                                            }
                                                        }

                                                        // just else, expecting brace group
                                                        if let Some(token) = it.peek() {
                                                            match token {
                                                                RustHtmlToken::Group(delimiter, stream, group) if group.delimiter() == Delimiter::Brace => {
                                                                    match parser.get_converter().convert_group(&group, false, ct) {
                                                                        Ok(group) => {
                                                                            output.push(group);
                                                                            it.next();
                                                                        },
                                                                        Err(RustHtmlError(err)) => {
                                                                            return Err(RustHtmlError::from_string(err.to_string()));
                                                                        }
                                                                    }
                                                                },
                                                                _ => {}
                                                            }
                                                        }
                                                    }
                                                },
                                                _ => {}
                                            }
                                        }

                                        break;
                                    },
                                    Err(RustHtmlError(err)) => {
                                        return Err(RustHtmlError::from_string(err.to_string()));
                                    }
                                }
                            },
                            _ => {
                                output.push(token.clone());
                                it.next();
                            }
                        }
                    },
                    _ => {
                        panic!("unexpected token: {:?}", token)
                    }
                }
            } else {
                break;
            }
        }

        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}