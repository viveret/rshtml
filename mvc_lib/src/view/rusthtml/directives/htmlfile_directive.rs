use std::rc::Rc;

use proc_macro2::Ident;

use crate::core::panic_or_return_error::PanicOrReturnError;
use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;

use super::irusthtml_directive::IRustHtmlDirective;


// The "htmlfile" directive is used to render html from a file.
pub struct HtmlFileDirective {}

impl HtmlFileDirective {
    pub fn new() -> Self {
        Self {}
    }

    // convert an external HTML directive to RustHtml tokens.
    // identifier: the identifier to convert.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    pub fn convert_externalhtml_directive(identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<(), RustHtmlError<'static>> {
        match parser.convert_views_path_str(identifier.clone(), it.clone(), parser.get_context().get_is_raw_tokenstream()) {
            Ok(path) => {
                match std::fs::File::open(path.as_str()) {
                    Ok(_f) => {
                        Self::convert_externalhtml_directive_file(identifier, parser, output, it, path)?;
                    },
                    Err(e) => {
                        return PanicOrReturnError::panic_or_return_error(parser.get_context().get_should_panic_or_return_error(), format!("(@{}) cannot read external HTML file '{}', could not open: {:?}", identifier, path, e));
                    }
                }
                Ok(())
            },
            Err(RustHtmlError(e)) => {
                return PanicOrReturnError::panic_or_return_error(parser.get_context().get_should_panic_or_return_error(), format!("(@{}) cannot read external HTML file, could not parse path: {}", identifier, e));
            }
        }
    }

    fn convert_externalhtml_directive_file(_ident: &Ident, _parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, path: String) -> Result<(), RustHtmlError<'static>> {
        match std::fs::read_to_string(path.clone()) {
            Ok(html) => {
                it.next();
                output.push(RustHtmlToken::AppendToHtml(vec![RustHtmlToken::Literal(None, Some(html))]));
                Ok(())
            },
            Err(e) => {
                PanicOrReturnError::panic_or_return_error(false, format!("cannot read external HTML file {}: {:?}", path, e))
            }
        }
    }
}

impl IRustHtmlDirective for HtmlFileDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "htmlfile" || name == "html_file"
    }

    fn execute(self: &Self, identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        Self::convert_externalhtml_directive(identifier, parser, output, it)?;
        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}