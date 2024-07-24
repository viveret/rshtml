use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;

use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::parserv3::contexts::ihtml_tag_parse_context::IHtmlTagParseContext;



// The IHtmlTagParsed trait is used to define a custom tag parser.
// The tag parser is used to parse a custom tag and generate Rust code.
// This is different from the node parser, which is used to parse a complete HTML node.
pub trait IHtmlTagParsed {
    fn matches(&self, tag_name: &str, is_opening_tag: bool) -> bool;
    fn on_tag_parsed(&self, tag_context: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<Rc<dyn IPeekableRustHtmlToken>, RustHtmlError>;
}