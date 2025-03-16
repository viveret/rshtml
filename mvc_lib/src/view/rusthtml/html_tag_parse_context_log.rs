use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use core_lib::impl_with_logging;
use proc_macro2::{Literal, Punct};

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::contexts::ihtml_tag_parse_context::IHtmlTagParseContext;

use super::rusthtml_token::{RustHtmlIdentAndPunctOrLiteral, RustHtmlIdentOrPunct};
use super::rusthtml_error::RustHtmlError;
use super::rusthtml_token::RustHtmlToken;


pub struct HtmlTagParseContextLog {
    real_context: Rc<dyn IHtmlTagParseContext>,
    order_of_operations: RefCell<Vec<String>>,
}
impl HtmlTagParseContextLog {
    pub fn new(real_ctx: Rc<dyn IHtmlTagParseContext>) -> Self {
        Self {
            real_context: real_ctx,
            order_of_operations: RefCell::new(vec![]),
        }
    }
}

impl_with_logging!(IHtmlTagParseContext, HtmlTagParseContextLog, real_context,
    fn get_main_context(&self) -> Rc<dyn IRustHtmlParserContext>;
    fn is_void_tag(&self) -> bool;
    fn set_is_void_tag(&self, v: bool);
    fn is_explicit_void_tag(&self) -> bool;
    fn set_is_explicit_void_tag(&self, v: bool);
    fn tag_name_as_str(&self) -> String;
    fn fmt_tag_name_as_str(&self, tag_name: &Vec<RustHtmlIdentOrPunct>) -> String;
    fn on_html_tag_name_parsed(&self, tag_name: Vec<RustHtmlIdentOrPunct>) -> Result<RustHtmlToken, RustHtmlError>;
    fn is_opening_tag(&self) -> bool;
    fn is_self_contained_tag(&self) -> bool;
    fn has_tag_name(&self) -> bool;
    fn get_tag_name(&self) -> Vec<RustHtmlIdentOrPunct>;

    fn set_is_self_contained_tag(&self, is_self_contained_tag: bool);

    fn set_is_opening_tag(&self, is_opening_tag: bool);

    fn html_attrs_insert(&self, 
        key_tokens: Option<Vec<RustHtmlToken>>, 
        key_tokens_special: Option<RustHtmlIdentAndPunctOrLiteral>, 
        key: String,
        equals_token: Option<RustHtmlToken>,
        equals_token_punct: Option<Punct>,
        value: Option<String>,
        value_literal: Option<Literal>,
        value_tokens: Option<Vec<RustHtmlToken>>,
        value_tokens_special: Option<RustHtmlIdentAndPunctOrLiteral>);

    fn html_attrs_get(&self, key: &str) -> Option<Option<Vec<RustHtmlToken>>>;

    fn get_html_attr(&self, key: &str) -> Option<Vec<RustHtmlToken>>;

    fn get_html_attrs(&self) -> HashMap<String, Option<Vec<RustHtmlToken>>>;
    fn get_html_attrs_output(&self) -> Vec<RustHtmlToken>;

    fn add_tag_end_punct(&self, punct: &Punct);
    fn get_tag_end_punct(&self) -> Option<Punct>;

    fn get_add_inner(&self) -> bool;
    fn set_add_inner(&self, val: bool);

    fn set_only_add_inner(&self, v: bool);
    fn get_only_add_inner(&self) -> Option<bool>;
    
    fn get_ignore(&self) -> bool;
    fn set_ignore(&self, val: bool);
);