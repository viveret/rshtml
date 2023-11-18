use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenStream, TokenTree};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::rusthtml::parser_parts::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;

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
    pub fn convert_mdfile_nocache_directive(
        ctx: Rc<dyn IRustHtmlParserContext>,
        identifier: &Ident,
        ident_token: &RustHtmlToken,
        parser: Rc<dyn IRustHtmlParserAll>,
        output: &mut Vec<RustHtmlToken>,
        it: Rc<dyn IPeekableRustHtmlToken>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<(), RustHtmlError<'static>> {
        // peek for prefix token
        let mut prefix_token = it.peek();
        let prefix_punct = if let TokenTree::Punct(p) = prefix_token.expect("expected punct") {
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
        match parser.get_rust_or_html_parser().peek_path_str(ctx, identifier, ident_token, it.clone()) {
            Ok(path) => {
                it.next();
                quote::quote! { #prefix_stream #path }
            },
            Err(RustHtmlError(e)) => {
                // couldn't peek path string, try parsing identity expression for dynamic path
                match parser.get_rust_parser().parse_rust_identifier_expression(false, ident_token, false, it.clone(), ct) {
                    Ok(ident_output) => {
                        let ident_output = ident_output.to_splice();
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
        output.push(RustHtmlToken::AppendToHtml(vec![RustHtmlToken::Group(proc_macro2::Delimiter::Brace, g)]));

        Ok(())
    }
}

impl IRustHtmlDirective for MarkdownFileNoCacheDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "mdfile_nocache" || name == "md_file_nocache" || name == "markdownfile_nocache" || name == "markdown_file_nocache"
    }

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        match Self::convert_mdfile_nocache_directive(context, identifier, ident_token, parser, output, it, ct) {
            Ok(_) => {
                Ok(RustHtmlDirectiveResult::OkContinue)
            },
            Err(RustHtmlError(e)) => {
                Err(RustHtmlError::from_string(format!("cannot read external markdown file nocache '{}': {}", identifier, e)))
            }
        }
    }
}