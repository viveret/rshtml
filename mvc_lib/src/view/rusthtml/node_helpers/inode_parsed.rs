use std::rc::Rc;

use crate::view::rusthtml::ihtml_tag_parse_context::IHtmlTagParseContext;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;



// this trait is used to allow the HtmlTagParseContext to call back to the HtmlNodeParsed
pub trait IHtmlNodeParsed {
    fn matches(&self, tag_name: &str) -> bool;
    fn on_node_parsed(&self, tag_context: Rc<dyn IHtmlTagParseContext>, html_context: Rc<dyn IRustHtmlParserContext>) -> Result<bool, RustHtmlError>;
}