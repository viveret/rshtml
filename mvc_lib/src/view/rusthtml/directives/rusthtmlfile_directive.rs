use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Ident;
use proc_macro2::TokenTree;

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::rusthtml::parser_parts::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;

use super::irusthtml_directive::IRustHtmlDirective;


// The "rshtmlfile" directive is used to include a RustHtml file.
pub struct RustHtmlFileDirective {}

impl RustHtmlFileDirective {
    pub fn new() -> Self {
        Self {}
    }

    // convert an external Rust HTML directive to RustHtml tokens.
    // identifier: the identifier to convert.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    pub fn convert_externalrusthtml_directive(ctx: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError<'static>> {
        if let Ok(path) = parser.get_rust_or_html_parser().next_path_str(ctx.clone(), identifier, ident_token, it.clone()) {
            let code = quote::quote!{
                let v = view_context.get_view(#path);
                v.render()
            };
            // let g = proc_macro2::Group::new(proc_macro2::Delimiter::Brace, code);
            // let t = TokenTree::Group(g);
            // let converted = parser.get_converter().convert_stream(code, ct)?;
            if let Ok(converted) = parser.get_converter().convert_stream(code, ctx, ct.clone()) {
                output.push(RustHtmlToken::AppendToHtml(converted));
                Ok(())
            } else {
                Err(RustHtmlError::from_string(format!("cannot read external Rust HTML file '{}', could not convert to Rust", identifier)))
            }
        } else {
            Err(RustHtmlError::from_string(format!("cannot read external Rust HTML file '{}', could not parse path", identifier)))
        }
    }
}

impl IRustHtmlDirective for RustHtmlFileDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "rshtmlfile" || name == "rusthtmlfile"
    }

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // do match instead of if let to access error
        match Self::convert_externalrusthtml_directive(context, identifier, ident_token, parser, output, it, ct) {
            Ok(_) => Ok(RustHtmlDirectiveResult::OkContinue),
            Err(e) => Err(e)
        }
    }
}