use std::rc::Rc;

use proc_macro2::{Group, Ident, TokenTree, TokenStream, Delimiter};

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
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

    fn execute(self: &Self, identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        output.push(RustHtmlToken::Identifier(identifier.clone()));
        
        loop {
            if let Some(token) = it.peek() {
                match token {
                    TokenTree::Ident(ident) => {
                        output.push(RustHtmlToken::Identifier(ident.clone()));
                        it.next();
                    },
                    TokenTree::Literal(literal) => {
                        output.push(RustHtmlToken::Literal(Some(literal.clone()), None));
                        it.next();
                    },
                    TokenTree::Punct(punct) => {
                        println!("punct: {:?}", punct);
                        output.push(RustHtmlToken::ReservedChar(punct.as_char(), punct.clone()));
                        it.next();
                        if punct.as_char() == ';' {
                            break;
                        }
                    },
                    TokenTree::Group(group) => {
                        let delimiter = group.delimiter();
                        match delimiter {
                            Delimiter::Brace => {
                                match parser.convert_group_to_rusthtmltoken(group, false, false, output, false) {
                                    Ok(_) => {
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
                                        break;
                                    },
                                    Err(RustHtmlError(err)) => {
                                        return Err(RustHtmlError::from_string(err.to_string()));
                                    }
                                }
                            },
                            _ => {
                                output.push(RustHtmlToken::Group(group.delimiter(), group.clone()));
                                it.next();
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }

        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}