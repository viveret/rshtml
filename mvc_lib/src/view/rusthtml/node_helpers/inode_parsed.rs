use std::rc::Rc;

use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::html_tag_parse_context::HtmlTagParseContext;
use crate::view::rusthtml::rusthtml_parser_context::IRustHtmlParserContext;




pub trait IHtmlNodeParsed {
    fn matches(&self, tag_name: &str) -> bool;
    fn on_node_parsed(&self, tag_context: &HtmlTagParseContext, html_context: Rc<dyn IRustHtmlParserContext>, output: &mut Vec<RustHtmlToken>) -> Result<bool, RustHtmlError>;
}