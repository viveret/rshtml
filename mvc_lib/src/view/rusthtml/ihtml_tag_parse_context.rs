// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
use std::collections::HashMap;
use std::rc::Rc;

use proc_macro2::Ident;
use proc_macro2::{Literal, Punct};

use crate::view::rusthtml::rusthtml_token::RustHtmlIdentOrPunct;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::irusthtml_parser_context::IRustHtmlParserContext;
use super::rusthtml_error::RustHtmlError;


// need trait for struct
pub trait IHtmlTagParseContext {
    // parent context
    fn get_main_context(&self) -> Rc<dyn IRustHtmlParserContext>;

    // returns true if the tag is a void tag (e.g. <input /> or <hr />)
    // returns false if the tag is not a void tag (e.g. <div></div> or <p></p>)
    fn is_void_tag(&self) -> bool;

    // clears the cached attribute key and value information.
    fn clear_attr_kvp(&self);

    // returns the tag name as a string.
    fn tag_name_as_str(&self) -> String;

    // formats the RustHtml tag name as a string.
    // tag_name: the RustHtml tag name to format as a string.
    // returns the formatted RustHtml tag name as a string.
    fn fmt_tag_name_as_str(&self, tag_name: &Vec<RustHtmlIdentOrPunct>) -> String;

    // called when the tag name is parsed.
    // output: the output RustHtml token stream to add the tag name to.
    fn on_html_tag_name_parsed(&self, output: &mut Vec<RustHtmlToken>);

    // returns true if the key-value pair is defined.
    fn is_kvp_defined(&self) -> bool;

    // returns true if the key is defined.
    fn is_key_defined(&self) -> bool;

    // whether or not the tag is an opening tag
    fn is_opening_tag(&self) -> bool;

    // whether or not the tag is a self-contained tag
    fn is_self_contained_tag(&self) -> bool;

    // returns true if the parser is parsing attributes
    fn is_parsing_attrs(&self) -> bool;

    // assigns the equals punct for the current attribute key-value pair.
    fn set_equals_punct(&self, punct: &Punct);

    fn get_equals_punct(&self) -> Option<Punct>;

    // returns true if the tag name is defined.
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

    fn add_operation_to_ooo_log(&self, operation: String);

    fn set_parse_attrs(&self, parse_attrs: bool);

    fn add_tag_end_punct(&self, punct: &Punct);
    fn get_tag_end_punct(&self) -> Option<Punct>;

    fn on_kvp_defined(&self) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn create_key_for_kvp(&self) -> Result<(RustHtmlToken, String), RustHtmlError>;
    fn create_val_for_kvp(&self, attr_name: String) -> Result<Option<(RustHtmlToken, String)>, RustHtmlError>;
}


