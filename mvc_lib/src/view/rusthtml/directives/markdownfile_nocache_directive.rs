use std::rc::Rc;
use std::borrow::Cow;

use proc_macro::{Ident, TokenTree, Delimiter, Group, TokenStream};

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
        // could be literal or ident
        if let Ok(path_tokens) = parser.convert_string_or_ident(it, parser.get_context().get_is_raw_tokenstream()) {
            if let Ok(x) = parser.convert_ident_and_punct_and_group_or_literal_to_tokenstream(&path_tokens) {
                let path = proc_macro2::TokenStream::from(x);
                let tokenstream = quote::quote! {
                    let mut path = std::path::PathBuf::new();
                    let cwd = std::env::current_dir().expect("could not get current directory");
                    path.push(cwd);
                    path.push(#path);

                    match std::fs::File::open(path.to_str().expect("could not get combined path").to_string()) {
                        Ok(mut f) => {
                            let mut buffer = String::new();
                            f.read_to_string(&mut buffer).expect("could not read markdown file");
                            html_output.write_html_str(comrak::markdown_to_html(&buffer, &comrak::ComrakOptions::default()).as_str());
                        },
                        Err(e) => {
                            println!("convert_mdfile_nocache_directive: could not find {}", #path);
                            return Err(RustHtmlError(Cow::Owned(format!("cannot read external markdown file '{}', could not open: {:?}", #path, e))));
                        }
                    }
                };
                output.push(RustHtmlToken::Group(Delimiter::None, Group::new(Delimiter::None, TokenStream::from(tokenstream))));

                Ok(())
            } else {
                return Err(RustHtmlError::from_string(format!("cannot read external markdown file '{}', could not parse path", identifier)));
            }
        } else {
            return Err(RustHtmlError::from_string(format!("cannot read external markdown file '{}', could not parse path", identifier)));
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