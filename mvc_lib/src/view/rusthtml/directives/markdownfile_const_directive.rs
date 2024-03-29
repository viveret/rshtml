use std::io::Read;
use std::rc::Rc;

use proc_macro2::Ident;
use proc_macro2::TokenTree;

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;

use super::irusthtml_directive::IRustHtmlDirective;


// The "mdfile" directive is used to render markdown from a file.
pub struct MarkdownFileConstDirective {}

impl MarkdownFileConstDirective {
    pub fn new() -> Self {
        Self {}
    }

    // read and convert a Markdown file directly to RustHtml tokens.
    // identifier: the identifier to convert.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_mdfile_const_directive(identifier: &Ident, ident_token: &TokenTree, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<(), RustHtmlError<'static>> {
        match parser.next_path_str(identifier, ident_token, it, parser.get_context().get_is_raw_tokenstream()) {
            Ok(path) => {
                let code = quote::quote! {
                    match view_context.open_data_file(#path) {
                        Ok(mut f) => {
                            let mut buffer = String::new();
                            f.read_to_string(&mut buffer).expect("could not read markdown file");
                            match comrak::markdown_to_html(&buffer, &comrak::ComrakOptions::default()) {
                                Some(x) => {
                                    if x == 0 {
                                        panic!("beep boop bop");
                                    } else {
                                        Ok(HtmlString::new_from_html(buffer))
                                    }
                                },
                                None => {
                                    panic!("beep beep beep");
                                }
                            }
                        },
                        Err(e) => {
                            panic!("cannot read external markdown file const '{}', could not open: {:?}", #path, e);
                        }
                    }
                };
        
                let g = proc_macro2::Group::new(proc_macro2::Delimiter::Brace, code);
                output.push(RustHtmlToken::AppendToHtml(vec![RustHtmlToken::Group(proc_macro2::Delimiter::Brace, g)]));
        
                Ok(())
            },
            Err(RustHtmlError(e)) => {
                Err(RustHtmlError::from_string(format!("cannot read external markdown file const '{}', could not parse path: {}", identifier, e)))
            }
        }
    }
}

impl IRustHtmlDirective for MarkdownFileConstDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "mdfile_const" || name == "markdownfile_const"
    }

    fn execute(self: &Self, identifier: &Ident, ident_token: &TokenTree, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        match Self::convert_mdfile_const_directive(identifier, ident_token, parser, output, it) {
            Ok(_) => {
                Ok(RustHtmlDirectiveResult::OkContinue)
            },
            Err(RustHtmlError(e)) => {
                Err(RustHtmlError::from_string(format!("cannot read external markdown file const '{}': {}", identifier, e)))
            }
        }
    }
}