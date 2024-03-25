use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Ident;
use proc_macro2::TokenTree;

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::irusthtmlparser_version_agnostic::IRustHtmlParserVersionAgnostic;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::rusthtml::parser_parts::peekable_tokentree::VecPeekableTokenTree;
use crate::view::rusthtml::parser_parts::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;

use super::irusthtml_directive::IRustHtmlDirective;


// The "model" directive is used to assign a model type for the view.
pub struct ModelDirective {}

impl ModelDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for ModelDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "model"
    }

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, _ident_token: &TokenTree, parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, _ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // expecting type identifier
        if let Ok(type_ident) = parser.get_old_parser().parse_type_identifier(it) {
            context.set_model_type(Some(type_ident)); // .to_splice().to_vec()
            Ok(RustHtmlDirectiveResult::OkContinue)
        } else {
            Err(RustHtmlError::from_string(format!("Expected type identifier after \"{}\" directive.", identifier.to_string())))
        }
    }
    
    fn execute_new(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        match parser.get_rust_parser().parse_type_identifier(it, ct.clone()) {
            Ok(type_ident_tokens) => {
                match parser.get_converter_out().convert_rusthtmltokens_to_plain_rust(type_ident_tokens, context.clone(), ct) {
                    Ok(type_ident_rust_out) => {
                        context.set_model_type(Some(type_ident_rust_out));
                        Ok(RustHtmlDirectiveResult::OkContinue)
                    },
                    Err(RustHtmlError(err)) => Err(RustHtmlError::from_string(err.to_string()))
                }
            },
            Err(RustHtmlError(err)) => Err(RustHtmlError::from_string(err.to_string()))
        }
    }
    
    fn execute_old(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &TokenTree, parser: Rc<crate::view::rusthtml::rusthtml_parser::RustHtmlParser>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        match parser.parser.parse_type_identifier(it) {
            Ok(type_ident_tokens) => {
                context.set_model_type(Some(type_ident_tokens));
                Ok(RustHtmlDirectiveResult::OkContinue)
            },
            Err(RustHtmlError(err)) => Err(RustHtmlError::from_string(err.to_string()))
        }
    }
}