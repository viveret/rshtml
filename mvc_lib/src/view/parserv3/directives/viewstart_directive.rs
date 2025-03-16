use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Ident;

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::{RustHtmlDirectiveResult, RustHtmlDirectiveResultV3};
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::irusthtml_directive::IRustHtmlDirective;


// The "viewstart" directive is used to define a viewstart view that is evaluated and rendered before the layout view.
// if the viewstart view is not defined, the layout view is rendered without a viewstart view.
pub struct ViewStartDirective {}

impl ViewStartDirective {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_service() -> Rc<dyn IRustHtmlDirective> {
        Rc::new(ViewStartDirective::new())
    }
}

impl IRustHtmlDirective for ViewStartDirective {
    fn matches(&self, name: &String) -> bool {
        name == "viewstart"
    }

    fn execute_new_v3(&self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        match parser.get_rust_parser().parse_string_with_quotes(false, identifier, it.clone()) {
            Ok(param_value) => {
                context.mut_params().insert("viewstart".to_string(), param_value);
                Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, None))
            },
            Err(RustHtmlError(e)) => {
                return Err(RustHtmlError::from_string(format!("The \"viewstart\" directive failed: ({})", e)));
            }
        }
    }
}