// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
use std::collections::HashMap;
use std::rc::Rc;

use proc_macro2::{Literal, Punct};

use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::{RustHtmlIdentAndPunctOrLiteral, RustHtmlIdentOrPunct};
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::irusthtml_parser_context::IRustHtmlParserContext;

// need trait for struct
pub trait IHtmlTagParseContext {
    // parent context
    fn get_main_context(&self) -> Rc<dyn IRustHtmlParserContext>;

    // returns true if the tag is a void tag (e.g. <input /> or <hr />)
    // returns false if the tag is not a void tag (e.g. <div></div> or <p></p>)
    fn is_void_tag(&self) -> bool;

    fn set_is_void_tag(&self, v: bool);

    fn is_explicit_void_tag(&self) -> bool;
    
    fn set_is_explicit_void_tag(&self, v: bool);

    // formats the RustHtml tag name as a string.
    // tag_name: the RustHtml tag name to format as a string.
    // returns the formatted RustHtml tag name as a string.
    fn fmt_tag_name_as_str(&self, tag_name: &Vec<RustHtmlIdentOrPunct>) -> String;

    // called when the tag name is parsed.
    // output: the output RustHtml token stream to add the tag name to.
    fn on_html_tag_name_parsed(&self, tag_name: Vec<RustHtmlIdentOrPunct>) -> Result<RustHtmlToken, RustHtmlError>;

    // whether or not the tag is an opening tag
    fn is_opening_tag(&self) -> bool;

    // whether or not the tag is a self-contained tag
    fn is_self_contained_tag(&self) -> bool;

    // returns true if the tag name is defined.
    fn has_tag_name(&self) -> bool;

    fn get_tag_name(&self) -> Vec<RustHtmlIdentOrPunct>;

    fn tag_name_as_str(&self) -> String;

    fn set_is_self_contained_tag(&self, is_self_contained_tag: bool);

    fn set_is_opening_tag(&self, is_opening_tag: bool);

    fn set_only_add_inner(&self, v: bool);
    fn get_only_add_inner(&self) -> Option<bool>;
    
    fn set_ignore(&self, v: bool);
    fn get_ignore(&self) -> bool;

    fn html_attrs_insert(&self,
        key_tokens: Option<Vec<RustHtmlToken>>,
        key_tokens_special: Option<RustHtmlIdentAndPunctOrLiteral>, 
        key: String,
        equals: Option<RustHtmlToken>,
        equals_token_punct: Option<Punct>,
        value: Option<String>,
        value_literal: Option<Literal>,
        value_tokens: Option<Vec<RustHtmlToken>>,
        value_tokens_special: Option<RustHtmlIdentAndPunctOrLiteral>);

    fn html_attrs_get(&self, key: &str) -> Option<Option<Vec<RustHtmlToken>>>;

    fn get_html_attr(&self, key: &str) -> Option<Vec<RustHtmlToken>>;

    fn get_html_attrs(&self) -> HashMap<String, Option<Vec<RustHtmlToken>>>;

    fn get_html_attrs_output(&self) -> Vec<RustHtmlToken>;

    // fn add_operation_to_ooo_log(&self, operation: String);

    fn add_tag_end_punct(&self, punct: &Punct);
    fn get_tag_end_punct(&self) -> Option<Punct>;

    fn get_add_inner(&self) -> bool;
    fn set_add_inner(&self, val: bool);
}


