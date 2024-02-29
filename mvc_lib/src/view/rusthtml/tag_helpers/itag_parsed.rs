use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;

use crate::view::rusthtml::ihtml_tag_parse_context::IHtmlTagParseContext;
use crate::view::rusthtml::parser_parts::rusthmtl_expand_loop_result::RustHtmlExpandLoopResult;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;



// The IHtmlTagParsed trait is used to define a custom tag parser.
// The tag parser is used to parse a custom tag and generate Rust code.
// This is different from the node parser, which is used to parse a complete HTML node.
pub trait IHtmlTagParsed {
    fn matches(&self, tag_name: &str, is_opening_tag: bool) -> bool;
    fn on_tag_parsed(&self, tag_context: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> RustHtmlExpandLoopResult;
}