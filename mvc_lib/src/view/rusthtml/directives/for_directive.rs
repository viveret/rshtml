use std::rc::Rc;

use proc_macro2::Delimiter;
use proc_macro2::Ident;
use proc_macro2::TokenTree;

use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;

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

    fn execute(self: &Self, identifier: &Ident, _ident_token: &TokenTree, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        output.push(RustHtmlToken::Identifier(identifier.clone()));
        
        let is_raw_tokenstream = false;
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
                                match parser.convert_group_to_rusthtmltoken(group.clone(), false, false, output, is_raw_tokenstream) {
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
                                output.push(RustHtmlToken::Group(delimiter, group.clone()));
                                it.next();
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