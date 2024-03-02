use std::rc::Rc;

use proc_macro2::{Ident, TokenStream, TokenTree};

use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::rusthtml::parser_parts::peekable_tokentree::{IPeekableTokenTree, StreamPeekableTokenTree};
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
        // peek for prefix token
        let mut prefix_token = it.peek();
        let prefix_punct = if let TokenTree::Punct(p) = prefix_token.unwrap() {
            prefix_token = it.next();
            Some(p)
        } else {
            prefix_token = None;
            None
        };
        let prefix_stream = if let Some(prefix_punct) = prefix_punct {
            quote::quote! { #prefix_punct }
        } else {
            quote::quote! {}
        };
        let open_inner_tokenstream = 
        match parser.parse_string_with_quotes(true, identifier.clone(), it.clone()) {
            Ok(path) => {
                it.next();
                quote::quote! { #prefix_stream #path }
            },
            Err(RustHtmlError(e)) => {
                // couldn't peek path string, try parsing identity expression for dynamic path
                match parser.extract_identifier_expression(false, ident_token, false, it.clone(), false) {
                    Ok(ident_output) => {
                        if ident_output.len() > 0 {
                            // might need to prepend ident_token?
                            let mut ident_output_final = vec![];
                            if let Some(ref x) = prefix_token {
                                ident_output_final.push(x.clone());
                            }
                            ident_output_final.extend_from_slice(&ident_output);
                            TokenStream::from_iter(ident_output_final.into_iter())
                        } else {
                            return Err(RustHtmlError::from_string(format!("cannot read external markdown file nocache '{}', could not parse path: {}", identifier, e)));
                        }
                    },
                    Err(RustHtmlError(e2)) => {
                        return Err(RustHtmlError::from_string(format!("cannot read external markdown file nocache '{}', could not parse path: {}", identifier, e2)));
                    }
                }
            }
        };

        // let path = format!("{}", open_inner_tokenstream);
        let code = quote::quote! {
            view_context.get_markdown_file_nocache(#open_inner_tokenstream)
        };

        let g = proc_macro2::Group::new(proc_macro2::Delimiter::Brace, code);
        let group_stream = Rc::new(StreamPeekableTokenTree::new(code));
        match parser.parse_tokenstream_to_rusthtmltokens(false, group_stream, false) {
            Ok(tokens) => {
                let group_stream_converted = Rc::new(VecPeekableRustHtmlToken::new(tokens));
                output.push(RustHtmlToken::AppendToHtml(vec![RustHtmlToken::Group(proc_macro2::Delimiter::Brace, group_stream_converted, Some(g))]));
                Ok(())
            },
            Err(RustHtmlError(e)) => {
                Err(RustHtmlError::from_string(format!("cannot read external markdown file nocache '{}', could not parse: {}", identifier, e)))
            }
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