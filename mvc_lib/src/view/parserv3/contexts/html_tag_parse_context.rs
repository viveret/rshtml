use std::cell::RefCell;
// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
use std::collections::HashMap;
use std::rc::Rc;

use proc_macro2::Literal;
use proc_macro2::Punct;

use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlIdentAndPunctOrLiteral;
use crate::view::rusthtml::rusthtml_token::RustHtmlIdentOrPunct;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::ihtml_tag_parse_context::IHtmlTagParseContext;
use super::irusthtml_parser_context::IRustHtmlParserContext;


pub struct HtmlAttrRustTokenGroup {
    pub key_tokens: Option<Vec<RustHtmlToken>>,
    pub key_tokens_special: Option<RustHtmlIdentAndPunctOrLiteral>,
    pub key: String,
    pub equals_token: Option<RustHtmlToken>,
    pub equals_token_punct: Option<Punct>,
    pub value: Option<String>,
    pub value_literal: Option<Literal>,
    pub value_tokens: Option<Vec<RustHtmlToken>>,
    pub value_tokens_special: Option<RustHtmlIdentAndPunctOrLiteral>,
}

impl HtmlAttrRustTokenGroup {
    pub fn to_output(&self) -> Vec<RustHtmlToken> {
        if self.equals_token.is_some() {
            vec![
                RustHtmlToken::HtmlTagAttributeName(self.key.clone(), self.key_tokens_special.clone()),
                RustHtmlToken::HtmlTagAttributeEquals('=', self.equals_token_punct.clone()),
                RustHtmlToken::HtmlTagAttributeValue(self.value.clone(), self.value_literal.clone(), self.value_tokens_special.clone(), self.value_tokens.clone()),
            ]
        } else {
            vec![
                RustHtmlToken::HtmlTagAttributeName(self.key.clone(), self.key_tokens_special.clone()),
            ]
        }
    }
}


// this is the main parsing context for the RustHtml language.
// it is used to parse the RustHtml language into a RustHtmlToken stream of RustHtml tokens.
pub struct HtmlTagParseContext {
    // main context of the parser
    pub main_context: Option<Rc<dyn IRustHtmlParserContext>>,
    // the HTML tag name
    pub tag_name: RefCell<Vec<RustHtmlIdentOrPunct>>,
    // the HTML tag attributes
    pub html_attrs: RefCell<HashMap<String, HtmlAttrRustTokenGroup>>,
    pub is_self_contained_tag: RefCell<bool>,
    pub is_opening_tag: RefCell<bool>,
    pub is_explicit_void_tag: RefCell<bool>,
    // end tag punct
    pub tag_end_punct: RefCell<Vec<Punct>>,
    pub add_inner: RefCell<bool>,
    pub only_add_inner: RefCell<Option<bool>>,
    pub ignore: RefCell<bool>,
}
impl HtmlTagParseContext {
    pub fn new(main_ctx: Option<Rc<dyn IRustHtmlParserContext>>) -> Self {
        Self {
            main_context: main_ctx,
            tag_name: RefCell::new(vec![]),
            html_attrs: RefCell::new(HashMap::new()),
            is_self_contained_tag: RefCell::new(false),
            is_opening_tag: RefCell::new(true),
            is_explicit_void_tag: RefCell::new(false),
            tag_end_punct: RefCell::new(vec![]),
            add_inner: RefCell::new(true),
            only_add_inner: RefCell::new(None),
            ignore: RefCell::new(false),
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
    fn get_main_context(&self) -> Rc<dyn IRustHtmlParserContext> {
        self.main_context.as_ref().expect("called get_main_context, expected context but none was supplied").clone()
    }

    // returns true if the tag is a void tag (e.g. <input /> or <hr />)
    // returns false if the tag is not a void tag (e.g. <div></div> or <p></p>)
    fn is_void_tag(&self) -> bool {
        match self.tag_name_as_str().as_str() {
            "input" | "hr" | "br" | "!DOCTYPE" | "partial" => true,
            _ => *self.is_explicit_void_tag.borrow(),
        }
    }

    // returns the tag name as a string.
    fn tag_name_as_str(&self) -> String {
        return self.fmt_tag_name_as_str(self.tag_name.borrow().as_ref());
    }

    // formats the RustHtml tag name as a string.
    // tag_name: the RustHtml tag name to format as a string.
    // returns the formatted RustHtml tag name as a string.
    fn fmt_tag_name_as_str(&self, tag_name: &Vec<RustHtmlIdentOrPunct>) -> String {
        RustHtmlIdentOrPunct::to_string_join(tag_name)
    }

    // called when the tag name is parsed.
    // output: the output RustHtml token stream to add the tag name to.
    fn on_html_tag_name_parsed(&self, tag_name: Vec<RustHtmlIdentOrPunct>) -> Result<RustHtmlToken, RustHtmlError> {
        if tag_name.is_empty() {
            panic!("tag_name.is_empty() = true")
        }
        self.tag_name.replace(tag_name.clone());
        Ok(if self.is_opening_tag() {
            if self.is_void_tag() {
                RustHtmlToken::HtmlTagVoid(self.tag_name_as_str(), Some(tag_name))
            } else if self.is_self_contained_tag() {
                RustHtmlToken::HtmlTagStart(self.tag_name_as_str(), Some(tag_name))
            } else {
                RustHtmlToken::HtmlTagStart(self.tag_name_as_str(), Some(tag_name))
            }
        } else {
            RustHtmlToken::HtmlTagEnd(self.tag_name_as_str(), Some(tag_name))
        })
    }

    fn is_opening_tag(&self) -> bool {
        *self.is_opening_tag.borrow()
    }

    fn is_self_contained_tag(&self) -> bool {
        *self.is_self_contained_tag.borrow()
    }

    fn has_tag_name(&self) -> bool {
        self.tag_name.borrow().len() > 0
    }

    fn get_tag_name(&self) -> Vec<RustHtmlIdentOrPunct> {
        self.tag_name.borrow().clone()
    }

    fn set_is_self_contained_tag(&self, is_self_contained_tag: bool) {
        *self.is_self_contained_tag.borrow_mut() = is_self_contained_tag;
    }

    fn set_is_opening_tag(&self, is_opening_tag: bool) {
        *self.is_opening_tag.borrow_mut() = is_opening_tag;
    }

    fn html_attrs_insert(&self, 
        key_tokens: Option<Vec<RustHtmlToken>>, 
        key_tokens_special: Option<RustHtmlIdentAndPunctOrLiteral>, 
        key: String,
        equals_token: Option<RustHtmlToken>,
        equals_token_punct: Option<Punct>,
        value: Option<String>,
        value_literal: Option<Literal>,
        value_tokens: Option<Vec<RustHtmlToken>>,
        value_tokens_special: Option<RustHtmlIdentAndPunctOrLiteral>) {
        self.html_attrs.borrow_mut().insert(key.clone(), HtmlAttrRustTokenGroup { 
            key_tokens, key_tokens_special, key, equals_token, equals_token_punct,
            value_tokens, value, value_literal, value_tokens_special
        });
    }

    fn html_attrs_get(&self, key: &str) -> Option<Option<Vec<RustHtmlToken>>> {
        self.html_attrs.borrow().get(key)
            .map(|x| &x.value_tokens)
            .cloned()
    }

    fn get_html_attr(&self, key: &str) -> Option<Vec<RustHtmlToken>> {
        match self.html_attrs.borrow().get(key) {
            Some(val) => val.value_tokens.clone(),
            None => None,
        }
    }

    fn get_html_attrs(&self) -> HashMap<String, Option<Vec<RustHtmlToken>>> {
        let mut hashmap: HashMap<String, Option<Vec<RustHtmlToken>>> = HashMap::new();

        for (key, values) in self.html_attrs.borrow().iter() {
            hashmap.insert(key.clone(), values.value_tokens.clone());
        }

        hashmap
    }

    fn get_html_attrs_output(&self) -> Vec<RustHtmlToken> {
        self.html_attrs.borrow().iter().flat_map(|x| x.1.to_output()).collect()
    }

    // fn on_kvp_defined(&self) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
    //     let (name_token, attr_name) = self.create_key_for_kvp()?;
    //     let val = self.create_val_for_kvp(attr_name.clone())?;

    //     let r = if let Some(val) = val {
    //         self.html_attrs_insert(attr_name, Some(val.0.clone()));
    //         Ok(vec![name_token, RustHtmlToken::HtmlTagAttributeEquals('=', self.get_equals_punct()), val.0])
    //     } else {
    //         self.html_attrs_insert(attr_name, None);
    //         Ok(vec![name_token])
    //     };

    //     self.clear_attr_kvp();
    //     r
    // }

    // fn create_key_for_kvp(&self) -> Result<(RustHtmlToken, String), RustHtmlError> {
    //     let mut attr_name = String::new();
    //     let token = if let Some(is_literal) = &self.get_html_attr_key_literal() {
    //         let s = snailquote::unescape(&is_literal.to_string()).expect("on_kvp_defined: failed to unescape literal");
    //         attr_name.push_str(&s);
    //         RustHtmlToken::HtmlTagAttributeName(is_literal.to_string(), Some(RustHtmlIdentAndPunctOrLiteral::Literal(is_literal.clone())))
    //     } else if self.has_html_attr_key_ident() {
    //         for ident_or_punct in &self.get_html_attr_key_ident() {
    //             match ident_or_punct {
    //                 RustHtmlIdentOrPunct::Ident(ident) => {
    //                     attr_name.push_str(&ident.to_string());
    //                 },
    //                 RustHtmlIdentOrPunct::Punct(punct) => {
    //                     attr_name.push(punct.as_char());
    //                 },
    //             }
    //         }
    //         RustHtmlToken::HtmlTagAttributeName(attr_name.clone(), Some(RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(self.get_html_attr_key_ident())))
    //     } else if self.has_html_attr_key() {
    //         attr_name.push_str(self.get_html_attr_key().as_str());
    //         RustHtmlToken::HtmlTagAttributeName(attr_name.clone(), None)
    //     } else {
    //         return Err(RustHtmlError::from_string(format!("on_kvp_defined: html_attr_key_literal and html_attr_key_ident are both None")));
    //     };
    //     Ok((token, attr_name))
    // }

    // fn create_val_for_kvp(&self, _attr_name: String) -> Result<Option<(RustHtmlToken, String)>, RustHtmlError> {
    //     if let Some(is_literal) = &self.get_html_attr_val_literal() {
    //         // let s = snailquote::unescape(&is_literal.to_string()).unwrap();
    //         let s = is_literal.to_string();
    //         Ok(Some((
    //             RustHtmlToken::HtmlTagAttributeValue(Some(s.clone()), Some(is_literal.clone()), None, None),
    //         s)))
    //     } else if self.has_html_attr_val_ident() {
    //         let ident = self.get_html_attr_val_ident();
    //         let s = ident.iter().map(|i| i.to_string()).collect::<Vec<String>>().join("");
    //         let html_attr_val = RustHtmlToken::HtmlTagAttributeValue(None, None, Some(ident), None);
    //         Ok(Some((html_attr_val, s)))
    //     } else if self.has_html_attr_val_rust() {
    //         let val_rust = self.get_html_attr_val_rust();
    //         let s = val_rust.iter().map(|i| i.to_string()).collect::<Vec<String>>().join("");
    //         let html_attr_val = RustHtmlToken::HtmlTagAttributeValue(None, None, None, Some(val_rust));
    //         Ok(Some((html_attr_val, s)))
    //     } else {
    //         Ok(None)
    //     }
    // }

    fn add_tag_end_punct(&self, punct: &Punct) {
        self.tag_end_punct.borrow_mut().push(punct.clone());
    }

    fn get_tag_end_punct(&self) -> Option<Punct> {
        self.tag_end_punct.borrow().last().cloned()
    }
    
    fn get_add_inner(&self) -> bool {
        *self.add_inner.borrow()
    }
    
    fn set_add_inner(&self, val: bool) {
        self.add_inner.replace(val);
    }
    
    fn set_is_void_tag(&self, v: bool) {
        self.is_explicit_void_tag.replace(v);
    }
    
    fn is_explicit_void_tag(&self) -> bool {
        *self.is_explicit_void_tag.borrow()
    }
    
    fn set_is_explicit_void_tag(&self, v: bool) {
        self.is_explicit_void_tag.replace(v);
    }
    
    fn set_only_add_inner(&self, v: bool) {
        self.only_add_inner.replace(Some(v));
    }
    
    fn get_only_add_inner(&self) -> Option<bool> {
        *self.only_add_inner.borrow()
    }
    
    fn set_ignore(&self, v: bool) {
        self.ignore.replace(v);
    }
    
    fn get_ignore(&self) -> bool {
        *self.ignore.borrow()
    }
}