use std::collections::HashMap;
use std::rc::Rc;

use core_macro_lib::nameof_member_fn;
use proc_macro2::Ident;
use proc_macro2::{Literal, Punct};

use crate::view::rusthtml::rusthtml_token::RustHtmlIdentOrPunct;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::ihtml_tag_parse_context::IHtmlTagParseContext;
use super::irusthtml_parser_context::IRustHtmlParserContext;
use super::rusthtml_error::RustHtmlError;


pub struct HtmlTagParseContextLog {
    real_context: Rc<dyn IHtmlTagParseContext>,
}
impl HtmlTagParseContextLog {
    pub fn new(real_ctx: Rc<dyn IHtmlTagParseContext>) -> Self {
        Self {
            real_context: real_ctx,
        }
    }
}

impl IHtmlTagParseContext for HtmlTagParseContextLog {
    fn get_main_context(&self) -> Rc<dyn IRustHtmlParserContext> {
        self.real_context.get_main_context()
    }

    fn is_void_tag(&self) -> bool {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::is_void_tag).to_string());
        self.real_context.is_void_tag()
    }

    fn clear_attr_kvp(&self) {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::clear_attr_kvp).to_string());
        self.real_context.clear_attr_kvp()
    }

    fn tag_name_as_str(&self) -> String {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::tag_name_as_str).to_string());
        self.real_context.tag_name_as_str()
    }

    fn fmt_tag_name_as_str(&self, tag_name: &Vec<RustHtmlIdentOrPunct>) -> String {
        self.real_context.fmt_tag_name_as_str(tag_name)
    }

    fn on_html_tag_name_parsed(&self) -> Result<(), RustHtmlError> {
        self.real_context.on_html_tag_name_parsed()
    }

    fn is_kvp_defined(&self) -> bool {
        self.real_context.is_kvp_defined()
    }

    fn is_key_defined(&self) -> bool {
        self.real_context.is_key_defined()
    }

    fn is_opening_tag(&self) -> bool {
        self.real_context.is_opening_tag()
    }

    fn is_self_contained_tag(&self) -> bool {
        self.real_context.is_self_contained_tag()
    }

    fn is_parsing_attrs(&self) -> bool {
        self.real_context.is_parsing_attrs()
    }

    fn set_equals_punct(&self, punct: &Punct) {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::set_equals_punct).to_string());
        self.real_context.set_equals_punct(punct)
    }

    fn get_equals_punct(&self) -> Option<Punct> {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::get_equals_punct).to_string());
        self.real_context.get_equals_punct()
    }

    fn has_tag_name(&self) -> bool {
        self.real_context.has_tag_name()
    }

    fn get_tag_name(&self) -> Vec<RustHtmlIdentOrPunct> {
        self.real_context.get_tag_name()
    }

    fn tag_name_push_ident(&self, ident: &Ident) {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::tag_name_push_ident).to_string());
        self.real_context.tag_name_push_ident(ident)
    }

    fn tag_name_push_punct(&self, punct: &Punct) {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::tag_name_push_punct).to_string());
        self.real_context.tag_name_push_punct(punct)
    }

    fn has_html_attr_key(&self) -> bool {
        self.real_context.has_html_attr_key()
    }

    fn get_html_attr_key(&self) -> String {
        self.real_context.get_html_attr_key()
    }

    fn get_html_attr_key_literal(&self) -> Option<Literal> {
        self.real_context.get_html_attr_key_literal()
    }

    fn get_html_attr_key_ident(&self) -> Vec<RustHtmlIdentOrPunct> {
        self.real_context.get_html_attr_key_ident()
    }

    fn html_attr_key_push_str(&self, s: &str) {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::html_attr_key_push_str).to_string());
        self.real_context.html_attr_key_push_str(s)
    }

    fn html_attr_key_ident_push(&self, ident: &Ident) {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::html_attr_key_ident_push).to_string());
        self.real_context.html_attr_key_ident_push(ident)
    }

    fn html_attr_key_ident_push_punct(&self, punct: &Punct) {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::html_attr_key_ident_push_punct).to_string());
        self.real_context.html_attr_key_ident_push_punct(punct)
    }

    fn html_attr_val_ident_push(&self, ident: &Ident) {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::html_attr_val_ident_push).to_string());
        self.real_context.html_attr_val_ident_push(ident)
    }

    fn html_attr_val_ident_push_punct(&self, punct: &Punct) {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::html_attr_val_ident_push_punct).to_string());
        self.real_context.html_attr_val_ident_push_punct(punct)
    }

    fn set_html_attr_key_literal(&self, literal: &Literal) {
        self.add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::set_html_attr_key_literal).to_string(), literal.to_string()));
        self.real_context.set_html_attr_key_literal(literal)
    }

    fn has_html_attr_key_ident(&self) -> bool {
        self.real_context.has_html_attr_key_ident()
    }

    fn set_html_attr_val_literal(&self, literal: &Literal) {
        self.add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::set_html_attr_val_literal).to_string(), literal.to_string()));
        self.real_context.set_html_attr_val_literal(literal)
    }

    fn has_html_attr_val(&self) -> bool {
        self.real_context.has_html_attr_val()
    }

    fn has_html_attr_val_ident(&self) -> bool {
        self.real_context.has_html_attr_val_ident()
    }

    fn set_is_self_contained_tag(&self, is_self_contained_tag: bool) {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::set_is_self_contained_tag).to_string());
        self.real_context.set_is_self_contained_tag(is_self_contained_tag)
    }

    fn set_is_opening_tag(&self, is_opening_tag: bool) {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::set_is_opening_tag).to_string());
        self.real_context.set_is_opening_tag(is_opening_tag)
    }

    fn is_parsing_attr_val(&self) -> bool {
        self.real_context.is_parsing_attr_val()
    }

    fn get_html_attr_val_ident(&self) -> Vec<RustHtmlIdentOrPunct> {
        self.real_context.get_html_attr_val_ident()
    }

    fn set_html_attr_val_rust(&self, rust: Vec<RustHtmlToken>) {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::set_html_attr_val_rust).to_string());
        self.real_context.set_html_attr_val_rust(rust)
    }

    fn get_html_attr_val_rust(&self) -> Vec<RustHtmlToken> {
        self.real_context.get_html_attr_val_rust()
    }

    fn get_html_attr_val_literal(&self) -> Option<Literal> {
        self.real_context.get_html_attr_val_literal()
    }

    fn html_attrs_insert(&self, key: String, val: Option<RustHtmlToken>) {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::html_attrs_insert).to_string());
        self.real_context.html_attrs_insert(key, val)
    }

    fn html_attrs_get(&self, key: &str) -> Option<Option<RustHtmlToken>> {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::html_attrs_get).to_string());
        self.real_context.html_attrs_get(key)
    }

    fn set_parse_attr_val(&self, parse_attr_val: bool) {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::set_parse_attr_val).to_string());
        self.real_context.set_parse_attr_val(parse_attr_val)
    }

    fn get_html_attr(&self, key: &str) -> Option<RustHtmlToken> {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::get_html_attr).to_string());
        self.real_context.get_html_attr(key)
    }

    fn get_html_attrs(&self) -> HashMap<String, Option<RustHtmlToken>> {
        self.real_context.get_html_attrs()
    }

    fn has_html_attr_val_rust(&self) -> bool {
        self.real_context.has_html_attr_val_rust()
    }

    fn add_operation_to_ooo_log(&self, operation: String) {
        self.real_context.add_operation_to_ooo_log(operation)
    }

    fn set_parse_attrs(&self, parse_attrs: bool) {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::set_parse_attrs).to_string());
        self.real_context.set_parse_attrs(parse_attrs)
    }

    fn on_kvp_defined(&self) -> Result<Vec<RustHtmlToken>, super::rusthtml_error::RustHtmlError> {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::on_kvp_defined).to_string());
        self.real_context.on_kvp_defined()
    }

    fn create_key_for_kvp(&self) -> Result<(RustHtmlToken, String), super::rusthtml_error::RustHtmlError> {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::create_key_for_kvp).to_string());
        self.real_context.create_key_for_kvp()
    }

    fn create_val_for_kvp(&self, attr_name: String) -> Result<Option<(RustHtmlToken, String)>, super::rusthtml_error::RustHtmlError> {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::create_val_for_kvp).to_string());
        self.real_context.create_val_for_kvp(attr_name)
    }

    fn add_tag_end_punct(&self, punct: &Punct) {
        self.add_operation_to_ooo_log(nameof_member_fn!(Self::add_tag_end_punct).to_string());
        self.real_context.add_tag_end_punct(punct)
    }

    fn get_tag_end_punct(&self) -> Option<Punct> {
        self.real_context.get_tag_end_punct()
    }
}