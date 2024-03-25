use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenTree};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::irusthtmlparser_version_agnostic::IRustHtmlParserVersionAgnostic;
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
    pub fn parse_let(parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        loop
        {
            match it.next() {
                Some(ref token) => {
                    match token {
                        TokenTree::Punct(punct) => {
                            let c = punct.as_char();
                            output.push(RustHtmlToken::ReservedChar(c, punct.clone()));
                            if c == ';' {
                                break;
                            }
                        },
                        _ => {
                            match parser.get_converter().convert(token.clone(), context.clone(), ct.clone()) {
                                Ok(converted) => {
                                    output.push(converted)
                                },
                                Err(e) => {
                                    return Err(e.clone());
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

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, _ident_token: &TokenTree, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        output.push(RustHtmlToken::Identifier(identifier.clone()));
        if let Ok(_) = Self::parse_let(parser, output, it, context.clone(), ct.clone()) {
            Ok(RustHtmlDirectiveResult::OkContinue)
        } else {
            return Err(RustHtmlError::from_str("Error parsing let statement"));
        }
    }
    
    fn execute_new(self: &Self, _context: Rc<dyn IRustHtmlParserContext>, _identifier: &Ident, _ident_token: &RustHtmlToken, _parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, _it: Rc<dyn IPeekableRustHtmlToken>, _ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        todo!("execute_new let directive")
    }
    
    fn execute_old(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &TokenTree, parser: Rc<crate::view::rusthtml::rusthtml_parser::RustHtmlParser>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        todo!("execute_old let directive")
    }
}