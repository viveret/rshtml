use std::rc::Rc;

use proc_macro2::{Ident, Delimiter, Group, TokenStream, TokenTree};

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
    pub fn convert_mdfile_nocache_directive(identifier: &Ident, ident_token: &TokenTree, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<(), RustHtmlError<'static>> {
        let mut open_inner_tokenstream: Option<TokenStream> = None;

        match parser.peek_path_str(identifier, ident_token, it.clone(), parser.get_context().get_is_raw_tokenstream()) {
            Ok(path) => {
                it.next();
                open_inner_tokenstream = Some(quote::quote! { #path });
            },
            Err(RustHtmlError(e)) => {
                // couldn't peek path string, try parsing identity expression for dynamic path
                match parser.extract_identifier_expression(false, ident_token, false, it.clone(), false) {
                    Ok(ident_output) => {
                        if ident_output.len() > 0 {
                            // might need to prepend ident_token?
                            let mut ident_output_final = vec![];
                            ident_output_final.push(ident_token.clone());
                            ident_output_final.extend_from_slice(&ident_output);
                            open_inner_tokenstream = Some(TokenStream::from_iter(ident_output_final.into_iter()));
                        } else {
                            return Err(RustHtmlError::from_string(format!("cannot read external markdown file nocache '{}', could not parse path: {}", identifier, e)));
                        }
                    },
                    Err(RustHtmlError(e2)) => {
                        return Err(RustHtmlError::from_string(format!("cannot read external markdown file nocache '{}', could not parse path: {}", identifier, e2)));
                    }
                }
            }
        }

        if let Some(open_inner_tokenstream) = open_inner_tokenstream {
            let path = format!("{}", open_inner_tokenstream);
            println!("path: {}", path);
            let code = quote::quote! {
                match view_context.open_data_file(#open_inner_tokenstream) {
                    Ok(mut f) => {
                        let mut buffer = String::new();
                        match f.read_to_string(&mut buffer) {
                            Some(x) => {
                                match comrak::markdown_to_html(&buffer, &comrak::ComrakOptions::default()) {
                                    Some(n) => {
                                        if n > 0 {
                                            HtmlString::new_from_html(buffer)
                                        } else {
                                            panic!("Could not convert markdown to html at {} (no bytes written)", #path);
                                        }
                                    },
                                    None => {
                                        panic!("Could not convert markdown to html at {}", #path);
                                    }
                                }
                            },
                            None => {
                                panic!("Could not read data at {}", #path);
                            }
                        }
                    },
                    Err(e) => {
                        panic!("cannot read external markdown file nocache '{}', could not open: {:?}", #path, e);
                    }
                }
            };

            let g = proc_macro2::Group::new(proc_macro2::Delimiter::Brace, code);
            output.push(RustHtmlToken::AppendToHtml(vec![RustHtmlToken::Group(proc_macro2::Delimiter::Brace, g)]));

            Ok(())
        } else {
            // no valid tokenstream for markdown directive
            panic!("no valid tokenstream for markdown directive");
        }
    }
}

impl IRustHtmlDirective for MarkdownFileNoCacheDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "mdfile_nocache" || name == "md_file_nocache" || name == "markdownfile_nocache" || name == "markdown_file_nocache"
    }

    fn execute(self: &Self, identifier: &Ident, ident_token: &TokenTree, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        match Self::convert_mdfile_nocache_directive(identifier, ident_token, parser, output, it) {
            Ok(_) => {
                Ok(RustHtmlDirectiveResult::OkContinue)
            },
            Err(RustHtmlError(e)) => {
                Err(RustHtmlError::from_string(format!("cannot read external markdown file nocache '{}': {}", identifier, e)))
            }
        }
    }
}