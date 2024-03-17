use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;

use crate::view::rusthtml::parser_parts::rusthmtl_expand_loop_result::RustHtmlExpandLoopResult;
use crate::view::rusthtml::ihtml_tag_parse_context::IHtmlTagParseContext;

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

    fn on_tag_parsed(&self, tag_context: Rc<dyn IHtmlTagParseContext>, _ct: Rc<dyn ICancellationToken>) -> RustHtmlExpandLoopResult {
        if tag_context.is_opening_tag() {
            // let environment_name = tag_context.html_attrs.get("name").unwrap();
            // let environment_value = tag_context.html_attrs.get("value").unwrap();
            // output.push_str(&format!("let {} = {};", environment_name.unwrap(), environment_value.unwrap()));
        }
        Ok((vec![], true))
    }
}