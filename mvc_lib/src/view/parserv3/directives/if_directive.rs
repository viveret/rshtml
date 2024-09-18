use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenTree, Delimiter};

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
                        return match parser.get_converter_middle().convert(s.clone(), context.clone(), ct) {
                            Ok(group_converted) => {
                                context.pop_is_in_html_mode();
                                tokens.push(RustHtmlToken::Group(*d, group_converted, None));
                                let tokens_stream = Rc::new(VecPeekableRustHtmlToken::new(tokens));
                                Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, Some(tokens_stream)))
                            },
                            Err(e) => {
                                Err(e)
                            }
                        }
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