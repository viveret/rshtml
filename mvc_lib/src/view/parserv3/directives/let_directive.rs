use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenTree};

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::vec_peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::{RustHtmlDirectiveResult, RustHtmlDirectiveResultV3};
use crate::view::parserv3::parserv3::IParserV3;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};

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
    pub fn parse_let(parser: Rc<dyn IParserV3>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        loop
        {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_cancellationtoken(ct));
            }

            match it.next() {
                Some(ref token) => {
                    match token {
                        RustHtmlToken::ReservedChar(c, punct) => {
                            let c = punct.as_char();
                            output.push(RustHtmlToken::ReservedChar(c, punct.clone()));
                            if c == ';' {
                                break;
                            }
                        },
                        _ => {
                            output.push(token.clone())
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
    fn matches(&self, name: &String) -> bool {
        name == "let"
    }
    
    fn execute_new_v3(&self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        let mut output = vec![];
        output.push(RustHtmlToken::Identifier(identifier.clone()));
        if let Ok(_) = Self::parse_let(parser, &mut output, it, context.clone(), ct.clone()) {
            let output_stream = Rc::new(VecPeekableRustHtmlToken::new(output));
            Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, Some(output_stream)))
        } else {
            return Err(RustHtmlError::from_str("Error parsing let statement"));
        }
    }
}