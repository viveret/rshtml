use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenTree, Delimiter};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::rusthtml::parser_parts::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;

use super::irusthtml_directive::IRustHtmlDirective;



// the "while" directive is used to create a while loop in the view.
// It will loop over the contents of the directive until the condition is false.
pub struct WhileDirective {}

impl WhileDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for WhileDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "while"
    }

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn IRustHtmlParserAll>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        output.push(RustHtmlToken::Identifier(identifier.clone()));
        
        // read until we reach the loop body {}
        loop {
            if let Some(token) = it.peek() {
                match token {
                    RustHtmlToken::Identifier(ident) => {
                        output.push(token.clone());
                        it.next();
                    },
                    RustHtmlToken::Literal(literal, s) => {
                        output.push(token.clone());
                        it.next();
                    },
                    RustHtmlToken::ReservedChar(c, punct) => {
                        output.push(token.clone());
                        it.next();
                    },
                    RustHtmlToken::Group(delimiter, stream, group) => {
                        match delimiter {
                            Delimiter::Brace => {
                                match parser.get_converter().convert_group(&group.clone().unwrap(), false, context, ct) {
                                    Ok(group) => {
                                        output.push(group);
                                        break;
                                    },
                                    Err(RustHtmlError(err)) => {
                                        return Err(RustHtmlError::from_string(err.to_string()));
                                    }
                                }
                            },
                            _ => {
                                panic!("unexpected group delimiter: {:?}", delimiter);
                            }
                        }
                    },
                    _ => {
                        return Err(RustHtmlError::from_string(format!("unexpected token: {:?}", token)));
                    }
                }
            } else {
                break;
            }
        }

        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}