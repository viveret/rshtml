use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Ident;
use proc_macro2::TokenTree;

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::rusthtml::parser_parts::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
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
    pub fn convert_externalhtml_directive(
        context: Rc<dyn IRustHtmlParserContext>,
        identifier: &Ident,
        identifier_token: &RustHtmlToken,
        parser: Rc<dyn IRustHtmlParserAll>,
        output: &mut Vec<RustHtmlToken>,
        it: Rc<dyn IPeekableRustHtmlToken>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<(), RustHtmlError<'static>> {
        match parser.get_rust_or_html_parser().next_path_str(context.clone(), identifier, identifier_token, it.clone()) {
            Ok(path) => {
                let code = quote::quote! {
                    match view_context.open_view_file(#path) {
                        Ok(mut f) => {
                            let mut buffer = String::new();
                            f.read_to_string(&mut buffer).expect("could not read HTML file");
                            buffer
                        },
                        Err(e) => {
                            return Err(RustHtmlError::from_string(format!("cannot read external HTML file '{}', could not open: {:?}", #path, e)));
                        }
                    }
                };
                let g = proc_macro2::Group::new(proc_macro2::Delimiter::Brace, code);
                // let code_converted = parser.get_converter().convert_group(&g, true, ct.clone())?;
                if let Ok(code_converted) = parser.get_converter().convert_group(&g, true, context, ct.clone()) {
                    output.push(RustHtmlToken::AppendToHtml(vec![code_converted]));
                } else {
                    return Err(RustHtmlError::from_string(format!("cannot read external HTML file '{}', could not convert to Rust", identifier)));
                }
                Ok(())
            },
            Err(RustHtmlError(e)) => {
                return Err(RustHtmlError::from_string(format!("(@{}) cannot read external HTML file, could not parse path: {}", identifier, e)));
            }
        }
    }
}

impl IRustHtmlDirective for HtmlFileDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "htmlfile" || name == "html_file"
    }

    fn execute(
        self: &Self,
        context: Rc<dyn IRustHtmlParserContext>,
        identifier: &Ident,
        ident_token: &RustHtmlToken,
        parser: Rc<dyn IRustHtmlParserAll>,
        output: &mut Vec<RustHtmlToken>,
        it: Rc<dyn IPeekableRustHtmlToken>,
        ct: Rc<dyn ICancellationToken>,
    ) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        Self::convert_externalhtml_directive(context, identifier, ident_token, parser, output, it, ct)?;
        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}