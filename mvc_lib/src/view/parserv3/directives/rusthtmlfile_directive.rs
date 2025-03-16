use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Ident;
use proc_macro2::TokenTree;

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::RustHtmlDirectiveResultV3;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};

use super::irusthtml_directive::IRustHtmlDirective;


// The "rshtmlfile" directive is used to include a RustHtml file.
pub struct RustHtmlFileDirective {}

impl RustHtmlFileDirective {
    pub fn new() -> Self {
        Self {}
    }
/*
    // convert an external Rust HTML directive to RustHtml tokens.
    // identifier: the identifier to convert.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    pub fn convert_externalrusthtml_directive(ctx: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &TokenTree, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        if let Ok(path) = parser.get_old_parser().next_path_str(ctx.clone(), identifier, ident_token, it.clone(), false, ct.clone()) {
            let code = quote::quote!{
                let v = view_context.get_view(#path);
                v.render()
            };
            let g = proc_macro2::Group::new(proc_macro2::Delimiter::Brace, code);
            let gconverted = parser.get_converter().convert_group(&g, true, ctx.clone(), ct.clone()).expect("could not convert group");
            output.push(RustHtmlToken::AppendToHtml(vec![gconverted]));

            Ok(())
        } else {
            Err(RustHtmlError::from_string(format!("cannot read external Rust HTML file '{}', could not parse path", identifier)))
        }
    } */
}

impl IRustHtmlDirective for RustHtmlFileDirective {
    fn matches(&self, name: &String) -> bool {
        name == "rshtmlfile" || name == "rusthtmlfile"
    }
    
    fn execute_new_v3(&self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        todo!("execute_new_v3 rusthtmlfile directive")
    }
}