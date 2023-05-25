use std::{borrow::Cow, rc::Rc};

use proc_macro::{Ident, TokenTree};

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;

use super::irusthtml_directive::IRustHtmlDirective;


// The "rshtmlfile" directive is used to include a RustHtml file.
pub struct RustHtmlFileDirective {}

impl RustHtmlFileDirective {
    pub fn new() -> Self {
        Self {}
    }

    // convert an external Rust HTML directive to RustHtml tokens.
    // identifier: the identifier to convert.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    pub fn convert_externalrusthtml_directive(identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<(), RustHtmlError<'static>> {
        if let Ok(path) = parser.convert_path_str(identifier.clone(), it, parser.get_context().get_is_raw_tokenstream()) {
            match std::fs::File::open(&path) {
                Ok(_f) => {
                    // output.push(RustHtmlToken::ExternalRustHtml(path.clone(), identifier.span()));
                    if let Ok(_) = parser.expand_external_tokenstream(&path, output) {
                        Ok(())
                    } else {
                        Err(RustHtmlError::from_string(format!("cannot read external Rust HTML file '{}', could not expand: {:?}", path, identifier)))
                    }
                },
                Err(e) => {
                    Err(RustHtmlError::from_string(format!("cannot read external Rust HTML file '{}', could not open: {:?}", path, e)))
                }
            }
        } else {
            Err(RustHtmlError::from_string(format!("cannot read external Rust HTML file '{}', could not parse path", identifier)))
        }
    }
}

impl IRustHtmlDirective for RustHtmlFileDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "rshtmlfile" || name == "rusthtmlfile"
    }

    fn execute(self: &Self, identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        if let Ok(_) = Self::convert_externalrusthtml_directive(identifier, parser, output, it) {
            Ok(RustHtmlDirectiveResult::OkContinue)
        } else {
            Err(RustHtmlError::from_string(format!("cannot read external Rust HTML file '{}', could not parse path", identifier)))
        }
    }
}