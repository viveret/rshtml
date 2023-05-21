use std::rc::Rc;

use proc_macro::{Ident, Group};

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;

use super::irusthtml_directive::IRustHtmlDirective;


// The "render_body" directive is used to render a body view in a layout view.
pub struct RenderBodyDirective {}

impl RenderBodyDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for RenderBodyDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "render_body"
    }

    fn execute(self: &Self, _: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        it.next(); // skip ()

        let inner_tokens = quote::quote! { mvc_lib::view::rusthtml::rusthtml_view_macros::RustHtmlViewMacros::render_body(view_context, services)?; };
        let inner_tokens2 = inner_tokens.into();

        output.push(
            RustHtmlToken::Group(proc_macro::Delimiter::None, Group::new(proc_macro::Delimiter::None, inner_tokens2))
        );
        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}