use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Ident;
use proc_macro2::TokenTree;

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::parserv3::core::rusthtml_directive_result::RustHtmlDirectiveResultV3;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};

use super::irusthtml_directive::IRustHtmlDirective;


// The "htmlfile" directive is used to render html from a file.
pub struct HtmlFileDirective {}

impl HtmlFileDirective {
    pub fn new() -> Self {
        Self {}
    }
/*
    // convert an external HTML directive to RustHtml tokens.
    // identifier: the identifier to convert.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    pub fn convert_externalhtml_directive(context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, identifier_token: &TokenTree, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        match parser.get_old_parser().next_path_str(context.clone(), identifier, identifier_token, it.clone(), false, ct.clone()) {
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
                match parser.get_converter().convert_group(&g, false, context.clone(), ct) {
                    Ok(converted) => {
                        output.push(RustHtmlToken::AppendToHtml(vec![converted]));
                
                        Ok(())
                    },
                    Err(e) => {
                        return Err(RustHtmlError::from_string(format!("cannot read external HTML file '{}', could not convert to RustHtml tokens: {:?}", path, e)));
                    }
                }
            },
            Err(RustHtmlError(e)) => {
                return Err(RustHtmlError::from_string(format!("(@{}) cannot read external HTML file, could not parse path: {}", identifier, e)));
            }
        }
    } */
}

impl IRustHtmlDirective for HtmlFileDirective {
    fn matches(&self, name: &String) -> bool {
        name == "htmlfile" || name == "html_file"
    }

    fn execute_new_v3(&self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        // get literal
        match parser.get_rust_parser().parse_string_with_quotes(false, identifier, it) {
            Ok(path) => {
                
                Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, None)) // ("execute_new_v3 htmlfile directive")
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}