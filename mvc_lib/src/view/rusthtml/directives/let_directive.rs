use std::rc::Rc;

use proc_macro2::{Ident, TokenTree};

use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;

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
    pub fn parse_let(parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<(), RustHtmlError> {
        loop
        {
            match it.next() {
                Some(token) => {
                    match token {
                        TokenTree::Punct(punct) => {
                            let c = punct.as_char();
                            output.push(RustHtmlToken::ReservedChar(c, punct));
                            if c == ';' {
                                break;
                            }
                        },
                        _ => {
                            match parser.convert_copy(token, output) {
                                Ok(_) => {},
                                Err(RustHtmlError(e)) => {
                                    return Err(RustHtmlError::from_string(e.as_ref().to_string()));
                                }
                            }
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

    fn execute(self: &Self, identifier: &Ident, _ident_token: &TokenTree, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        output.push(RustHtmlToken::Identifier(identifier.clone()));
        if let Ok(_) = Self::parse_let(parser, output, it) {
            Ok(RustHtmlDirectiveResult::OkContinue)
        } else {
            return Err(RustHtmlError::from_str("Error parsing let statement"));
        }
    }
}