use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Ident;

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::vec_peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::{RustHtmlDirectiveResult, RustHtmlDirectiveResultV3};
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};

use super::irusthtml_directive::IRustHtmlDirective;


// the "for" directive is used to iterate over a collection and render a section of the view for each item in the collection.
pub struct ForDirective {}

impl ForDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for ForDirective {
    fn matches(&self, name: &String) -> bool {
        name == "for"
    }

    fn execute_new_v3(&self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        let mut output = vec![ident_token.clone()];
        
        loop {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_cancellationtoken(ct));
            }
            if let Some(token) = it.next() {
                match &token {
                    RustHtmlToken::Group(delimiter, stream, group) => {
                        match delimiter {
                            proc_macro2::Delimiter::Brace => {
                                // recurse
                                match parser.get_converter_middle().convert(stream.clone(), context.clone(), ct.clone()) {
                                    Ok(new_input) => {
                                        output.push(RustHtmlToken::Group(*delimiter, new_input, None));
                                        break;
                                    },
                                    Err(e) => {
                                        return Err(e);
                                    }
                                }
                            },
                            _ => {
                                output.push(token.clone());
                            },
                        }
                    },
                    _ => {
                        output.push(token.clone());
                    }
                }
            } else {
                break;
            }
        }

        let output_stream = Rc::new(VecPeekableRustHtmlToken::new(output));
        Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, Some(output_stream)))
    }
}