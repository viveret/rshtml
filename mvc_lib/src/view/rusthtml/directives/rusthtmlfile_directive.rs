use std::rc::Rc;

use proc_macro2::Ident;

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
        if let Ok(path) = parser.convert_path_str(identifier.clone(), it.clone(), parser.get_context().get_is_raw_tokenstream()) {
            let code = quote::quote!{
                let v = view_context.get_view(#path);
                v.render()
            };
            let g = proc_macro2::Group::new(proc_macro2::Delimiter::Brace, code);
            output.push(RustHtmlToken::AppendToHtml(vec![RustHtmlToken::Group(proc_macro2::Delimiter::Brace, g)]));

            Ok(())
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
        // do match instead of if let to access error
        match Self::convert_externalrusthtml_directive(identifier, parser, output, it) {
            Ok(_) => Ok(RustHtmlDirectiveResult::OkContinue),
            Err(e) => Err(e)
        }
    }
}