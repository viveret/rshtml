use std::rc::Rc;

use proc_macro2::{Ident, Delimiter, Group, TokenStream};

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;

use super::irusthtml_directive::IRustHtmlDirective;


// The "mdfile_nocache" directive is used to render markdown from a file without caching.
pub struct MarkdownFileNoCacheDirective {}

impl MarkdownFileNoCacheDirective {
    pub fn new() -> Self {
        Self {}
    }

    // generate Rust code that reads and converts a Markdown file to HTML without caching.
    // identifier: the identifier to convert.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    pub fn convert_mdfile_nocache_directive(identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<(), RustHtmlError<'static>> {
        match parser.convert_path_str(identifier.clone(), it.clone(), parser.get_context().get_is_raw_tokenstream()) {
            Ok(path) => {
                let code = quote::quote! {
                    match view_context.open_data_file(#path) {
                        Ok(mut f) => {
                            let mut buffer = String::new();
                            f.read_to_string(&mut buffer).expect("could not read markdown file");
                            comrak::markdown_to_html(&buffer, &comrak::ComrakOptions::default())
                        },
                        Err(e) => {
                            panic!("cannot read external markdown file '{}', could not open: {:?}", #path, e);
                        }
                    }
                };
        
                let g = proc_macro2::Group::new(proc_macro2::Delimiter::Brace, code);
                output.push(RustHtmlToken::AppendToHtml(vec![RustHtmlToken::Group(proc_macro2::Delimiter::Brace, g)]));
        
                Ok(())
            },
            Err(RustHtmlError(e)) => {
                return Err(RustHtmlError::from_string(format!("cannot read external markdown file '{}', could not parse path: {}", identifier, e)));
            }
        }

    }
}

impl IRustHtmlDirective for MarkdownFileNoCacheDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "mdfile_nocache" || name == "md_file_nocache" || name == "markdownfile_nocache" || name == "markdown_file_nocache"
    }

    fn execute(self: &Self, identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        if let Ok(_) = Self::convert_mdfile_nocache_directive(identifier, parser, output, it) {
            Ok(RustHtmlDirectiveResult::OkContinue)
        } else {
            Err(RustHtmlError::from_string(format!("cannot read external markdown file '{}'", identifier)))
        }
    }
}