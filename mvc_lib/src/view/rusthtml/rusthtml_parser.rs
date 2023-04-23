// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Peekable;
use std::io::Read;
use std::rc::Rc;
use std::str::FromStr;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::quote;

use crate::view::rusthtml::rusthtml_lang_part::RustHtmlLangPart;
use crate::view::rusthtml::rusthtml_token::{RustHtmlToken, RustHtmlIdentAndPunctOrLiteral, RustHtmlIdentOrPunct, RustHtmlIdentAndPunctAndGroupOrLiteral, RustHtmlIdentOrPunctOrGroup };
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

pub struct HtmlTagParseContext {
    pub tag_name: Vec<RustHtmlIdentOrPunct>,
    pub html_attrs: HashMap<String, Option<RustHtmlToken>>,
    pub html_attr_key: String,
    pub html_attr_key_literal: Option<Literal>,
    pub html_attr_key_ident: Vec<RustHtmlIdentOrPunct>,
    pub html_attr_val: Vec<RustHtmlToken>,
    pub parse_attrs: bool,
    pub parse_attr_val: bool,
    pub is_self_contained_tag: bool,
    pub is_opening_tag: bool,
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
            html_attr_val: vec![],
            parse_attrs: false,
            parse_attr_val: false,
            is_self_contained_tag: false,
            is_opening_tag: true,
            equals_punct: None,
        }
    }

    pub fn is_void_tag(self: &Self) -> bool {
        match self.tag_name_as_str().as_str() {
            "input" | "hr" | "!DOCTYPE" => true,
            _ => false,
        }
    }

    pub fn clear_attr_kvp(self: &mut Self) {
        self.parse_attr_val = false;

        self.html_attr_val = vec![];

        self.html_attr_key = String::new();
        self.html_attr_key_literal = None;
        self.html_attr_key_ident = vec![];

        self.equals_punct = None;
    }

    pub fn tag_name_as_str(self: &Self) -> String {
        return Self::fmt_tag_name_as_str(&self.tag_name);
    }

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

    pub fn on_html_tag_name_parsed(self: &mut Self, output: &mut Vec<RustHtmlToken>) {
        self.parse_attrs = true;
        if self.is_opening_tag {
            if self.is_void_tag() {
                output.push(RustHtmlToken::HtmlTagVoid(self.tag_name_as_str(), self.tag_name.clone()));
            } else if self.is_self_contained_tag {
                output.push(RustHtmlToken::HtmlTagStart(self.tag_name_as_str(), self.tag_name.clone()));
            } else {
                output.push(RustHtmlToken::HtmlTagStart(self.tag_name_as_str(), self.tag_name.clone()));
            }
        } else {
            output.push(RustHtmlToken::HtmlTagEnd(self.tag_name_as_str(), self.tag_name.clone()));
        }
    }
}

pub struct RustHtmlParser {
    pub should_panic_or_return_error: bool,
    pub lang_parts: Vec<Rc<dyn RustHtmlLangPart>>,
    pub lang_part_stack: RefCell<Vec<Rc<dyn RustHtmlLangPart>>>,

    pub punctuation_scope_stack: RefCell<Vec<char>>,
    pub htmltag_scope_stack: RefCell<Vec<String>>,

    pub params: RefCell<HashMap<String, String>>,
    pub functions_section: RefCell<Option<TokenStream>>,
    pub struct_section: RefCell<Option<TokenStream>>,
    pub impl_section: RefCell<Option<TokenStream>>,
    pub model_type: RefCell<Option<Vec<TokenTree>>>,
    pub use_statements: RefCell<Vec<TokenStream>>,

    pub raw: RefCell<String>,

    pub has_included_view_start: RefCell<bool>,

    pub environment_name: String,
}
impl RustHtmlParser {
    pub fn new(should_panic_or_return_error: bool, environment_name: String) -> Self {
        Self {
            should_panic_or_return_error: should_panic_or_return_error,
            lang_parts: vec![
                Rc::new(crate::view::rusthtml::rusthtml_lang_parts::rust_html::RustHtml { }),
                Rc::new(crate::view::rusthtml::rusthtml_lang_parts::directive::Directive { }),
            ],
            lang_part_stack: RefCell::new(vec![]),
            htmltag_scope_stack: RefCell::new(vec![]),
            punctuation_scope_stack: RefCell::new(vec![]),
            params: RefCell::new(HashMap::new()),
            functions_section: RefCell::new(None),
            struct_section: RefCell::new(None),
            impl_section: RefCell::new(None),
            model_type: RefCell::new(None),
            use_statements: RefCell::new(vec![]),
            raw: RefCell::new(String::new()),
            has_included_view_start: RefCell::new(false),
            environment_name: environment_name,
        }
    }

    pub fn get_model_type_name(self: &Self) -> String {
        let mut s = String::new();
        for type_part in self.get_model_type() {
            s.push_str(&type_part.to_string());
        }
        s
    }

    pub fn get_model_type(self: &Self) -> Vec<TokenTree> {
        self.model_type.borrow().clone().unwrap_or(vec![])
    }

    pub fn try_get_param_string(self: &Self, key: &str) -> Option<String> {
        match self.params.borrow().get(&key.to_string()) {
            Some(str_val) => {
                let s = snailquote::unescape(str_val).unwrap();
                Some(s)
            },
            None => {
                None
            }
        }
    }

    pub fn get_param_string(self: &Self, key: &str) -> Result<String, RustHtmlError> {
        match self.params.borrow().get(&key.to_string()) {
            Some(str_val) => {
                let s = snailquote::unescape(str_val).unwrap();
                Ok(s)
            },
            None => {
                return self.panic_or_return_error(format!("missing param '@{}' in rusthtml", key));
            }
        }
    }

    pub fn get_functions_section(self: &Self) -> Option<TokenStream> {
        if let Some(has_functions) = self.functions_section.borrow().as_ref() {
            Some(has_functions.clone())
        } else {
            None
        }
    }

    pub fn get_struct_section(self: &Self) -> Option<TokenStream> {
        if let Some(has_struct) = self.struct_section.borrow().as_ref() {
            Some(has_struct.clone())
        } else {
            None
        }
    }

    pub fn get_impl_section(self: &Self) -> Option<TokenStream> {
        if let Some(has_impl) = self.impl_section.borrow().as_ref() {
            Some(has_impl.clone())
        } else {
            None
        }
    }

    pub fn get_model_ident(self: &Self) -> Option<TokenStream> {
        if let Some(has_model) = self.model_type.borrow().as_ref() {
            Some(TokenStream::from_iter(has_model.clone()))
        } else {
            None
        }
    }

    pub fn expand_tokenstream(self: &Self, input: TokenStream) -> Result<TokenStream, RustHtmlError> {
        let mut it = input.into_iter().peekable();

        let rusthtml_tokens_for_view = self.parse_tokenstream_to_rusthtmltokens(true, it.by_ref(), false)?;

        // prefix with _view_start
        let view_start_path = self.get_param_string("viewstart").unwrap_or("src/views/home/_view_start.rshtml".to_string());
        let mut view_start_tokens = vec![];
        self.expand_external_tokenstream(&view_start_path, &mut view_start_tokens)?;

        let rusthtml_tokens = view_start_tokens.iter().chain(rusthtml_tokens_for_view.iter()).cloned().collect();
        let rust_output = self.parse_rusthtmltokens_to_plain_rust(rusthtml_tokens)?;

        self.raw.replace(self.display_as_code(&mut rust_output.iter().cloned().peekable()));

        Ok(TokenStream::from_iter(rust_output))
    }

    pub fn print_as_code(self: &Self, rust_output: &Vec<TokenTree>) {
        println!("{}", self.display_as_code(&mut rust_output.iter().cloned().peekable()));
    }

    pub fn display_as_code(self: &Self, rust_output: &mut Peekable<impl Iterator<Item=TokenTree>>) -> String {
        let mut s = String::new();
        for token in rust_output {
            if let TokenTree::Group(group) = token {
                let delimiter = group.delimiter();
                s.push_str(self.get_opening_delim(delimiter));
                s.push_str(&self.display_as_code(&mut group.stream().into_iter().peekable()));
                s.push_str(self.get_closing_delim(delimiter));
            } else {
                s.push_str(&token.to_string());
            }
        }
        s
    }

    pub fn panic_or_return_error<'a, T>(self: &Self, message: String) -> Result<T, RustHtmlError<'a>> {
        if self.should_panic_or_return_error {
            panic!("{}", message);
        } else {
            return Err(RustHtmlError::from_string(message));
        }
    }

    pub fn parse_to_ast(self: &Self, input: TokenStream) -> Result<syn::Item, RustHtmlError> {
        let ts = self.expand_tokenstream(input)?;
        let ast = syn::parse(ts).unwrap();
        Ok(ast)
    }

    pub fn parse_tokenstream_to_rusthtmltokens(self: &Self, is_in_html_mode: bool, it: &mut Peekable<impl Iterator<Item = TokenTree>>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut rusthtml_tokens = Vec::new();
        self.loop_next_and_convert(is_in_html_mode, &mut rusthtml_tokens, it, is_raw_tokenstream)?;
        Ok(rusthtml_tokens)
    }

    pub fn loop_next_and_convert(self: &Self, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError> {
        loop {
            if self.next_and_convert(is_in_html_mode, output, it, is_raw_tokenstream)? {
                break;
            }
        }
        Ok(())
    }

    pub fn next_and_convert(self: &Self, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>, is_raw_tokenstream: bool) -> Result<bool, RustHtmlError> {
        let token_option = it.next();

        if is_raw_tokenstream {
            // println!("raw next_and_convert: {:?}", token_option);
        } else  {
            // println!("next_and_convert: {:?}", token_option);
        }

        if let Some(token) = token_option {
            if self.convert_tokentree_to_rusthtmltoken(token, is_in_html_mode, output, it, is_raw_tokenstream)? {
                return Ok(true); // break outer loop
            }
        } else {
            return Ok(true); // break outer loop
            // return Err(RustHtmlError::from_str("could not read next token"));
        }

        Ok(false)
    }

    pub fn parse_rusthtmltokens_to_plain_rust(self: &Self, rusthtml_tokens: Vec<RustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError> { // , Option<Vec<TokenTree>>)
        let mut rust_output = Vec::new();
        let iter = rusthtml_tokens.iter();
        let mut it = iter.peekable();
        loop 
        {
            if self.convert_rusthtmltokens_to_plain_rust(&mut rust_output, &mut it)? {
                break;
            }
        }
        Ok(rust_output)
    }

    pub fn convert_rusthtmltokens_to_plain_rust<'a>(self: &Self, output: &mut Vec<TokenTree>, it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<bool, RustHtmlError> { // , Option<Vec<TokenTree>>)
        let mut should_break_outer_loop = false;
        loop 
        {
            let token_option = it.next();
            if let Some(token) = token_option {
                let break_loop = self.convert_rusthtmltoken_to_tokentree(&token, output, it)?;
                if break_loop {
                    break;
                }
            } else {
                should_break_outer_loop = true;
                break;
            }
        }

        Ok(should_break_outer_loop)
    }

    pub fn convert_tokentree_to_rusthtmltoken(self: &Self, token: TokenTree, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>, is_raw_tokenstream: bool) -> Result<bool, RustHtmlError> {
        match token.clone() {
            TokenTree::Ident(ident) => {
                if is_in_html_mode {
                    output.push(RustHtmlToken::HtmlTextNode(ident.to_string(), token.span().clone()));
                } else {
                    output.push(RustHtmlToken::Identifier(ident));
                }
            },
            TokenTree::Literal(literal) => {
                if is_in_html_mode {
                    output.push(RustHtmlToken::HtmlTextNode(literal.to_string(), token.span().clone()));
                } else {
                    output.push(RustHtmlToken::Literal(literal));
                }
            },
            TokenTree::Punct(punct) => {
                if self.convert_punct_to_rusthtmltoken(punct, is_in_html_mode, output, it, is_raw_tokenstream)? {
                    return Ok(true);
                }
            },
            TokenTree::Group(group) => {
                self.convert_group_to_rusthtmltoken(group, is_in_html_mode, output, is_raw_tokenstream)?;
            },
        }
        Ok(false) // continue
    }

    pub fn is_start_of_current_expression(self: &Self, output: &mut Vec<RustHtmlToken>) -> bool {
        if output.len() == 0 {
            // println!("is_start_of_current_expression output.len() == 0, returning true");
            true
        } else {
            let last = output.last().unwrap();
            match last {
                RustHtmlToken::ReservedChar(c, _punct) => {
                    match c {
                        ';' => {
                            // println!("is_start_of_current_expression output.last() == ';', returning true");
                            true
                        },
                        _ => {
                            // println!("is_start_of_current_expression output.last() != ';', returning false");
                            false
                        }
                    }
                },
                RustHtmlToken::Group(..) => {
                    // println!("is_start_of_current_expression output.last() == group, returning true");
                    true
                },
                _ => {
                    // println!("is_start_of_current_expression output.last() == {:?}, returning false", last);
                    false
                },
            }
        }
    }
    
    pub fn convert_punct_to_rusthtmltoken(self: &Self, punct: Punct, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>, is_raw_tokenstream: bool) -> Result<bool, RustHtmlError> {
        let c = punct.as_char();
        // println!("c: {}", c);
        match c {
            '@' => {
                if is_in_html_mode {
                    let directive_ident = it.next().unwrap();
                    // println!("directive_ident: {:?}", directive_ident);
                    self.convert_rusthtml_directive_to_rusthtmltoken(directive_ident, None, output, it, is_raw_tokenstream)?;
                } else {
                    return self.panic_or_return_error(format!("Cannot escape HTML when already in rust mode (hint: remove '@'?)"));
                }
            },
            '<' => {
                if is_in_html_mode || self.is_start_of_current_expression(output) {
                    let mut ctx = HtmlTagParseContext::new();
                    let mut output_inner = vec![];
                    loop {
                        let token_option = it.next();
                        if self.next_and_parse_html_tag(token_option, &mut ctx, &mut output_inner, it, is_raw_tokenstream)? {
                            break;
                        }
                    }

                    let mut add_inner = true;
                    // println!("is_self_contained_tag: {}", ctx.is_self_contained_tag);
                    if ctx.is_opening_tag && !ctx.is_void_tag() && !ctx.is_self_contained_tag {
                        // parse inner elements / code until we find closing tag
                        self.htmltag_scope_stack.borrow_mut().push(ctx.tag_name_as_str());
                        loop {
                            if self.next_and_convert(true, &mut output_inner, it, is_raw_tokenstream)? {
                                break;
                            }
                            match output_inner.last().unwrap() {
                                RustHtmlToken::HtmlTagEnd(tag_end, _tag_end_tokens) => {
                                    if tag_end == &ctx.tag_name_as_str() {
                                        // println!("Found end tag, breaking {:?}", tag_end);
                                        break;
                                    }
                                },
                                _ => {
                                    // println!("last token while processing html tag '{}' child elements: {:?}", ctx.tag_name_as_str(), output_inner.last().unwrap());
                                }
                            }
                        }
                        let last_scope_from_stack = self.htmltag_scope_stack.borrow_mut().pop().unwrap();
                        if last_scope_from_stack != ctx.tag_name_as_str() {
                            self.panic_or_return_error(format!("Mismatched HTML tags (found {} but expected {})", last_scope_from_stack, ctx.tag_name_as_str()))?;
                        }

                        match output_inner.last().unwrap() {
                            RustHtmlToken::HtmlTagEnd(_tag_end, _tag_end_tokens) => {
                                add_inner = self.on_html_node_parsed(ctx, &mut output_inner)?;
                            },
                            _ => {}
                        }
                    }

                    if add_inner {
                        output.extend_from_slice(&output_inner);
                    }
                } else {
                    output.push(RustHtmlToken::ReservedChar(c, punct));
                }
            },
            '}' if !is_in_html_mode => {
                return Ok(true); // do not continue
            },
            '>' if !is_in_html_mode => {
                output.push(RustHtmlToken::ReservedChar(c, punct));
                // return self.panic_or_return_error(format!("Unexpected > (did you mean \"&gt;\"?)"));
            },
            _ => {
                if is_in_html_mode {
                    output.push(RustHtmlToken::HtmlTextNode(punct.as_char().to_string(), punct.span().clone()));
                } else {
                    output.push(RustHtmlToken::ReservedChar(c, punct));
                }
            },
        }

        Ok(false)
    }

    pub fn convert_group_to_rusthtmltoken(self: &Self, group: Group, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError> {
        let delimiter = group.delimiter();
        if is_in_html_mode {
            let c_start = self.get_opening_delim(delimiter);
            let c_end = self.get_closing_delim(delimiter);

            output.push(RustHtmlToken::HtmlTextNode(c_start.to_string(), group.span()));
            self.loop_next_and_convert(true, output, group.stream().into_iter().peekable().by_ref(), is_raw_tokenstream)?;
            output.push(RustHtmlToken::HtmlTextNode(c_end.to_string(), group.span()));
        } else {
            if delimiter == Delimiter::Brace {
                let mut inner_tokens = vec![];
                self.loop_next_and_convert(false, &mut inner_tokens, group.stream().into_iter().peekable().by_ref(), is_raw_tokenstream)?;

                output.push(RustHtmlToken::GroupParsed(delimiter, inner_tokens));
            } else {
                output.push(RustHtmlToken::Group(delimiter, group));
            }
        }

        Ok(())
    }

    pub fn get_opening_delim(self: &Self, delimiter: Delimiter) -> &'static str {
        match delimiter {
            Delimiter::Brace => "{",
            Delimiter::Bracket => "[",
            Delimiter::Parenthesis => "(",
            Delimiter::None => "",
        }
    }

    pub fn get_closing_delim(self: &Self, delimiter: Delimiter) -> &'static str {
        match delimiter {
            Delimiter::Brace => "}",
            Delimiter::Bracket => "]",
            Delimiter::Parenthesis => ")",
            Delimiter::None => "",
        }
    }

    pub fn expect_punct(self: &Self, c: char, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<(), Option<TokenTree>> {
        if let Some(actual_c_token) = it.peek() {
            match actual_c_token { 
                TokenTree::Punct(punct) => {
                    let actual_c = punct.as_char();
                    if actual_c == c {
                        Ok(())
                    } else {
                        Err(Some(actual_c_token.clone()))
                    }
                },
                _ => Err(Some(actual_c_token.clone()))
            }
        } else {
            Err(None)
        }
    }

    pub fn on_html_tag_parsed(
        self: &Self,
        punct: &Punct,
        parse_ctx: &mut HtmlTagParseContext,
        output: &mut Vec<RustHtmlToken>
    ) -> Result<bool, RustHtmlError> {
        // println!("on_html_tag_parsed tag: {}, attrs: {:?}", parse_ctx.tag_name_as_str(), parse_ctx.html_attrs);

        if parse_ctx.is_opening_tag {
            if parse_ctx.html_attr_key.len() > 0 {
                self.on_kvp_defined(parse_ctx, output);
            }
            
            match parse_ctx.tag_name_as_str().as_str() {
                "input" | "!DOCTYPE" => {
                },
                "environment" if parse_ctx.is_opening_tag => {
                },
                _ => {}
            }

            output.push(
                if parse_ctx.is_void_tag() {
                    // println!("Closed void tag {}", parse_ctx.tag_name_as_str());
                    RustHtmlToken::HtmlTagCloseVoidPunct(punct.clone())
                } else if parse_ctx.is_self_contained_tag {
                    // println!("Self contained tag: {}", parse_ctx.tag_name_as_str());
                    RustHtmlToken::HtmlTagCloseSelfContainedPunct(punct.clone())
                } else {
                    // println!("Closed and starting children for {}", parse_ctx.tag_name_as_str());
                    RustHtmlToken::HtmlTagCloseStartChildrenPunct(punct.clone())
                }
            );
            return Ok(true); // parse_ctx.is_void_tag() break if void tag, no children
        } else {
            return Ok(true); // break when closing
        }
    }

    pub fn on_html_node_parsed(
        self: &Self,
        parse_ctx: HtmlTagParseContext,
        output: &mut Vec<RustHtmlToken>
    ) -> Result<bool, RustHtmlError> {
        // println!("on_html_node_parsed tag: {}, attrs: {:?}", parse_ctx.tag_name_as_str(), parse_ctx.html_attrs);
        match parse_ctx.tag_name_as_str().as_str() {
            "input" => {
            },
            "environment" if parse_ctx.is_opening_tag => {
                // look for include or exclude attributes
                let mut keep_or_remove: Option<bool> = None;

                match parse_ctx.html_attrs.get("include") {
                    Some(token) => {
                        match token.clone().unwrap() {
                            RustHtmlToken::HtmlTagAttributeValue(v_parts) => {
                                for v in v_parts {
                                    match v {
                                        RustHtmlToken::Literal(literal) => {
                                            let literal_as_str = snailquote::unescape(&literal.to_string()).unwrap();
                                            // println!("literal_as_str: {}", literal_as_str);

                                            if self.environment_name == literal_as_str {
                                                keep_or_remove = Some(true);
                                            } else {
                                                // println!("self.environment_name ({}) does not match literal_as_str ({})", self.environment_name, literal_as_str);
                                                keep_or_remove = Some(false);
                                            }
                                        },
                                        _ => panic!("Unexpected token for environment tag value: {:?}", token),
                                    }
                                }
                            }
                            _ => panic!("Unexpected token for environment tag: {:?}", token),
                        }
                    },
                    None => {
                        // println!("environment tag does not have include field");
                    }
                }
                
                match parse_ctx.html_attrs.get("exclude") {
                    Some(token) => {
                        match token.clone().unwrap() {
                            RustHtmlToken::HtmlTagAttributeValue(v_parts) => {
                                for v in v_parts {
                                    match v {
                                        RustHtmlToken::Literal(literal) => {
                                            let literal_as_str = snailquote::unescape(&literal.to_string()).unwrap();
                                            // println!("literal_as_str: {}", literal_as_str);
                                            if self.environment_name != literal_as_str {
                                                keep_or_remove = Some(true);
                                            } else {
                                                // println!("self.environment_name ({}) DOES match literal_as_str ({})", self.environment_name, literal_as_str);
                                                keep_or_remove = Some(false);
                                            }
                                        },
                                        _ => panic!("Unexpected token for environment tag value: {:?}", token),
                                    }
                                }
                            }
                            _ => panic!("Unexpected token for environment tag: {:?}", token),
                        }
                    },
                    None => {
                        // println!("environment tag does not have exclude field");
                    }
                }
                
                return match keep_or_remove {
                    Some(keep_or_remove) => {
                        if keep_or_remove {
                            // keep - don't add outer environment tags but do add inner elements
                            loop {
                                match output.first().unwrap() {
                                    RustHtmlToken::HtmlTagCloseVoidPunct(_) |
                                    RustHtmlToken::HtmlTagCloseSelfContainedPunct(_) |
                                    RustHtmlToken::HtmlTagCloseStartChildrenPunct(_) => {
                                        output.remove(0);
                                        break;
                                    },
                                    _ => {
                                        output.remove(0);
                                    }
                                }
                            }
                            
                            match output.last().unwrap() {
                                RustHtmlToken::HtmlTagEnd(tag_end, _tag_end_tokens) => {
                                    if tag_end == &parse_ctx.tag_name_as_str() {
                                        let _pop_result = output.pop();
                                        // println!("output.pop(): {:?}", pop_result);
                                    } else {
                                        println!("mismatch while processing environment HTML tag (found {})", tag_end);
                                    }
                                },
                                _ => {}
                            }

                            // do not add environment tag start but do add child nodes
                            Ok(true)
                        } else {
                            // do not add anything
                            Ok(false)
                        }
                    },
                    None => self.panic_or_return_error(format!("rust html tag environment expects attribute 'include' or 'exclude' to be defined (attrs: {:?})", parse_ctx.html_attrs)),
                }
            },
            _ => {}
        }
        
        Ok(true)
    }

    pub fn next_and_parse_html_tag(
        self: &Self,
        token_option: Option<TokenTree>,
        parse_ctx: &mut HtmlTagParseContext,
        output: &mut Vec<RustHtmlToken>,
        it: &mut Peekable<impl Iterator<Item = TokenTree>>,
        is_raw_tokenstream: bool,
    ) -> Result<bool, RustHtmlError> {
        match token_option {
            Some(token) => {
                match token {
                    TokenTree::Ident(ident) => {
                        self.convert_html_ident_to_rusthtmltoken(&ident, parse_ctx, output, it)?;
                    },
                    TokenTree::Literal(literal) => {
                        self.convert_html_literal_to_rusthtmltoken(&literal, parse_ctx, output)?;
                    },
                    TokenTree::Punct(punct) => {
                        if self.convert_html_punct_to_rusthtmltoken(&punct, parse_ctx, output, it, is_raw_tokenstream)? {
                            return Ok(true); // break
                        }
                    },
                    _ => {
                        return self.panic_or_return_error(format!("Unexpected token {:?}", token));
                    },
                }
            },
            _ => {
                return Ok(true);// self.panic_or_return_error(format!("Could not read next token in next_and_parse_html_tag for {}", parse_ctx.tag_name));
            },
        }

        Ok(false) // continue
    }

    pub fn on_kvp_defined(
        self: &Self,
        parse_ctx: &mut HtmlTagParseContext,
        output: &mut Vec<RustHtmlToken>,
    ) {
        // println!("{}={:?}", parse_ctx.html_attr_key, parse_ctx.html_attr_val);

        if let Some(is_literal) = &parse_ctx.html_attr_key_literal {
            output.push(RustHtmlToken::HtmlTagAttributeName(is_literal.to_string(), RustHtmlIdentAndPunctOrLiteral::Literal(is_literal.clone())));
        } else if parse_ctx.html_attr_key_ident.len() > 0 {
            output.push(RustHtmlToken::HtmlTagAttributeName("todo-fixme".to_string(), RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(parse_ctx.html_attr_key_ident.clone())));
        }

        if parse_ctx.html_attr_val.len() > 0 {
            output.push(RustHtmlToken::HtmlTagAttributeEquals(parse_ctx.equals_punct.as_ref().unwrap().clone()));
            output.push(RustHtmlToken::HtmlTagAttributeValue(parse_ctx.html_attr_val.clone()));
            parse_ctx.html_attrs.insert(parse_ctx.html_attr_key.clone(), Some(RustHtmlToken::HtmlTagAttributeValue(parse_ctx.html_attr_val.clone())));
        } else {
            parse_ctx.html_attrs.insert(parse_ctx.html_attr_key.clone(), None);
        }
        
        parse_ctx.clear_attr_kvp();
    }

    pub fn convert_html_ident_to_rusthtmltoken(
        self: &Self, 
        ident: &Ident,
        parse_ctx: &mut HtmlTagParseContext,
        output: &mut Vec<RustHtmlToken>, 
        it: &mut Peekable<impl Iterator<Item = TokenTree>>
    ) -> Result<(), RustHtmlError> {
        if parse_ctx.parse_attrs {
            if parse_ctx.parse_attr_val {
                parse_ctx.html_attr_val.push(RustHtmlToken::Identifier(ident.clone()));
                parse_ctx.parse_attr_val = false;
            } else {
                parse_ctx.html_attr_key_ident.push(RustHtmlIdentOrPunct::Ident(ident.clone()));
                parse_ctx.html_attr_key.push_str(&ident.to_string());
            }
        } else {
            parse_ctx.tag_name.push(RustHtmlIdentOrPunct::Ident(ident.clone()));
            let mut last_token_was_ident = true;
            loop {
                let next_token = it.peek().unwrap();
                match next_token {
                    TokenTree::Punct(punct) if punct.as_char() == '-' => {
                        parse_ctx.tag_name.push(RustHtmlIdentOrPunct::Punct(punct.clone()));
                        it.next();
                        last_token_was_ident = false;
                    },
                    TokenTree::Ident(ident) if last_token_was_ident == false => {
                        parse_ctx.tag_name.push(RustHtmlIdentOrPunct::Ident(ident.clone()));
                        it.next();
                        last_token_was_ident = true;
                    },
                    _ => {
                        parse_ctx.on_html_tag_name_parsed(output);
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn convert_html_literal_to_rusthtmltoken(
        self: &Self, 
        literal: &Literal,
        parse_ctx: &mut HtmlTagParseContext,
        output: &mut Vec<RustHtmlToken>, 
    ) -> Result<(), RustHtmlError> {
        if parse_ctx.parse_attrs {
            // println!("literal.to_string(): {}", literal.to_string());
            if parse_ctx.parse_attr_val {
                parse_ctx.html_attr_val.push(RustHtmlToken::Literal(literal.clone()));
                self.on_kvp_defined(parse_ctx, output);
            } else {
                parse_ctx.html_attr_key_literal = Some(literal.clone());
                parse_ctx.html_attr_key.push_str(&literal.to_string());
                parse_ctx.parse_attr_val = true;
            }
        } else {
            return Err(RustHtmlError(Cow::Owned(format!("Cannot use literal for tag name"))))
        }

        Ok(())
    }

    pub fn convert_html_punct_to_rusthtmltoken(
        self: &Self, 
        punct: &Punct,
        parse_ctx: &mut HtmlTagParseContext,
        output: &mut Vec<RustHtmlToken>, 
        it: &mut Peekable<impl Iterator<Item = TokenTree>>,
        _is_raw_tokenstream: bool,
    ) -> Result<bool, RustHtmlError> {
        let c = punct.as_char();
        // println!("c: {}", c);
        if parse_ctx.parse_attrs {
            match c {
                '>' => {
                    return self.on_html_tag_parsed(punct, parse_ctx, output);
                },
                '=' => {
                    parse_ctx.equals_punct = Some(punct.clone());
                    parse_ctx.parse_attr_val = true;
                },
                '/' => {
                    let expect_closing_punct = it.next().unwrap();
                    match expect_closing_punct {
                        TokenTree::Punct(closing_punct) => {
                            if closing_punct.as_char() == '>' {
                                parse_ctx.is_self_contained_tag = true;
                                return self.on_html_tag_parsed(&closing_punct, parse_ctx, output);
                            } else {
                                return self.panic_or_return_error(format!("Unexpected character '{}' (expected '>', prev: '{}')", closing_punct, c));
                            }
                        },
                        _ => {
                            return self.panic_or_return_error(format!("Unexpected token after /: {}", c));
                        },
                    }
                },
                '"' => {
                    if parse_ctx.html_attr_key.len() > 0 {
                        parse_ctx.parse_attr_val = true;
                    } else if parse_ctx.html_attr_val.len() > 0 {
                        self.on_kvp_defined(parse_ctx, output);
                    }
                },
                '-' => {
                    if parse_ctx.parse_attr_val {
                        parse_ctx.html_attr_val.push(RustHtmlToken::ReservedChar(c, punct.clone()));
                    } else {
                        parse_ctx.html_attr_key_ident.push(RustHtmlIdentOrPunct::Punct(punct.clone()));
                        parse_ctx.html_attr_key.push_str(format!("{}", c).as_str());
                    }
                },
                '@' => {
                    // escaping the html to insert value
                    let directive_ident = it.next().unwrap();
                    match directive_ident {
                        TokenTree::Ident(ident) => {
                            self.parse_identifier_expression(ident, &mut parse_ctx.html_attr_val, it)?;
                        },
                        TokenTree::Literal(literal) => {
                            parse_ctx.html_attr_val.push(RustHtmlToken::Literal(literal.clone()));
                        },
                        _ => {
                            return self.panic_or_return_error(format!("Unexpected directive token after '@' in html attribute val parse: {:?}", directive_ident))?;
                        }
                    }
                    // println!("parse_ctx.html_attr_val: {:?}", parse_ctx.html_attr_val);
                    self.on_kvp_defined(parse_ctx, output);
                }
                _ => {
                    return self.panic_or_return_error(format!(
                        "Unexpected punct '{}' while parsing HTML tag '{}' attributes \
                        (read {:?}, current key: {}, current val: {:?})", c, parse_ctx.tag_name_as_str(),
                        parse_ctx.html_attrs, parse_ctx.html_attr_key, parse_ctx.html_attr_val));
                }
            }
        } else {
            match c {
                '>' => {
                    return self.on_html_tag_parsed(punct, parse_ctx, output);
                },
                '/' => {
                    if parse_ctx.tag_name.len() > 0 {
                        let expect_closing_punct = it.next().unwrap();
                        return match expect_closing_punct {
                            TokenTree::Punct(closing_punct) => {
                                if closing_punct.as_char() == '>' {
                                    parse_ctx.is_self_contained_tag = true;
                                    return self.on_html_tag_parsed(&closing_punct, parse_ctx, output);
                                } else {
                                    self.panic_or_return_error(format!("Unexpected character '{}' (expected '>', prev: '{}')", closing_punct, c))
                                }
                            },
                            _ => {
                                self.panic_or_return_error(format!("Unexpected token after / (tag_name = {}): {:?}", parse_ctx.tag_name_as_str(), expect_closing_punct))
                            },
                        };
                    } else {
                        parse_ctx.is_opening_tag = false;
                    }
                },
                '-' | '_' | '!' => {
                    parse_ctx.tag_name.push(RustHtmlIdentOrPunct::Punct(punct.clone()));
                },
                _ => {
                    return self.panic_or_return_error(format!("Unexpected character '{}'", c));
                },
            }
        }
        Ok(false) // do not break
    }

    pub fn convert_rusthtml_directive_to_rusthtmltoken(self: &Self, token: TokenTree, prefix_token_option: Option<RustHtmlToken>, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>, is_raw_tokenstream: bool) -> Result<bool, RustHtmlError>  {
        // println!("convert_rusthtml_directive_to_rusthtmltoken: {:?}", token);
        match token {
            TokenTree::Ident(ident) => {
                // println!("ident: {}", ident.to_string());
                self.convert_rusthtml_directive_identifier_to_rusthtmltoken(ident, prefix_token_option, output, it, is_raw_tokenstream)?;
            },
            TokenTree::Group(group) => {
                self.convert_rusthtml_directive_group_to_rusthtmltoken(group, prefix_token_option, output, is_raw_tokenstream)?;
            },
            TokenTree::Literal(literal) => {
                // println!("literal: {}", literal.to_string());
                output.push(RustHtmlToken::AppendToHtml(vec![RustHtmlToken::Literal(literal.clone())]));
                // self.convert_rusthtml_literal_to_rusthtmltoken(group, output, it);
            },
            TokenTree::Punct(punct) => {
                let c = punct.as_char();
                match c {
                    '@' => {
                        // escape '@'
                        // println!("escaped '@'");
                        output.push(RustHtmlToken::AppendToHtml(vec![RustHtmlToken::ReservedChar(c, punct.clone())]));
                    },
                    '&' => {
                        let prefix_token = RustHtmlToken::ReservedChar(c, punct.clone());
                        
                        let next_token = it.next();
                        if let Some(token) = next_token {
                            return self.convert_rusthtml_directive_to_rusthtmltoken(token, Some(prefix_token), output, it, is_raw_tokenstream);
                        }
                    },
                    _ => {
                        return self.panic_or_return_error(format!("unexpected directive char: {}", c))?;
                    }
                }
            },
        }
        Ok(true)
    }

    pub fn convert_rusthtml_directive_group_to_rusthtmltoken(self: &Self, group: Group, prefix_token_option: Option<RustHtmlToken>, output: &mut Vec<RustHtmlToken>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError> {
        let mut inner_tokens = vec![];
        self.loop_next_and_convert(false, &mut inner_tokens, group.stream().into_iter().peekable().by_ref(), is_raw_tokenstream)?;
        if inner_tokens.len() > 0 {
            //println!("inner_tokens: {:?}", inner_tokens);
            let delimiter = group.delimiter();
            match delimiter {
                Delimiter::Brace => {
                    output.extend_from_slice(&inner_tokens);
                },
                Delimiter::Parenthesis => {
                    output.push(RustHtmlToken::AppendToHtml(inner_tokens));
                },
                _ => {
                    return self.panic_or_return_error(format!("unexpected delimiter: {:?}", delimiter));
                },
            }
        }
        Ok(())
    }

    pub fn convert_rusthtml_directive_identifier_to_rusthtmltoken(self: &Self, identifier: Ident, prefix_token_option: Option<RustHtmlToken>, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError> {
        // println!("convert_rusthtml_directive_identifier_to_rusthtmltoken: {}", identifier);
        match identifier.to_string().as_str() {
            "name" | "viewstart" => {
                let param_value = self.parse_string_with_quotes(identifier.clone(), it)?;
                // println!("ident {} val: {}", identifier.to_string().clone(), param_value);
                self.params.borrow_mut().insert(identifier.to_string().clone(), param_value);
            },
            "use" => {
                let type_ident_tokens = self.parse_type_identifier(it)?; // expecting type identifier
                let inner_tokenstream = proc_macro2::TokenStream::from(TokenStream::from_iter(type_ident_tokens));
                self.use_statements.borrow_mut().push(TokenStream::from(quote! { use #inner_tokenstream; }));
            },
            "model" => {
                let type_ident = self.parse_type_identifier(it)?; // expecting type identifier
                self.model_type.replace(Some(type_ident));
            },
            "for" => {
                output.push(RustHtmlToken::Identifier(identifier));
                // read until we reach the loop body {}
                self.parse_for_or_while_loop_preamble(output, it, is_raw_tokenstream)?;
            },
            "while" => {
                output.push(RustHtmlToken::Identifier(identifier));
                // read until we reach the loop body {}
                self.parse_for_or_while_loop_preamble(output, it, is_raw_tokenstream)?;
            },
            "if" => {
                output.push(RustHtmlToken::Identifier(identifier));
                self.parse_for_or_while_loop_preamble(output, it, is_raw_tokenstream)?;
            },
            "let" => {
                output.push(RustHtmlToken::Identifier(identifier));
                self.parse_let(output, it)?;
            },
            "functions" => {
                // expecting group
                let group_token = it.next().unwrap();
                match group_token {
                    TokenTree::Group(group) => {
                        self.functions_section.replace(Some(group.stream()));
                    },
                    _ => {
                        return self.panic_or_return_error(format!("unexpected token after functions directive: {:?}", group_token));
                    }
                }
            },
            "impl" => {
                // expecting group
                let group_token = it.next().unwrap();
                match group_token {
                    TokenTree::Group(group) => {
                        self.impl_section.replace(Some(group.stream()));
                    },
                    _ => {
                        return self.panic_or_return_error(format!("unexpected token after functions directive: {:?}", group_token));
                    }
                }
            },
            "struct" => {
                // expecting group
                let group_token = it.next().unwrap();
                match group_token {
                    TokenTree::Group(group) => {
                        self.struct_section.replace(Some(group.stream()));
                    },
                    _ => {
                        return self.panic_or_return_error(format!("unexpected token after functions directive: {:?}", group_token));
                    }
                }
            },
            "rshtmlfile" => {
                self.convert_externalrusthtml_directive(identifier, output, it)?;
            },
            "htmlfile" => {
                self.convert_externalhtml_directive(identifier, output, it)?;
            },
            "mdfile_const" => {
                self.convert_mdfile_const_directive(identifier, output, it)?;
            },
            "mdfile_nocache" => {
                self.convert_mdfile_nocache_directive(identifier, output, it)?;
            },
            _ => {
                let mut inner_tokens = vec![];
                if let Some(prefix_token) = prefix_token_option {
                    inner_tokens.push(prefix_token);
                }
                self.parse_identifier_expression(identifier, &mut inner_tokens, it)?;
                output.push(RustHtmlToken::AppendToHtml(inner_tokens));
            }
        }
        Ok(())
    }

    pub fn convert_path_str(self: &Self, identifier: Ident, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<String, RustHtmlError> {
        let mut path = std::path::PathBuf::new();

        let cwd = std::env::current_dir().unwrap();
        path.push(cwd);
        let relative_path = self.parse_string_with_quotes(identifier.clone(), it)?;
        path.push(relative_path.clone());
        // println!("convert_path_str: {:?} -> {}", identifier, relative_path);

        Ok(path.to_str().unwrap().to_string())
    }

    pub fn convert_externalrusthtml_directive(self: &Self, identifier: Ident, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<(), RustHtmlError> {
        let path = self.convert_path_str(identifier.clone(), it)?;
        match std::fs::File::open(&path) {
            Ok(_f) => {
                // output.push(RustHtmlToken::ExternalRustHtml(path.clone(), identifier.span()));
                self.expand_external_tokenstream(&path, output)?;
            },
            Err(e) => {
                return self.panic_or_return_error(format!("cannot read external Rust HTML file '{}', could not open: {:?}", path, e));
            }
        }
        Ok(())
    }

    pub fn expand_external_tokenstream(self: &Self, path: &String, output: &mut Vec<RustHtmlToken>) -> Result<(), RustHtmlError> {
        let input_str = std::fs::read_to_string(path).unwrap();
        let input_result = TokenStream::from_str(input_str.as_str());
        
        match input_result {
            Ok(input) => {
                // println!("Expanding external token stream: {}", path);
                let mut it = input.into_iter().peekable();
                let rusthtml_tokens = self.parse_tokenstream_to_rusthtmltokens(true, it.by_ref(), true)?;
                output.extend_from_slice(&rusthtml_tokens);
            },
            Err(e) => {
                return self.panic_or_return_error(format!("{}", e));
            },
        }

        Ok(())
    }

    pub fn convert_externalhtml_directive(self: &Self, identifier: Ident, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<(), RustHtmlError> {
        let path = self.convert_path_str(identifier.clone(), it)?;
        match std::fs::File::open(path.as_str()) {
            Ok(_f) => {
                output.push(RustHtmlToken::ExternalHtml(path, identifier.span()));
            },
            Err(e) => {
                return self.panic_or_return_error(format!("cannot read external HTML file '{}', could not open: {:?}", path, e))?;
            }
        }
        Ok(())
    }

    pub fn convert_mdfile_const_directive(self: &Self, identifier: Ident, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<(), RustHtmlError> {
        let path = self.convert_path_str(identifier.clone(), it)?;
        match std::fs::File::open(path.as_str()) {
            Ok(mut f) => {
                let mut buffer = String::new();
                f.read_to_string(&mut buffer).expect("could not read markdown file");
                let mdtext = comrak::markdown_to_html(&buffer, &comrak::ComrakOptions::default());
                output.push(RustHtmlToken::HtmlTextNode(mdtext, identifier.span()));
            },
            Err(e) => {
                println!("convert_mdfile_const_directive: could not find {}", path);
                return self.panic_or_return_error(format!("cannot read external markdown file '{}', could not open: {:?}", path, e))?;
            }
        }
        Ok(())
    }

    pub fn convert_string_or_ident(self: &Self, _identifier: Ident, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<RustHtmlIdentAndPunctAndGroupOrLiteral, RustHtmlError> {
        let expect_string_or_ident_token = it.next().unwrap();
        match expect_string_or_ident_token {
            TokenTree::Literal(literal) => {
                Ok(RustHtmlIdentAndPunctAndGroupOrLiteral::Literal(literal.clone()))
            },
            TokenTree::Ident(ident2) => {
                let mut inner_tokens = vec![];
                self.parse_identifier_expression(ident2.clone(), &mut inner_tokens, it)?;
                // println!("inner_tokens: {:?}", inner_tokens);
                Ok(RustHtmlIdentAndPunctAndGroupOrLiteral::IdentAndPunctAndGroup(self.convert_rusthtmltokens_to_ident_or_punct_or_group(inner_tokens)?))
            },
            _ => {
                self.panic_or_return_error(format!("convert_string_or_ident did not find string or ident"))?
            }
        }
    }

    pub fn convert_rusthtmltokens_to_ident_or_punct_or_group(self: &Self, tokens: Vec<RustHtmlToken>) -> Result<Vec<RustHtmlIdentOrPunctOrGroup>, RustHtmlError> {
        if tokens.len() == 0 {
            return self.panic_or_return_error(format!("tokens was empty"))?;
        }

        Ok(tokens.iter()
            .map(|x| match x {
                RustHtmlToken::Identifier(ident) => RustHtmlIdentOrPunctOrGroup::Ident(ident.clone()),
                RustHtmlToken::ReservedChar(_, punct) => RustHtmlIdentOrPunctOrGroup::Punct(punct.clone()),
                RustHtmlToken::Group(_, group) => RustHtmlIdentOrPunctOrGroup::Group(group.clone()),
                _ => panic!("Unexpected token {:?}", x),
            })
            .collect())
    }

    pub fn convert_mdfile_nocache_directive(self: &Self, identifier: Ident, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<(), RustHtmlError> {
        // could be literal or ident
        let path_tokens = self.convert_string_or_ident(identifier.clone(), it)?;
        let path = proc_macro2::TokenStream::from(self.convert_ident_and_punct_and_group_or_literal_to_tokenstream(&path_tokens)?);
        let tokenstream = quote! {
            let mut path = std::path::PathBuf::new();
            let cwd = std::env::current_dir().unwrap();
            path.push(cwd);
            path.push(#path);

            match std::fs::File::open(path.to_str().unwrap().to_string()) {
                Ok(mut f) => {
                    let mut buffer = String::new();
                    f.read_to_string(&mut buffer).expect("could not read markdown file");
                    view_context.write_html_str(comrak::markdown_to_html(&buffer, &comrak::ComrakOptions::default()).as_str());
                },
                Err(e) => {
                    println!("convert_mdfile_nocache_directive: could not find {}", #path);
                    return Err(RustHtmlError(Cow::Owned(format!("cannot read external markdown file '{}', could not open: {:?}", #path, e))));
                }
            }
        };
        output.push(RustHtmlToken::Group(Delimiter::None, Group::new(Delimiter::None, TokenStream::from(tokenstream))));

        Ok(())
    }

    pub fn parse_string_with_quotes(self: &Self, identifier: Ident, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<String, RustHtmlError> {
        let expect_string_token = it.next().unwrap();
        match expect_string_token {
            TokenTree::Literal(literal) => Ok(snailquote::unescape(&literal.to_string()).unwrap()),
            _ => self.panic_or_return_error(format!("unexpected token after {} directive: {:?}", identifier, expect_string_token))?
        }
    }

    pub fn parse_for_or_while_loop_preamble(self: &Self, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError> {
        // already parsed and added first ident (for or while)
        // only allow parsing ident, literal, and punct if punct != '{'
        loop {
            let token_option = it.next();
            match token_option {
                Some(token) => {
                    match token {
                        TokenTree::Ident(ident) => {
                            output.push(RustHtmlToken::Identifier(ident.clone()));
                        },
                        TokenTree::Literal(literal) => {
                            output.push(RustHtmlToken::Literal(literal.clone()));
                        },
                        TokenTree::Punct(punct) => {
                            let c = punct.as_char();
                            match c {
                                '{' => {
                                    panic!("unexpected {{");
                                },
                                _ => {
                                    output.push(RustHtmlToken::ReservedChar(c, punct.clone()));
                                }
                            }
                        },
                        TokenTree::Group(group) => {
                            let delimiter = group.delimiter();
                            match delimiter {
                                Delimiter::Brace => {
                                    self.convert_group_to_rusthtmltoken(group, false, output, is_raw_tokenstream)?;
                                    break;
                                },
                                _ => {
                                    output.push(RustHtmlToken::Group(delimiter, group.clone()));
                                },
                            }
                        },
                    }
                },
                None => {
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn parse_identifier_expression(self: &Self, identifier: Ident, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<(), RustHtmlError> {
        // println!("first identifier: {:?}", identifier);
        output.push(RustHtmlToken::Identifier(identifier.clone()));
        let mut last_token_was_ident = true;
        loop {
            let param_value_token_option = it.peek();
            if let Some(param_value_token) = param_value_token_option {
                // println!("param_value_token: {:?}", param_value_token);
                match param_value_token {
                    TokenTree::Literal(literal) => {
                        output.push(RustHtmlToken::Literal(literal.clone()));
                        it.next();
                    },
                    TokenTree::Ident(ident) => {
                        if last_token_was_ident {
                            break;
                        } else {
                            output.push(RustHtmlToken::Identifier(ident.clone()));
                            it.next();
                            last_token_was_ident = true;
                            continue;
                        }
                    },
                    TokenTree::Group(group) => {
                        let delimeter = group.delimiter();
                        output.push(RustHtmlToken::Group(delimeter, group.clone()));
                        it.next();

                        // not a function call or index
                        if delimeter != Delimiter::Parenthesis && delimeter != Delimiter::Bracket {
                            break;
                        }
                    },
                    TokenTree::Punct(punct) => {
                        let c = punct.as_char();
                        match c {
                            '.' | '?' | '!' | '_' => {
                                output.push(RustHtmlToken::ReservedChar(c, punct.clone()));
                                it.next();
                            },
                            _ => break
                        }
                    },
                }
            } else {
                break;
            }

            last_token_was_ident = false;
        }
        Ok(())
    }

    pub fn insert_self_dot(self: &Self, output: &mut Vec<RustHtmlToken>) {
        output.push(RustHtmlToken::Identifier(Ident::new("self", Span::call_site())));
        output.push(RustHtmlToken::ReservedChar('.', Punct::new('.', Spacing::Alone)));
    }

    pub fn parse_let(self: &Self, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<(), RustHtmlError> {
        loop
        {
            let token = it.next().unwrap();
            match token.clone() {
                TokenTree::Punct(punct) => {
                    let c = punct.as_char();
                    output.push(RustHtmlToken::ReservedChar(c, punct));
                    if c == ';' {
                        break;
                    }
                },
                _ => self.convert_copy(token, output)?,
            }
        }
        Ok(())
    }

    pub fn convert_copy(self: &Self, token: TokenTree, output: &mut Vec<RustHtmlToken>) -> Result<(), RustHtmlError> {
        match token.clone() {
            TokenTree::Literal(literal) => {
                output.push(RustHtmlToken::Literal(literal.clone()));
            },
            TokenTree::Ident(ident) => {
                output.push(RustHtmlToken::Identifier(ident.clone()));
            },
            TokenTree::Group(group) => {
                output.push(RustHtmlToken::Group(group.delimiter(), group.clone()));
            },
            _ => {
                return self.panic_or_return_error(format!("unexpected token: {:?}", token));
            },
        }
        Ok(())
    }

    pub fn parse_type_identifier(self: &Self, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let mut type_parts: Vec<TokenTree> = vec![];
        loop
        {
            let ident_token = it.peek().unwrap();
            match ident_token.clone() {
                TokenTree::Ident(_ident) => {
                    type_parts.push(it.next().unwrap().clone());
                    for _ in 0..2 {
                        let peek_after_ident = it.peek().unwrap().clone();
                        match peek_after_ident.clone() {
                            TokenTree::Punct(punct) => {
                                match punct.as_char() {
                                    ':' => {
                                        type_parts.push(it.next().unwrap().clone());
                                    },
                                    _ => break,
                                }
                            },
                            _ => break,
                        }
                    }
                },
                TokenTree::Punct(punct) => {
                    match punct.as_char() {
                        '_' => type_parts.push(it.next().unwrap().clone()),
                        _ =>  break,
                    }
                },
                _ => {
                    return self.panic_or_return_error(format!("unexpected token after model directive: {:?}", ident_token));
                }
            }
        }
        Ok(type_parts)
    }

    pub fn convert_rusthtmltoken_to_tokentree<'a>(self: &Self, token: &RustHtmlToken, output: &mut Vec<TokenTree>, it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<bool, RustHtmlError> {
        match token {
            RustHtmlToken::Identifier(ident) => output.push(TokenTree::Ident(ident.clone())),
            RustHtmlToken::Literal(literal) => output.push(TokenTree::Literal(literal.clone())),
            RustHtmlToken::ReservedChar(_, punct) => output.push(TokenTree::Punct(punct.clone())),
            RustHtmlToken::Group(_delimiter, group) => output.push(TokenTree::Group(group.clone())),
            RustHtmlToken::GroupParsed(delimiter, inner_tokens) => 
                self.convert_rusthtmlgroupparsed_to_tokentree(delimiter, inner_tokens, output, it)?,
            RustHtmlToken::HtmlTagStart(_tag, tag_tokens) =>
                self.convert_rusthtmltagstart_to_tokentree(tag_tokens, output, it)?,
            RustHtmlToken::HtmlTagVoid(_tag, tag_tokens) =>
                self.convert_rusthtmltagstart_to_tokentree(tag_tokens, output, it)?,
            RustHtmlToken::HtmlTagEnd(_tag, tag_tokens) =>
                self.convert_rusthtmltagend_to_tokentree(tag_tokens, output, it)?,
            RustHtmlToken::HtmlTagCloseStartChildrenPunct(_punct) =>
                self.convert_rusthtmltagclosestartchildren_to_tokentree(output, it)?,
            RustHtmlToken::HtmlTagCloseSelfContainedPunct(_punct) =>
                self.convert_rusthtmltagclosesselfcontained_to_tokentree(output, it)?,
            RustHtmlToken::HtmlTagCloseVoidPunct(_punct) =>
                self.convert_rusthtmltagclosevoid_to_tokentree(output, it)?,
            RustHtmlToken::HtmlTagAttributeEquals(_punct) =>
                self.convert_rusthtmltagattributeequals_to_tokentree(output, it)?,
            RustHtmlToken::HtmlTagAttributeName(_tag, tag_tokens) =>
                self.convert_rusthtmltagattributename_to_tokentree(tag_tokens, output, it)?,
            RustHtmlToken::HtmlTagAttributeValue(v) =>
                self.convert_rusthtmltagattributevalue_to_tokentree(v.clone(), output, it)?,
            // RustHtmlToken::HtmlTagAttributeValueString(s) =>
            //     self.convert_rusthtmltagattributevaluestring_to_tokentree(s, output, it)?,
            RustHtmlToken::HtmlTextNode(text, span) => 
                self.convert_rusthtmltextnode_to_tokentree(text, span, output, it)?,
            RustHtmlToken::AppendToHtml(inner) =>
                self.convert_rusthtmlappendhtml_to_tokentree(inner, output)?,
            RustHtmlToken::ExternalHtml(path, span) =>
                self.convert_htmlexternal_to_tokentree(path, span.clone(), output, it)?,
            _ => { return Err(RustHtmlError::from_string(format!("Could not handle token {:?}", token))); }
        }
        Ok(false)
    }

    pub fn convert_htmlexternal_to_tokentree<'a>(self: &Self, path: &String, span: Span, output: &mut Vec<TokenTree>, it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        let content = std::fs::read_to_string(path).unwrap();
        self.convert_rusthtmltextnode_to_tokentree(&content, &span, output, it)
    }

    pub fn convert_rusthtmltagstart_to_tokentree<'a>(self: &Self, tag: &Vec<RustHtmlIdentOrPunct>, output: &mut Vec<TokenTree>, _it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        let tag_as_html = format!("<{}", HtmlTagParseContext::fmt_tag_name_as_str(tag));
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { view_context.write_html_str(#tag_as_html); }))));

        Ok(())
    }

    pub fn convert_rusthtmltagend_to_tokentree<'a>(self: &Self, tag: &Vec<RustHtmlIdentOrPunct>, output: &mut Vec<TokenTree>, _it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        let tag_as_html = format!("</{}>", HtmlTagParseContext::fmt_tag_name_as_str(tag));
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { view_context.write_html_str(#tag_as_html); }))));
        Ok(())
    }

    pub fn convert_rusthtmltagclosestartchildren_to_tokentree<'a>(self: &Self, output: &mut Vec<TokenTree>, _it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { view_context.write_html_str(">"); }))));
        Ok(())
    }
    
    pub fn convert_rusthtmltagclosesselfcontained_to_tokentree<'a>(self: &Self, output: &mut Vec<TokenTree>, _it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { view_context.write_html_str("/>"); }))));
        Ok(())
    }

    pub fn convert_rusthtmltagclosevoid_to_tokentree<'a>(self: &Self, output: &mut Vec<TokenTree>, _it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { view_context.write_html_str(">"); }))));
        Ok(())
    }

    pub fn convert_rusthtmltagattributeequals_to_tokentree<'a>(self: &Self, output: &mut Vec<TokenTree>, _it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { view_context.write_html_str("="); }))));
        Ok(())
    }

    pub fn convert_ident_and_punct_or_literal_to_tokenstream(self: &Self, tag: &RustHtmlIdentAndPunctOrLiteral) -> Result<TokenStream, RustHtmlError> {
        Ok(TokenStream::from_iter(match tag {
            RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(ident_and_punct) => {
                if ident_and_punct.len() == 0 {
                    return self.panic_or_return_error(format!("ident_and_punct was empty"))?;
                }
        
                ident_and_punct.iter()
                    .map(|x| match x {
                        RustHtmlIdentOrPunct::Ident(ident) => TokenTree::Ident(ident.clone()),
                        RustHtmlIdentOrPunct::Punct(punct) => TokenTree::Punct(punct.clone()),
                    })
                    .collect()
            },
            RustHtmlIdentAndPunctOrLiteral::Literal(literal) => vec![TokenTree::Literal(literal.clone())],
        }.iter().cloned()))
    }

    pub fn convert_ident_and_punct_and_group_or_literal_to_tokenstream(self: &Self, tag: &RustHtmlIdentAndPunctAndGroupOrLiteral) -> Result<TokenStream, RustHtmlError> {
        Ok(TokenStream::from_iter(match tag {
            RustHtmlIdentAndPunctAndGroupOrLiteral::IdentAndPunctAndGroup(ident_and_punct) => {
                if ident_and_punct.len() == 0 {
                    return self.panic_or_return_error(format!("ident_and_punct was empty"))?;
                }
        
                ident_and_punct.iter()
                    .map(|x| match x {
                        RustHtmlIdentOrPunctOrGroup::Ident(ident) => TokenTree::Ident(ident.clone()),
                        RustHtmlIdentOrPunctOrGroup::Punct(punct) => TokenTree::Punct(punct.clone()),
                        RustHtmlIdentOrPunctOrGroup::Group(group) => TokenTree::Group(group.clone()),
                    })
                    .collect()
            },
            RustHtmlIdentAndPunctAndGroupOrLiteral::Literal(literal) => vec![TokenTree::Literal(literal.clone())],
        }.iter().cloned()))
    }

    pub fn convert_rusthtmltagattributename_to_tokentree<'a>(self: &Self, tag: &RustHtmlIdentAndPunctOrLiteral, output: &mut Vec<TokenTree>, _it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { view_context.write_html_str(" "); }))));
        
        let tag_as_html = match tag {
            RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(ident_and_punct) => HtmlTagParseContext::fmt_tag_name_as_str(ident_and_punct),
            RustHtmlIdentAndPunctOrLiteral::Literal(literal) => literal.to_string(),
        };
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { view_context.write_html_str(#tag_as_html); }))));
        Ok(())
    }

    pub fn convert_rusthtmltagattributevaluestring_to_tokentree<'a>(self: &Self, v: &String, output: &mut Vec<TokenTree>, it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        self.convert_appendhtmlstring_to_tokentree(v.to_string(), output, it)?;
        Ok(())
    }

    pub fn convert_rusthtmltagattributevalue_to_tokentree<'a>(self: &Self, v: Vec<RustHtmlToken>, output: &mut Vec<TokenTree>, it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        self.convert_appendhtmlstring_to_tokentree("\"".to_string(), output, it)?;
        // inner tokens
        self.convert_rusthtmlappendhtml_to_tokentree(&v, output)?;
        self.convert_appendhtmlstring_to_tokentree("\"".to_string(), output, it)?;
        Ok(())
    }

    pub fn convert_rusthtmltextnode_to_tokentree<'a>(self: &Self, first_text: &String, _first_span: &Span, output: &mut Vec<TokenTree>, it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        let mut text_node_content = Vec::new();
        text_node_content.push(first_text.clone());

        loop {
            let peek_token_option = it.peek();
            if let Some(peek_token) = peek_token_option {
                if let RustHtmlToken::HtmlTextNode(text, _span) = peek_token {
                    text_node_content.push(text.clone());
                    it.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        let text = text_node_content.join("");
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { view_context.write_html_str(#text); }))));
        Ok(())
    }

    pub fn convert_rusthtmlgroupparsed_to_tokentree<'a>(self: &Self, delimiter: &Delimiter, inner_tokens: &Vec<RustHtmlToken>, output: &mut Vec<TokenTree>, _it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        let mut group = vec![];
        let mut inner_it = inner_tokens.iter().peekable();
        self.convert_rusthtmltokens_to_plain_rust(&mut group, &mut inner_it)?;
        output.push(TokenTree::Group(Group::new(delimiter.clone(), TokenStream::from_iter(group.iter().cloned()))));
        Ok(())
    }

    pub fn convert_rusthtmlappendhtml_to_tokentree<'a>(self: &Self, inner: &Vec<RustHtmlToken>, output: &mut Vec<TokenTree>) -> Result<(), RustHtmlError> {
        let mut inner_tokens = vec![];
        let mut inner_it = inner.iter().peekable();
        self.convert_rusthtmltokens_to_plain_rust(&mut inner_tokens, &mut inner_it)?;
        let inner_tokenstream1 = TokenStream::from_iter(inner_tokens);
        let inner_tokenstream = proc_macro2::TokenStream::from(inner_tokenstream1);
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { view_context.write_html((#inner_tokenstream).into()); }))));
        Ok(())
    }

    pub fn convert_appendhtmlstring_to_tokentree<'a>(self: &Self, html_string: String, output: &mut Vec<TokenTree>, _it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { view_context.write_html_str((#html_string).into()); }))));
        Ok(())
    }
}
