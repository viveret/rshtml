use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenTree, Delimiter};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::irusthtmlparser_version_agnostic::IRustHtmlParserVersionAgnostic;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::rusthtml::parser_parts::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::irusthtml_directive::IRustHtmlDirective;


// The "else" directive is used to render a section of the view if the previous "if" or "else if" directive evaluated to false.
pub struct ElseDirective {}

impl ElseDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for ElseDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "else"
    }

    fn execute(self: &Self, _context: Rc<dyn IRustHtmlParserContext>, _identifier: &Ident, _ident_token: &TokenTree, _parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, _ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // output directive identifier and opening bracket to output.
        
        // check if the next token is a '{'
        if let Some(TokenTree::Punct(punct)) = it.peek() {
            if punct.as_char() == '{' {
                it.next();
                // parse the body of the else block
                // let mut else_body = vec![];

                if let Some(token) = it.peek() {
                    match token.clone() {
                        TokenTree::Group(group) => {
                            if group.delimiter() == Delimiter::Brace {
                                it.next();
                                todo!("parse the else body")
                                // parser.convert_rust_directive_group_to_rusthtmltoken(group, identifier, &mut else_body, is_raw_tokenstream)?;

                            } else {
                                return Err(RustHtmlError::from_string(format!("Expected '}}' after else group and directive, found '{:?}'", group.delimiter())));
                            }
                        },
                        _ => {
                            return Err(RustHtmlError::from_string(format!("Expected group after else directive, found '{:?}'", token)));
                        }
                    }
                } else {
                    return Err(RustHtmlError::from_string(format!("Expected '}}' after else directive, found end of file")));
                }
            } else {
                return Err(RustHtmlError::from_string(format!("Expected '{{' after else directive, found '{}'", punct.as_char())));
            }
        }
        Ok(RustHtmlDirectiveResult::OkContinue)
    }
    
    fn execute_new(self: &Self, _context: Rc<dyn IRustHtmlParserContext>, _identifier: &Ident, _ident_token: &RustHtmlToken, _parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, _it: Rc<dyn IPeekableRustHtmlToken>, _ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        todo!("execute_new for else directive")
    }
    
    fn execute_old(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &TokenTree, parser: Rc<crate::view::rusthtml::rusthtml_parser::RustHtmlParser>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        todo!("execute_old for else directive")
    }
}