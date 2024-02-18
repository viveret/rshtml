use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenTree};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::rusthtml::parser_parts::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;

use super::irusthtml_directive::IRustHtmlDirective;


// The "let" directive is used to assign a variable in the view. it is similar to the "let" keyword in Rust.
pub struct LetDirective {}

impl LetDirective {
    pub fn new() -> Self {
        Self {}
    }

    // parse a Rust let statement and convert it to RustHtml tokens.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    pub fn parse_let(parser: Rc<dyn IRustHtmlParserAll>, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<(), RustHtmlError<'static>> {
        loop
        {
            match it.next() {
                Some(token) => {
                    match token {
                        RustHtmlToken::ReservedChar(c, punct) => {
                            ctx.push_output_token(token.clone());
                            if *c == ';' {
                                break;
                            }
                        },
                        _ => {
                            ctx.push_output_token(token.clone())
                        }
                    }
                },
                None => {
                    return Err(RustHtmlError::from_str("Unexpected end of let statement"));
                }
            }
        }
        Ok(())
    }
}

impl IRustHtmlDirective for LetDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "let"
    }

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, _ident_token: &RustHtmlToken, parser: Rc<dyn IRustHtmlParserAll>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        context.push_output_token(RustHtmlToken::Identifier(identifier.clone()));
        if let Ok(_) = Self::parse_let(parser, context, it) {
            Ok(RustHtmlDirectiveResult::OkContinue)
        } else {
            return Err(RustHtmlError::from_str("Error parsing let statement"));
        }
    }
}