use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Delimiter;
use proc_macro2::Ident;
use proc_macro2::TokenTree;

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;
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

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, _ident_token: &RustHtmlToken, parser: Rc<dyn IRustHtmlParserAll>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken> ) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        output.push(RustHtmlToken::Identifier(identifier.clone()));
        
        let is_raw_tokenstream = false;
        loop {
            if let Some(token) = it.peek() {
                match token {
                    RustHtmlToken::Identifier(ident) => {
                        output.push(RustHtmlToken::Identifier(ident.clone()));
                        it.next();
                    },
                    RustHtmlToken::Literal(literal, s) => {
                        output.push(token.clone());
                        it.next();
                    },
                    RustHtmlToken::ReservedChar(c, punct) => {
                        output.push(RustHtmlToken::ReservedChar(punct.as_char(), punct.clone()));
                        it.next();
                    },
                    RustHtmlToken::Group(delimiter, stream, group) => {
                        match delimiter {
                            Delimiter::Brace => {
                                if let Some(group) = group {
                                    match parser.get_converter().convert_group(group, false, context.clone(), ct) {
                                        Ok(_) => {
                                            // println!("for_directive: {} -> {:?}", token.to_string(), output.last());
                                            it.next();
                                            break;
                                        },
                                        Err(RustHtmlError(e)) => {
                                            return Err(RustHtmlError::from_string(e.to_string()));
                                        }
                                    }   
                                } else {
                                    return Err(RustHtmlError::from_string(format!("Expected group after for directive, found '{:?}'", token)));
                                }
                            },
                            _ => {
                                output.push(token.clone());
                                it.next();
                            },
                        }
                    },
                    _ => {
                        return Err(RustHtmlError::from_string(format!("Unexpected token after for directive: {:?}", token)));
                    }
                }
                // println!("for_directive: {} -> {:?}", token.to_string(), output.last());
            } else {
                break;
            }
        }

        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}