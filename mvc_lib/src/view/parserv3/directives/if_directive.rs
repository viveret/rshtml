use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Delimiter;
use proc_macro2::Ident;

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::vec_peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::{RustHtmlDirectiveResult, RustHtmlDirectiveResultV3};
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::irusthtml_directive::IRustHtmlDirective;


// The "if" directive is used to conditionally render a section of the view.
pub struct IfDirective {}

impl IfDirective {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_for_else_or_else_if(&self, context: Rc<dyn IRustHtmlParserContext>, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut tokens = vec![];

        while let Some(else_token) = it.peek() {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_cancellationtoken(ct));
            }

            if let RustHtmlToken::Identifier(i) = &else_token {
                if i.to_string() == "else" {
                    it.next(); // else
                    tokens.push(else_token.clone());
                    // check if is block, "if" ident, or expression
                    if let Some(else_type_token) = it.peek() {
                        match &else_type_token {
                            RustHtmlToken::Group(d, s, g) if *d == Delimiter::Brace => {
                                it.next(); // block
                                context.push_is_in_html_mode(false);
                                let group_converted = parser.get_converter_middle().convert(s.clone(), context.clone(), ct.clone())?;
                                context.pop_is_in_html_mode();
                                tokens.push(RustHtmlToken::Group(d.clone(), group_converted, None));
                                break;
                            },
                            RustHtmlToken::Identifier(i) => {
                                if i.to_string() == "if" {
                                    it.next(); // if
                                    tokens.push(else_type_token.clone());

                                    // block or expression
                                    if let Some(block_or_expression_token) = it.peek() {
                                        match &block_or_expression_token {
                                            RustHtmlToken::Group(d, s, g) if *d == Delimiter::Brace => {
                                                // tokens.push(block_or_expression_token.clone());
                                                context.push_is_in_html_mode(false);
                                                let group_converted = parser.get_converter_middle().convert(s.clone(), context.clone(), ct.clone())?;
                                                context.pop_is_in_html_mode();
                                                tokens.push(RustHtmlToken::Group(d.clone(), group_converted, None));
                                                it.next();
                                                break;
                                            },
                                            RustHtmlToken::Identifier(i) => {
                                                panic!("todo else if expression");
                                            },
                                            _ => {
                                                panic!("todo else if unexpected token {:?}", block_or_expression_token);
                                            }
                                        }
                                    }
                                } else {
                                    panic!("todo else expression")
                                }
                            },
                            _ => {
                                panic!("don't know what to do after if block with token: {:?}", else_type_token);
                            }
                        }
                    } else {
                        panic!("expected token after else");
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(tokens)
    }
}

impl IRustHtmlDirective for IfDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "if"
    }

    // need to check for else if and else
    fn execute_new_v3(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        let mut tokens = vec![ident_token.clone()];
        // expect anything until group with {}
        while let Some(token) = it.next() {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_cancellationtoken(ct));
            }
            
            match &token {
                RustHtmlToken::Group(d, s, g) => {
                    if *d == Delimiter::Brace {
                        // new context within if block
                        context.push_is_in_html_mode(false);
                        let group_converted = parser.get_converter_middle().convert(s.clone(), context.clone(), ct.clone())?;
                        context.pop_is_in_html_mode();
                        tokens.push(RustHtmlToken::Group(*d, group_converted, None));

                        let other_branches = self.check_for_else_or_else_if(context, parser, it, ct)?;
                        tokens.extend_from_slice(&other_branches);

                        let tokens_stream = Rc::new(VecPeekableRustHtmlToken::new(tokens));
                        return Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, Some(tokens_stream)));
                    } else {
                        tokens.push(token);
                    }
                },
                _ => {
                    tokens.push(token);
                }
            }
        }

        Err(RustHtmlError::from_str("Unexpected end of input while parsing if directive"))
    }
}