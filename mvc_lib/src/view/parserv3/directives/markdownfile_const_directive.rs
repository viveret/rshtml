use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Ident;
use proc_macro2::TokenTree;

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::vec_peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::parserv3::core::rusthtml_directive_result::RustHtmlDirectiveResultV3;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};

use super::irusthtml_directive::IRustHtmlDirective;
use super::markdownfile_nocache_directive::MarkdownFileNoCacheDirective;


// The "mdfile" directive is used to render markdown from a file.
pub struct MarkdownFileConstDirective {}

impl MarkdownFileConstDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for MarkdownFileConstDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "mdfile_const" || name == "markdownfile_const"
    }

    fn execute_new_v3(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        let ident_tokens_rshtml = parser.get_rust_parser().parse_expression(it, context, ct.clone())?;
        let ident_tokens_rshtml_stream = Rc::new(VecPeekableRustHtmlToken::new(ident_tokens_rshtml));
        let ident_tokens_peekable_stream = parser.get_converter_out().convert(ident_tokens_rshtml_stream, ct.clone())?;
        let ident_tokens_stream = ident_tokens_peekable_stream.to_stream();

        // let path = parser.get_rust_parser().parse_string_with_quotes(false, identifier, it.clone())?;
        let code = quote::quote! {
            view_context.get_markdown_file_nocache(#ident_tokens_stream)
        };

        let g = proc_macro2::Group::new(proc_macro2::Delimiter::Brace, code);
        let grushtml = parser.get_converter_in().convert_group(g);
        let out_stream = Rc::new(VecPeekableRustHtmlToken::new(vec![RustHtmlToken::AppendToHtml(vec![grushtml])]));
        Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, Some(out_stream)))
    }
}