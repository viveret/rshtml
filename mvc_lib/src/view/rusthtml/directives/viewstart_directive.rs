use std::rc::Rc;
use std::borrow::Cow;

use proc_macro2::Ident;

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::irusthtml_directive::IRustHtmlDirective;


// The "viewstart" directive is used to define a viewstart view that is evaluated and rendered before the layout view.
// if the viewstart view is not defined, the layout view is rendered without a viewstart view.
pub struct ViewStartDirective {}

impl ViewStartDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for ViewStartDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "viewstart" || name == "view_start"
    }

    fn execute(self: &Self, identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, _output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        if let Ok(param_value) = parser.convert_views_path_str(identifier.clone(), it, false) {
            parser.get_context().mut_params().insert(identifier.to_string().clone(), param_value);
            Ok(RustHtmlDirectiveResult::OkBreak)
        } else {
            return Err(RustHtmlError(Cow::Owned(format!("The \"viewstart\" directive must be followed by a valid Rust identifier."))));
        }
    }
}