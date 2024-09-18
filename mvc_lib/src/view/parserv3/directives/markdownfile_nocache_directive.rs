use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenTree};

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::{RustHtmlDirectiveResult, RustHtmlDirectiveResultV3};
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};

use super::irusthtml_directive::IRustHtmlDirective;


// The "mdfile_nocache" directive is used to render markdown from a file without caching.
pub struct MarkdownFileNoCacheDirective {}

impl MarkdownFileNoCacheDirective {
    pub fn new() -> Self {
        Self {}
    }
/*
    // generate Rust code that reads and converts a Markdown file to HTML without caching.
    // identifier: the identifier to convert.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    pub fn convert_mdfile_nocache_directive(
        ctx: Rc<dyn IRustHtmlParserContext>,
        identifier: &Ident,
        ident_token: &TokenTree,
        parser: Rc<dyn IRustHtmlParserAll>,
        output: &mut Vec<RustHtmlToken>,
        it: Rc<dyn IPeekableTokenTree>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<(), RustHtmlError> {
        // peek for prefix token
        let mut _prefix_token = it.peek();
        let prefix_punct = if let TokenTree::Punct(p) = _prefix_token.expect("could not peek prefix token") {
            _prefix_token = it.next();
            Some(p)
        } else {
            _prefix_token = None;
            None
        };
        let prefix_stream = if let Some(prefix_punct) = prefix_punct {
            quote::quote! { #prefix_punct }
        } else {
            quote::quote! {}
        };
        let open_inner_tokenstream = 
        match parser.get_old_parser().peek_path_str(ctx.clone(), identifier, ident_token, it.clone(), false, ct.clone()) {
            Ok(path) => {
                it.next();
                quote::quote! { #prefix_stream #path }
            },
            Err(RustHtmlError(_e)) => {
                // couldn't peek path string, try parsing identity expression for dynamic path
                unimplemented!("TODO: implement dynamic path parsing for mdfile_nocache directive")
                // match parser.get_old_parser().parse_identifier_expression(false, identifier, ident_token, false, output, it.clone(), false, ct) {
                //     Ok(()) => { // ident_output
                //         let ident_output = output;
                //         if ident_output.len() > 0 {
                //             // might need to prepend ident_token?
                //             let mut ident_output_final = vec![];
                //             if let Some(ref x) = prefix_token {
                //                 ident_output_final.push(x.clone());
                //             }
                //             ident_output_final.extend_from_slice(&ident_output);
                //             TokenStream::from_iter(ident_output_final.into_iter())
                //         } else {
                //             return Err(RustHtmlError::from_string(format!("cannot read external markdown file nocache '{}', could not parse path: {}", identifier, e)));
                //         }
                //     },
                //     Err(RustHtmlError(e2)) => {
                //         return Err(RustHtmlError::from_string(format!("cannot read external markdown file nocache '{}', could not parse path: {}", identifier, e2)));
                //     }
                // }
            }
        };

        // let path = format!("{}", open_inner_tokenstream);
        let code = quote::quote! {
            view_context.get_markdown_file_nocache(#open_inner_tokenstream)
        };

        let g = proc_macro2::Group::new(proc_macro2::Delimiter::Brace, code);
        match parser.get_converter().convert_group(&g, false, ctx.clone(), ct.clone()) {
            Ok(gconverted) => {
                output.push(RustHtmlToken::AppendToHtml(vec![gconverted]));
                Ok(())
            },
            Err(e) => {
                return Err(RustHtmlError::from_string(format!("cannot read external markdown file nocache '{}': {}", identifier, e)));
            }
        }
    }

    pub fn convert_mdfile_nocache_directive_old(
        ctx: Rc<dyn IRustHtmlParserContext>,
        identifier: &Ident,
        ident_token: &TokenTree,
        parser: Rc<crate::view::rusthtml::rusthtml_parser::RustHtmlParser>,
        output: &mut Vec<RustHtmlToken>,
        it: Rc<dyn IPeekableTokenTree>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<(), RustHtmlError> {
        // peek for prefix token
        let mut _prefix_token = it.peek();
        let prefix_punct = if let TokenTree::Punct(p) = _prefix_token.expect("could not peek prefix token") {
            _prefix_token = it.next();
            Some(p)
        } else {
            _prefix_token = None;
            None
        };
        let prefix_stream = if let Some(prefix_punct) = prefix_punct {
            quote::quote! { #prefix_punct }
        } else {
            quote::quote! {}
        };
        let open_inner_tokenstream = 
        match parser.parser.peek_path_str(ctx.clone(), identifier, ident_token, it.clone(), false, ct.clone()) {
            Ok(path) => {
                it.next();
                quote::quote! { #prefix_stream #path }
            },
            Err(RustHtmlError(_e)) => {
                // couldn't peek path string, try parsing identity expression for dynamic path
                unimplemented!("TODO: implement dynamic path parsing for mdfile_nocache directive")
                // match parser.get_old_parser().parse_identifier_expression(false, identifier, ident_token, false, output, it.clone(), false, ct) {
                //     Ok(()) => { // ident_output
                //         let ident_output = output;
                //         if ident_output.len() > 0 {
                //             // might need to prepend ident_token?
                //             let mut ident_output_final = vec![];
                //             if let Some(ref x) = prefix_token {
                //                 ident_output_final.push(x.clone());
                //             }
                //             ident_output_final.extend_from_slice(&ident_output);
                //             TokenStream::from_iter(ident_output_final.into_iter())
                //         } else {
                //             return Err(RustHtmlError::from_string(format!("cannot read external markdown file nocache '{}', could not parse path: {}", identifier, e)));
                //         }
                //     },
                //     Err(RustHtmlError(e2)) => {
                //         return Err(RustHtmlError::from_string(format!("cannot read external markdown file nocache '{}', could not parse path: {}", identifier, e2)));
                //     }
                // }
            },
        };

        // let path = format!("{}", open_inner_tokenstream);
        let code = quote::quote! {
            view_context.get_markdown_file_nocache(#open_inner_tokenstream)
        };
        let code_token = TokenTree::Group(proc_macro2::Group::new(proc_macro2::Delimiter::Brace, code));
        match parser.parser.convert_copy(code_token, output, ct) {
            Ok(_) => {},
            Err(e) => {
                return Err(RustHtmlError::from_string(format!("cannot read external markdown file nocache '{}': {}", identifier, e)));
            }
        }
        Ok(())
    } */
}

impl IRustHtmlDirective for MarkdownFileNoCacheDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "mdfile_nocache" || name == "md_file_nocache" || name == "markdownfile_nocache" || name == "markdown_file_nocache"
    }
    
    fn execute_new_v3(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        let ident_tokens = parser.get_rust_parser().parse_var(it, ct);
        context.insert_params("mdfile_nocache".to_string(), ident_tokens.unwrap()[0].to_string());
        Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, None))
    }
}