use std::cell::RefCell;
// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
use std::collections::HashMap;
use std::rc::Rc;

use proc_macro2::Ident;
use proc_macro2::{Literal, Punct};

use crate::view::rusthtml::rusthtml_token::RustHtmlIdentOrPunct;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::ihtml_tag_parse_context::IHtmlTagParseContext;
use super::irusthtml_parser_context::IRustHtmlParserContext;
use super::rusthtml_error::RustHtmlError;
use super::rusthtml_token::RustHtmlIdentAndPunctOrLiteral;

// this is the main parsing context for the RustHtml language.
// it is used to parse the RustHtml language into a RustHtmlToken stream of RustHtml tokens.
pub struct HtmlTagParseContext {
    // main context of the parser
    main_context: Option<Rc<dyn IRustHtmlParserContext>>,
    // the HTML tag name
    tag_name: RefCell<Vec<RustHtmlIdentOrPunct>>,
    // the HTML tag attributes
    html_attrs: RefCell<HashMap<String, Option<RustHtmlToken>>>,
    // the current HTML attribute key
    html_attr_key: RefCell<String>,
    // the current HTML attribute key literal
    html_attr_key_literal: RefCell<Option<Literal>>,
    // the current HTML attribute key ident
    html_attr_key_ident: RefCell<Vec<RustHtmlIdentOrPunct>>,
    // the current HTML attribute value literal
    html_attr_val_literal: RefCell<Option<Literal>>,
    // the current HTML attribute value ident
    html_attr_val_ident: RefCell<Vec<RustHtmlIdentOrPunct>>,
    // the current HTML attribute value Rust tokens
    html_attr_val_rust: RefCell<Vec<RustHtmlToken>>,
    // whether or not to parse attributes
    parse_attrs: RefCell<bool>,
    // whether or not to parse attribute values
    parse_attr_val: RefCell<bool>,
    is_self_contained_tag: RefCell<bool>,
    is_opening_tag: RefCell<bool>,
    // the equals punct
    equals_punct: RefCell<Option<Punct>>,
    // end tag punct
    tag_end_punct: RefCell<Vec<Punct>>,
}
impl HtmlTagParseContext {
    pub fn new(main_ctx: Option<Rc<dyn IRustHtmlParserContext>>) -> Self {
        Self {
            main_context: main_ctx,
            tag_name: RefCell::new(vec![]),
            html_attrs: RefCell::new(HashMap::new()),
            html_attr_key: RefCell::new(String::new()),
            html_attr_key_literal: RefCell::new(None),
            html_attr_key_ident: RefCell::new(vec![]),
            html_attr_val_literal: RefCell::new(None),
            html_attr_val_ident: RefCell::new(vec![]),
            html_attr_val_rust: RefCell::new(vec![]),
            parse_attrs: RefCell::new(false),
            parse_attr_val: RefCell::new(false),
            is_self_contained_tag: RefCell::new(false),
            is_opening_tag: RefCell::new(true),
            equals_punct: RefCell::new(None),
            tag_end_punct: RefCell::new(vec![]),
        }
    }

    pub fn new_and_attach(parent_ctx: Rc<dyn IRustHtmlParserContext>) -> Rc<dyn IHtmlTagParseContext> {
        let ctx = Rc::new(Self::new(Some(parent_ctx.clone())));
        parent_ctx.push_html_tag_parse_context(ctx.clone());
        ctx
    }

    // pub fn new_and_attach_child(parent_ctx: Rc<dyn IRustHtmlParserContext>) -> Rc<dyn IHtmlTagParseContext> {
    //     let ctx = Rc::new(Self::new(Some(parent_ctx.clone())));
    //     parent_ctx.push_html_tag_parse_context(ctx.clone());
    //     ctx
    // }
}

impl IHtmlTagParseContext for HtmlTagParseContext {
    fn get_main_context(self: &Self) -> Rc<dyn IRustHtmlParserContext> {
        self.main_context.as_ref().expect("called get_main_context, expected context but none was supplied").clone()
    }

    // returns true if the tag is a void tag (e.g. <input /> or <hr />)
    // returns false if the tag is not a void tag (e.g. <div></div> or <p></p>)
    fn is_void_tag(self: &Self) -> bool {
        match self.tag_name_as_str().as_str() {
            "input" | "hr" | "br" | "!DOCTYPE" => true,
            _ => false,
        }
    }

    // clears the cached attribute key and value information.
    fn clear_attr_kvp(self: &Self) {
        *self.parse_attr_val.borrow_mut() = false;
        
        *self.html_attr_val_literal.borrow_mut() = None;
        *self.html_attr_val_ident.borrow_mut() = vec![];
        *self.html_attr_val_rust.borrow_mut() = vec![];

        *self.html_attr_key.borrow_mut() = String::new();
        *self.html_attr_key_literal.borrow_mut() = None;
        *self.html_attr_key_ident.borrow_mut() = vec![];
        
        *self.equals_punct.borrow_mut() = None;
    }

    // returns the tag name as a string.
    fn tag_name_as_str(self: &Self) -> String {
        return self.fmt_tag_name_as_str(self.tag_name.borrow().as_ref());
    }

    // formats the RustHtml tag name as a string.
    // tag_name: the RustHtml tag name to format as a string.
    // returns the formatted RustHtml tag name as a string.
    fn fmt_tag_name_as_str(self: &Self, tag_name: &Vec<RustHtmlIdentOrPunct>) -> String {
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
    fn on_html_tag_name_parsed(self: &Self) -> Result<(), RustHtmlError<'static>> {
        *self.parse_attrs.borrow_mut() = true;
        let context = self.get_main_context();
        if self.is_opening_tag() {
            if self.is_void_tag() {
                context.push_output_token(RustHtmlToken::HtmlTagVoid(self.tag_name_as_str(), Some(self.tag_name.borrow().clone())))
            } else if self.is_self_contained_tag() {
                context.push_output_token(RustHtmlToken::HtmlTagStart(self.tag_name_as_str(), Some(self.tag_name.borrow().clone())))
            } else {
                context.push_output_token(RustHtmlToken::HtmlTagStart(self.tag_name_as_str(), Some(self.tag_name.borrow().clone())))
            }
        } else {
            context.push_output_token(RustHtmlToken::HtmlTagEnd(self.tag_name_as_str(), Some(self.tag_name.borrow().clone())))
        }
    }

    fn is_kvp_defined(&self) -> bool {
        return self.is_key_defined();
    }

    fn is_key_defined(&self) -> bool {
        // call equivalent of is_some on string
        return self.html_attr_key.borrow().len() > 0 ||
                self.html_attr_key_ident.borrow().len() > 0 ||
                self.html_attr_key_literal.borrow().is_some() ||
                self.equals_punct.borrow().is_some();
    }

    fn is_opening_tag(&self) -> bool {
        *self.is_opening_tag.borrow()
    }

    fn is_self_contained_tag(&self) -> bool {
        *self.is_self_contained_tag.borrow()
    }

    fn is_parsing_attrs(&self) -> bool {
        *self.parse_attrs.borrow()
    }

    fn set_equals_punct(&self, punct: &Punct) {
        *self.equals_punct.borrow_mut() = Some(punct.clone());
        *self.parse_attr_val.borrow_mut() = true;
    }

    fn has_tag_name(&self) -> bool {
        self.tag_name.borrow().len() > 0
    }

    fn get_tag_name(&self) -> Vec<RustHtmlIdentOrPunct> {
        self.tag_name.borrow().clone()
    }

    fn has_html_attr_key(&self) -> bool {
        self.html_attr_key.borrow().len() > 0 ||
            self.html_attr_key_ident.borrow().len() > 0 ||
            self.html_attr_key_literal.borrow().is_some()
    }

    fn html_attr_key_ident_push(&self, ident: &Ident) {
        self.html_attr_key_ident.borrow_mut().push(RustHtmlIdentOrPunct::Ident(ident.clone()));
    }

    fn html_attr_key_ident_push_punct(&self, punct: &Punct) {
        self.html_attr_key_ident.borrow_mut().push(RustHtmlIdentOrPunct::Punct(punct.clone()));
    }

    fn set_html_attr_key_literal(&self, literal: &Literal) {
        self.html_attr_key_literal.borrow_mut().replace(literal.clone());
    }

    fn set_html_attr_val_literal(&self, literal: &Literal) {
        self.html_attr_val_literal.borrow_mut().replace(literal.clone());
    }

    fn set_is_self_contained_tag(&self, is_self_contained_tag: bool) {
        *self.is_self_contained_tag.borrow_mut() = is_self_contained_tag;
    }

    fn set_is_opening_tag(&self, is_opening_tag: bool) {
        *self.is_opening_tag.borrow_mut() = is_opening_tag;
    }

    fn get_equals_punct(&self) -> Option<Punct> {
        self.equals_punct.borrow().clone()
    }

    fn html_attr_key_push_str(&self, s: &str) {
        self.html_attr_key.borrow_mut().push_str(s);
    }

    fn html_attr_val_ident_push(&self, ident: &Ident) {
        self.html_attr_val_ident.borrow_mut().push(RustHtmlIdentOrPunct::Ident(ident.clone()));
    }

    fn html_attr_val_ident_push_punct(&self, punct: &Punct) {
        self.html_attr_val_ident.borrow_mut().push(RustHtmlIdentOrPunct::Punct(punct.clone()));
    }

    fn is_parsing_attr_val(&self) -> bool {
        *self.parse_attr_val.borrow()
    }

    fn get_html_attr_val_ident(&self) -> Vec<RustHtmlIdentOrPunct> {
        self.html_attr_val_ident.borrow().clone()
    }

    fn get_html_attr_val_literal(&self) -> Option<Literal> {
        self.html_attr_val_literal.borrow().clone()
    }

    fn html_attrs_insert(&self, key: String, val: Option<RustHtmlToken>) {
        self.html_attrs.borrow_mut().insert(key, val);
    }

    fn html_attrs_get(&self, key: &str) -> Option<Option<RustHtmlToken>> {
        self.html_attrs.borrow().get(key).cloned()
    }

    fn tag_name_push_ident(&self, ident: &Ident) {
        self.tag_name.borrow_mut().push(RustHtmlIdentOrPunct::Ident(ident.clone()));
    }

    fn tag_name_push_punct(&self, punct: &Punct) {
        self.tag_name.borrow_mut().push(RustHtmlIdentOrPunct::Punct(punct.clone()));
    }

    fn set_parse_attr_val(&self, parse_attr_val: bool) {
        *self.parse_attr_val.borrow_mut() = parse_attr_val;
    }

    fn get_html_attr_key(&self) -> String {
        self.html_attr_key.borrow().clone()
    }

    fn get_html_attr_key_literal(&self) -> Option<Literal> {
        self.html_attr_key_literal.borrow().clone()
    }

    fn get_html_attr_key_ident(&self) -> Vec<RustHtmlIdentOrPunct> {
        self.html_attr_key_ident.borrow().clone()
    }

    fn has_html_attr_key_ident(&self) -> bool {
        self.html_attr_key_ident.borrow().len() > 0
    }

    fn has_html_attr_val(&self) -> bool {
        self.html_attr_val_literal.borrow().is_some() ||
            self.html_attr_val_ident.borrow().len() > 0 ||
            self.html_attr_val_rust.borrow().len() > 0
    }

    fn has_html_attr_val_ident(&self) -> bool {
        self.html_attr_val_ident.borrow().len() > 0
    }

    fn set_html_attr_val_rust(&self, rust: Vec<RustHtmlToken>) {
        self.html_attr_val_rust.replace(rust);
    }

    fn get_html_attr(&self, key: &str) -> Option<RustHtmlToken> {
        match self.html_attrs.borrow().get(key) {
            Some(val) => val.clone(),
            None => None,
        }
    }

    fn get_html_attrs(&self) -> HashMap<String, Option<RustHtmlToken>> {
        self.html_attrs.borrow().clone()
    }

    fn has_html_attr_val_rust(&self) -> bool {
        self.html_attr_val_rust.borrow().len() > 0
    }

    fn get_html_attr_val_rust(&self) -> Vec<RustHtmlToken> {
        self.html_attr_val_rust.borrow().clone()
    }

    fn add_operation_to_ooo_log(&self, operation: String) {
        self.main_context.as_ref().unwrap().add_operation_to_ooo_log(operation);
    }

    fn set_parse_attrs(&self, parse_attrs: bool) {
        self.parse_attrs.replace(parse_attrs);
    }

    fn on_kvp_defined(&self) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let (name_token, attr_name) = self.create_key_for_kvp()?;
        let val = self.create_val_for_kvp(attr_name.clone())?;

        let r = if let Some(val) = val {
            self.html_attrs_insert(attr_name, Some(val.0.clone()));
            Ok(vec![name_token, RustHtmlToken::HtmlTagAttributeEquals('=', self.get_equals_punct()), val.0])
        } else {
            self.html_attrs_insert(attr_name, None);
            Ok(vec![name_token])
        };

        self.clear_attr_kvp();
        r
    }

    fn create_key_for_kvp(&self) -> Result<(RustHtmlToken, String), RustHtmlError> {
        let mut attr_name = String::new();
        let token = if let Some(is_literal) = &self.get_html_attr_key_literal() {
            let s = snailquote::unescape(&is_literal.to_string()).unwrap();
            attr_name.push_str(&s);
            RustHtmlToken::HtmlTagAttributeName(is_literal.to_string(), Some(RustHtmlIdentAndPunctOrLiteral::Literal(is_literal.clone())))
        } else if self.has_html_attr_key_ident() {
            for ident_or_punct in &self.get_html_attr_key_ident() {
                match ident_or_punct {
                    RustHtmlIdentOrPunct::Ident(ident) => {
                        attr_name.push_str(&ident.to_string());
                    },
                    RustHtmlIdentOrPunct::Punct(punct) => {
                        attr_name.push(punct.as_char());
                    },
                }
            }
            RustHtmlToken::HtmlTagAttributeName(attr_name.clone(), Some(RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(self.get_html_attr_key_ident())))
        } else if self.has_html_attr_key() {
            attr_name.push_str(self.get_html_attr_key().as_str());
            RustHtmlToken::HtmlTagAttributeName(attr_name.clone(), None)
        } else {
            return Err(RustHtmlError::from_string(format!("on_kvp_defined: html_attr_key_literal and html_attr_key_ident are both None")));
        };
        Ok((token, attr_name))
    }

    fn create_val_for_kvp(&self, _attr_name: String) -> Result<Option<(RustHtmlToken, String)>, RustHtmlError> {
        if let Some(is_literal) = &self.get_html_attr_val_literal() {
            // let s = snailquote::unescape(&is_literal.to_string()).unwrap();
            let s = is_literal.to_string();
            Ok(Some((
                RustHtmlToken::HtmlTagAttributeValue(Some(s.clone()), Some(is_literal.clone()), None, None),
            s)))
        } else if self.has_html_attr_val_ident() {
            let ident = self.get_html_attr_val_ident();
            let s = ident.iter().map(|i| i.to_string()).collect::<Vec<String>>().join("");
            let html_attr_val = RustHtmlToken::HtmlTagAttributeValue(None, None, Some(ident), None);
            Ok(Some((html_attr_val, s)))
        } else if self.has_html_attr_val_rust() {
            let val_rust = self.get_html_attr_val_rust();
            let s = val_rust.iter().map(|i| i.to_string()).collect::<Vec<String>>().join("");
            let html_attr_val = RustHtmlToken::HtmlTagAttributeValue(None, None, None, Some(val_rust));
            Ok(Some((html_attr_val, s)))
        } else {
            Ok(None)
        }
    }

    fn add_tag_end_punct(&self, punct: &Punct) {
        self.tag_end_punct.borrow_mut().push(punct.clone());
    }

    fn get_tag_end_punct(&self) -> Option<Punct> {
        self.tag_end_punct.borrow().last().cloned()
    }
}