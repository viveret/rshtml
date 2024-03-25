use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::TokenStream;

use crate::view::rusthtml::{irusthtml_parser_context::IRustHtmlParserContext, rusthtml_error::RustHtmlError};


pub trait IRustHtmlParserVersionAgnostic {
    fn abstract_parser_versions_expand(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, input: TokenStream, ct: Rc<dyn ICancellationToken>) -> Result<TokenStream, RustHtmlError>;
}
