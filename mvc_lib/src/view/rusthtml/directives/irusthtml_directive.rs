use std::rc::Rc;

use proc_macro::Ident;

use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;



// this trait is used to define a directive that can be used in a RustHtml view.
// the directive is defined by a keyword and a function that is executed when the keyword is encountered in the view.
pub trait IRustHtmlDirective {
    fn matches(self: &Self, name: &String) -> bool;
    fn execute(self: &Self, identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError>;
}