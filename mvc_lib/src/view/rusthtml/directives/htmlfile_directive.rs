use std::rc::Rc;

use proc_macro2::{Ident, Group, Delimiter, TokenTree};

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
        if let Ok(path) = parser.convert_views_path_str(identifier.clone(), it, parser.get_context().get_is_raw_tokenstream()) {
            match std::fs::File::open(path.as_str()) {
                Ok(_f) => {
                    let mut inner_tokens = vec![];
                    let result = parser.convert_copy(TokenTree::Group(Group::new(Delimiter::None, quote::quote! { 
                        let content = std::fs::read_to_string(path).unwrap();
                        self.convert_rusthtmltextnode_to_tokentree(&content, &span, output, it)
                    })), &mut inner_tokens);
                    if let Err(RustHtmlError(e)) = result {
                        return Err(RustHtmlError::from_string(e.to_string()));
                    }
                    output.push(RustHtmlToken::AppendToHtml(inner_tokens));
                },
                Err(e) => {
                    return PanicOrReturnError::panic_or_return_error(parser.get_context().get_should_panic_or_return_error(), format!("cannot read external HTML file '{}', could not open: {:?}", path, e));
                }
            }
            Ok(())
        } else {
            return PanicOrReturnError::panic_or_return_error(parser.get_context().get_should_panic_or_return_error(), format!("cannot read external HTML file '{}', could not parse path", identifier));
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