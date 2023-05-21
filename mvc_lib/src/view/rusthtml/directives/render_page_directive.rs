use std::{borrow::Cow, rc::Rc};

use proc_macro::{Ident, TokenTree};

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;

use super::irusthtml_directive::IRustHtmlDirective;


// The "render_page" directive is used to render a page view.
pub struct RenderPageDirective {}

impl RenderPageDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for RenderPageDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "render_page"
    }

    fn execute(self: &Self, identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}