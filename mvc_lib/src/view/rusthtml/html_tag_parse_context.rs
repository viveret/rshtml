// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
use std::collections::HashMap;

use proc_macro2::{Literal, Punct};

use crate::view::rusthtml::rusthtml_token::RustHtmlIdentOrPunct;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

// this is the main parsing context for the RustHtml language.
// it is used to parse the RustHtml language into a RustHtmlToken stream of RustHtml tokens.
pub struct HtmlTagParseContext {
    // the HTML tag name
    pub tag_name: Vec<RustHtmlIdentOrPunct>,
    // the HTML tag attributes
    pub html_attrs: HashMap<String, Option<RustHtmlToken>>,
    // the current HTML attribute key
    pub html_attr_key: String,
    // the current HTML attribute key literal
    pub html_attr_key_literal: Option<Literal>,
    // the current HTML attribute key ident
    pub html_attr_key_ident: Vec<RustHtmlIdentOrPunct>,
    // the current HTML attribute value literal
    pub html_attr_val_literal: Option<Literal>,
    // the current HTML attribute value ident
    pub html_attr_val_ident: Vec<RustHtmlIdentOrPunct>,
    // the current HTML attribute value Rust tokens
    pub html_attr_val_rust: Vec<RustHtmlToken>,
    // whether or not to parse attributes
    pub parse_attrs: bool,
    // whether or not to parse attribute values
    pub parse_attr_val: bool,
    // whether or not the tag is self-contained
    pub is_self_contained_tag: bool,
    // whether or not the tag is an opening tag
    pub is_opening_tag: bool,
    // the equals punct
    pub equals_punct: Option<Punct>,
}
impl HtmlTagParseContext {
    pub fn new() -> Self {
        Self {
            tag_name: vec![],
            html_attrs: HashMap::new(),
            html_attr_key: String::new(),
            html_attr_key_literal: None,
            html_attr_key_ident: vec![],
            html_attr_val_literal: None,
            html_attr_val_ident: vec![],
            html_attr_val_rust: vec![],
            parse_attrs: false,
            parse_attr_val: false,
            is_self_contained_tag: false,
            is_opening_tag: true,
            equals_punct: None,
        }
    }

    // returns true if the tag is a void tag (e.g. <input /> or <hr />)
    // returns false if the tag is not a void tag (e.g. <div></div> or <p></p>)
    pub fn is_void_tag(self: &Self) -> bool {
        match self.tag_name_as_str().as_str() {
            "input" | "hr" | "br" | "!DOCTYPE" => true,
            _ => false,
        }
    }

    // clears the cached attribute key and value information.
    pub fn clear_attr_kvp(self: &mut Self) {
        self.parse_attr_val = false;

        self.html_attr_val_literal = None;
        self.html_attr_val_ident = vec![];
        self.html_attr_val_rust = vec![];

        self.html_attr_key = String::new();
        self.html_attr_key_literal = None;
        self.html_attr_key_ident = vec![];

        // self.html_attrs.clear();
        self.equals_punct = None;
    }

    // returns the tag name as a string.
    pub fn tag_name_as_str(self: &Self) -> String {
        return Self::fmt_tag_name_as_str(&self.tag_name);
    }

    // formats the RustHtml tag name as a string.
    // tag_name: the RustHtml tag name to format as a string.
    // returns the formatted RustHtml tag name as a string.
    pub fn fmt_tag_name_as_str(tag_name: &Vec<RustHtmlIdentOrPunct>) -> String {
        let mut s = String::new();
        for part in tag_name.iter() {
            match part {
                RustHtmlIdentOrPunct::Ident(ident) => s.push_str(&ident.to_string()),
                RustHtmlIdentOrPunct::Punct(punct) => s.push(punct.as_char()),
            }
        }
        return s;
    }

    // called when the tag name is parsed.
    // output: the output RustHtml token stream to add the tag name to.
    pub fn on_html_tag_name_parsed(self: &mut Self, output: &mut Vec<RustHtmlToken>) {
        self.parse_attrs = true;
        if self.is_opening_tag {
            if self.is_void_tag() {
                output.push(RustHtmlToken::HtmlTagVoid(self.tag_name_as_str(), Some(self.tag_name.clone())));
            } else if self.is_self_contained_tag {
                output.push(RustHtmlToken::HtmlTagStart(self.tag_name_as_str(), Some(self.tag_name.clone())));
            } else {
                output.push(RustHtmlToken::HtmlTagStart(self.tag_name_as_str(), Some(self.tag_name.clone())));
            }
        } else {
            output.push(RustHtmlToken::HtmlTagEnd(self.tag_name_as_str(), Some(self.tag_name.clone())));
        }
    }

    pub fn is_kvp_defined(&self) -> bool {
        return self.is_key_defined();
    }

    pub fn is_key_defined(&self) -> bool {
        // call equivalent of is_some on string
        return self.html_attr_key.len() > 0 ||
                self.html_attr_key_ident.len() > 0 ||
                self.html_attr_key_literal.is_some() ||
                self.equals_punct.is_some();
    }
}