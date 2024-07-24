use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use core_lib::impl_with_logging;
use proc_macro2::Literal;
use proc_macro2::Punct;
use proc_macro2::Ident;

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::contexts::ihtml_tag_parse_context::IHtmlTagParseContext;

use super::rusthtml_token::RustHtmlIdentOrPunct;
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
    fn clear_attr_kvp(&self);
    fn tag_name_as_str(&self) -> String;
    fn fmt_tag_name_as_str(&self, tag_name: &Vec<RustHtmlIdentOrPunct>) -> String;
    fn on_html_tag_name_parsed(&self) -> Result<(), RustHtmlError>;
    fn is_kvp_defined(&self) -> bool;
    fn is_key_defined(&self) -> bool;
    fn is_opening_tag(&self) -> bool;
    fn is_self_contained_tag(&self) -> bool;
    fn is_parsing_attrs(&self) -> bool;
    fn set_equals_punct(&self, punct: &Punct);
    fn get_equals_punct(&self) -> Option<Punct>;
    fn has_tag_name(&self) -> bool;
    fn get_tag_name(&self) -> Vec<RustHtmlIdentOrPunct>;
    fn tag_name_push_ident(&self, ident: &Ident);
    fn tag_name_push_punct(&self, punct: &Punct);
    fn has_html_attr_key(&self) -> bool;
    fn get_html_attr_key(&self) -> String;
    fn get_html_attr_key_literal(&self) -> Option<Literal>;
    fn get_html_attr_key_ident(&self) -> Vec<RustHtmlIdentOrPunct>;
    fn html_attr_key_push_str(&self, s: &str);
    fn html_attr_key_ident_push(&self, ident: &Ident);
    fn html_attr_key_ident_push_punct(&self, punct: &Punct);
    fn html_attr_val_ident_push(&self, ident: &Ident);
    fn html_attr_val_ident_push_punct(&self, punct: &Punct);
    fn set_html_attr_key_literal(&self, literal: &Literal);
    fn has_html_attr_key_ident(&self) -> bool;
    fn set_html_attr_val_literal(&self, literal: &Literal);
    fn has_html_attr_val(&self) -> bool;
    fn has_html_attr_val_ident(&self) -> bool;

    fn set_is_self_contained_tag(&self, is_self_contained_tag: bool);

    fn set_is_opening_tag(&self, is_opening_tag: bool);

    fn is_parsing_attr_val(&self) -> bool;

    fn get_html_attr_val_ident(&self) -> Vec<RustHtmlIdentOrPunct>;

    fn set_html_attr_val_rust(&self, rust: Vec<RustHtmlToken>);

    fn get_html_attr_val_rust(&self) -> Vec<RustHtmlToken>;

    fn get_html_attr_val_literal(&self) -> Option<Literal>;

    fn html_attrs_insert(&self, key: String, val: Option<RustHtmlToken>);

    fn html_attrs_get(&self, key: &str) -> Option<Option<RustHtmlToken>>;

    fn set_parse_attr_val(&self, parse_attr_val: bool);

    fn get_html_attr(&self, key: &str) -> Option<RustHtmlToken>;

    fn get_html_attrs(&self) -> HashMap<String, Option<RustHtmlToken>>;

    fn has_html_attr_val_rust(&self) -> bool;

    fn set_parse_attrs(&self, parse_attrs: bool);

    fn add_tag_end_punct(&self, punct: &Punct);
    fn get_tag_end_punct(&self) -> Option<Punct>;

    fn on_kvp_defined(&self) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn create_key_for_kvp(&self) -> Result<(RustHtmlToken, String), RustHtmlError>;
    fn create_val_for_kvp(&self, attr_name: String) -> Result<Option<(RustHtmlToken, String)>, RustHtmlError>;
);