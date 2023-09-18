use std::rc::Rc;

use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::ihtml_tag_parse_context::IHtmlTagParseContext;
use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::itag_parsed::IHtmlTagParsed;


// The EnvironmentHtmlTagParsed struct is used to parse the environment tag.
// The environment tag is used to conditionally render a section of the view based on the environment name.
pub struct EnvironmentHtmlTagParsed {

}

impl EnvironmentHtmlTagParsed {
    pub fn new() -> Self {
        Self {}
    }
}

impl IHtmlTagParsed for EnvironmentHtmlTagParsed {
    fn matches(&self, tag_name: &str, is_opening_tag: bool) -> bool {
        tag_name == "environment" && is_opening_tag
    }

    fn on_tag_parsed(&self, tag_context: &dyn IHtmlTagParseContext, _html_context: Rc<dyn IRustHtmlParserContext>, _output: &mut Vec<RustHtmlToken>) -> Result<bool, RustHtmlError> {
        if tag_context.is_opening_tag() {
            // let environment_name = tag_context.html_attrs.get("name").unwrap();
            // let environment_value = tag_context.html_attrs.get("value").unwrap();
            // output.push_str(&format!("let {} = {};", environment_name.unwrap(), environment_value.unwrap()));
        }
        Ok(true)
    }
}