use std::rc::Rc;

use proc_macro2::Ident;
use proc_macro2::TokenTree;

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
    pub fn convert_externalhtml_directive(identifier: &Ident, identifier_token: &TokenTree, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<(), RustHtmlError<'static>> {
        // match parser.parse_string_with_quotes(false, identifier.clone(), it.clone()) {
        match parser.parse_string_with_quotes(false, identifier.clone(), it.clone()) {
            Ok(path) => {
                let code = quote::quote! {
                    match view_context.open_view_file(#path) {
                        Ok(mut f) => {
                            let mut buffer = String::new();
                            f.read_to_string(&mut buffer).expect("could not read HTML file");
                            buffer
                        },
                        Err(e) => {
                            let cwd = std::env::current_dir().unwrap();
                            return Err(RustHtmlError::from_string(format!("cannot read external HTML file '{}' in '{:?}', could not open: {:?}", #path, cwd, e)));
                        }
                    }
                };
                let g = proc_macro2::Group::new(proc_macro2::Delimiter::Brace, code);
                output.push(RustHtmlToken::AppendToHtml(vec![RustHtmlToken::Group(proc_macro2::Delimiter::Brace, g)]));
        
                Ok(())
            },
            Err(RustHtmlError(e)) => {
                Err(RustHtmlError::from_string(format!("(@{}) cannot read external HTML file, could not parse path: {}", identifier, e)))
            }
        }
    }
}

impl IRustHtmlDirective for HtmlFileDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "htmlfile" || name == "html_file"
    }

    fn execute(self: &Self, identifier: &Ident, ident_token: &TokenTree, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        Self::convert_externalhtml_directive(identifier, ident_token, parser, output, it)?;
        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}