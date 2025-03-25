use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Ident;

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::vec_peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::parserv3::core::rusthtml_directive_result::RustHtmlDirectiveResultV3;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};

use super::irusthtml_directive::IRustHtmlDirective;


// The "model" directive is used to assign a model type for the view.
pub struct ModelDirective {}

impl ModelDirective {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_service() -> Rc<dyn IRustHtmlDirective> {
        Rc::new(ModelDirective::new())
    }
}

impl IRustHtmlDirective for ModelDirective {
    fn matches(&self, name: &String) -> bool {
        name == "model"
    }

    fn execute_new_v3(&self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        match parser.get_rust_parser().parse_type_identifier(it, ct.clone()) {
            Ok(type_ident_tokens) => {
                let type_it = Rc::new(VecPeekableRustHtmlToken::new(type_ident_tokens));
                match parser.get_converter_out().convert(type_it, ct) {
                    Ok(type_ident_rust_out) => {
                        let type_ident_rust_out = type_ident_rust_out.to_vec();
                        // println!("model type: {:?}", type_ident_rust_out);
                        context.set_model_type(Some(type_ident_rust_out));
                        Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, None))
                    },
                    Err(RustHtmlError(err)) => Err(RustHtmlError::from_string(err.to_string()))
                }
            },
            Err(RustHtmlError(err)) => Err(RustHtmlError::from_string(err.to_string()))
            
        }
    }
}