// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs

// use crate::error::HtmlParseError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::iter::Peekable;
use std::rc::Rc;

use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use proc_macro2::token_stream::IntoIter;
use quote::{quote, quote_spanned};

use crate::view::rusthtml::rusthtml_node::RustHtmlNode;
use crate::view::rusthtml::rusthtml_lang_part::RustHtmlLangPart;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

pub struct RustHtmlParser {
    pub lang_parts: Vec<Rc<dyn RustHtmlLangPart>>,
    pub lang_part_stack: RefCell<Vec<Rc<dyn RustHtmlLangPart>>>,

    pub punctuation_scope_stack: RefCell<Vec<char>>,
    pub params: RefCell<HashMap<String, String>>,
    pub functions_section: RefCell<Option<TokenStream>>,
    pub model_type: RefCell<Option<Vec<TokenTree>>>,
    pub raw: RefCell<String>,
}
impl RustHtmlParser {
    pub fn new() -> Self {
        Self {
            lang_parts: vec![
                Rc::new(crate::view::rusthtml::rusthtml_lang_parts::rust_html::RustHtml { }),
                Rc::new(crate::view::rusthtml::rusthtml_lang_parts::directive::Directive { }),
            ],
            lang_part_stack: RefCell::new(vec![]),
            punctuation_scope_stack: RefCell::new(vec![]),
            params: RefCell::new(HashMap::new()),
            functions_section: RefCell::new(None),
            model_type: RefCell::new(None),
            raw: RefCell::new(String::new()),
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
        self.model_type.borrow().clone().unwrap_or(vec![
            TokenTree::Ident(Ident::new("Rc", Span::call_site())),
            TokenTree::Punct(Punct::new('<', Spacing::Alone)),
            TokenTree::Ident(Ident::new("dyn", Span::call_site())),
            TokenTree::Ident(Ident::new("Any", Span::call_site())),
            TokenTree::Punct(Punct::new('>', Spacing::Alone)),
        ])
    }

    pub fn get_param_string(self: &Self, key: &str) -> String {
        let str_val = 
            self.params.borrow().get(&key.to_string())
            .expect(format!("missing param '@{}' in rusthtml", key).as_str())
            .to_string();

        let mut chars = str_val.chars();
        chars.next();
        chars.next_back();
        chars.collect()
    }

    pub fn get_functions_section(self: &Self) -> Option<TokenStream> {
        if let Some(has_functions) = self.functions_section.borrow().as_ref() {
            Some(has_functions.clone())
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
        let rusthtml_tokens = self.parse_tokenstream_to_rusthtmltokens(true, input)?;
        let rust_output = self.parse_rusthtmltokens_to_plain_rust(rusthtml_tokens)?;

        self.raw.replace(self.display_as_code(&mut rust_output.iter().cloned().peekable()));
        // self.print_as_code(&rust_output);

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

    pub fn parse_to_ast(self: &Self, input: TokenStream) -> Result<syn::Item, RustHtmlError> {
        let ts = self.expand_tokenstream(input)?;
        let ast = syn::parse2(ts).unwrap();
        Ok(ast)
    }

    pub fn parse_tokenstream_to_rusthtmltokens(self: &Self, is_in_html_mode: bool, input: TokenStream) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut rusthtml_tokens = Vec::new();
        self.loop_next_and_convert(is_in_html_mode, &mut rusthtml_tokens, input)?;
        Ok(rusthtml_tokens)
    }

    pub fn loop_next_and_convert(self: &Self, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, input: TokenStream) -> Result<(), RustHtmlError> {
        let mut it = input.into_iter().peekable();
        loop {
            if self.next_and_convert(is_in_html_mode, output, it.by_ref())? {
                break;
            }
        }
        Ok(())
    }

    pub fn next_and_convert(self: &Self, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<bool, RustHtmlError> {
        let token_option = it.next();
        // println!("next_and_convert: {:?}", token_option);
        if let Some(token) = token_option {
            if self.convert_tokentree_to_rusthtmltoken(token, is_in_html_mode, output, it)? {
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
        let mut iter = rusthtml_tokens.iter();
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

    pub fn convert_tokentree_to_rusthtmltoken(self: &Self, token: TokenTree, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<bool, RustHtmlError> {
        match token {
            TokenTree::Ident(ident) => {
                if is_in_html_mode {
                    output.push(RustHtmlToken::HtmlTextNode(ident.to_string()));
                } else {
                    output.push(RustHtmlToken::Identifier(ident));
                }
            },
            TokenTree::Literal(literal) => {
                if is_in_html_mode {
                    output.push(RustHtmlToken::HtmlTextNode(literal.to_string()));
                } else {
                    output.push(RustHtmlToken::Literal(literal));
                }
            },
            TokenTree::Punct(punct) => {
                let c = punct.as_char();
                match c {
                    '@' => {
                        if is_in_html_mode {
                            let directive_ident = it.next().unwrap();
                            self.convert_rusthtml_directive_to_rusthtmltoken(directive_ident, output, it)?;
                        } else {
                            panic!("Cannot escape HTML when already in rust mode (hint: remove '@'?)");
                        }
                    },
                    '<' => {
                        if self.convert_html_to_rusthtmltoken(output, it)? {
                            // println!("Do not continue after tag");
                            return Ok(true); // do not continue
                        }
                        // println!("Continue after tag");
                    },
                    '}' if !is_in_html_mode => {
                        return Ok(true);
                    },
                    _ => {
                        if is_in_html_mode {
                            output.push(RustHtmlToken::HtmlTextNode(punct.as_char().to_string()));
                        } else {
                            output.push(RustHtmlToken::ReservedChar(c, punct));
                        }
                        // println!("beep: '{}'", c);
                    },
                }
            },
            TokenTree::Group(group) => {
                let delimiter = group.delimiter();
                if is_in_html_mode {
                    let c_start = self.get_opening_delim(delimiter);
                    let c_end = self.get_closing_delim(delimiter);

                    output.push(RustHtmlToken::HtmlTextNode(c_start.to_string()));
                    self.loop_next_and_convert(true, output, group.stream())?;
                    output.push(RustHtmlToken::HtmlTextNode(c_end.to_string()));
                } else {
                    if delimiter == Delimiter::Brace {
                        let mut inner_tokens = vec![];
                        self.loop_next_and_convert(false, &mut inner_tokens, group.stream())?;

                        // println!("inner_tokens: {:?}", inner_tokens);
                        output.push(RustHtmlToken::GroupParsed(delimiter, inner_tokens));
                    } else {
                        output.push(RustHtmlToken::Group(delimiter, group));
                    }
                }
            },
            _ => { 
                panic!("Could not handle token: {:?}", token);
                //return Err(RustHtmlError::from_string(format!("Could not handle token: {:?}", token)));
            }
        }
        Ok(false) // continue
    }

    pub fn get_opening_delim(self: &Self, delimiter: Delimiter) -> &'static str {
        match delimiter {
            Delimiter::Brace => "{",
            Delimiter::Bracket => "[",
            Delimiter::Parenthesis => "(",
            Delimiter::None => "",
            _ => panic!("Unknown delimeter {:?}", delimiter),
        }
    }

    pub fn get_closing_delim(self: &Self, delimiter: Delimiter) -> &'static str {
        match delimiter {
            Delimiter::Brace => "}",
            Delimiter::Bracket => "]",
            Delimiter::Parenthesis => ")",
            Delimiter::None => "",
            _ => panic!("Unknown delimeter {:?}", delimiter),
        }
    }

    pub fn convert_html_to_rusthtmltoken(self: &Self, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<bool, RustHtmlError> {
        let mut tag_name = String::new();
        let mut html_attrs: HashMap<String, Option<RustHtmlToken>> = HashMap::new();
        let mut html_attr_key = String::new();
        let mut html_attr_val = String::new();
        let mut parse_attrs = false;
        let mut parse_attr_val = false;
        let mut is_self_contained_tag = false;
        let mut is_opening_tag = true;

        fn on_kvp_defined(
            html_attr_key: &mut String,
            html_attr_val: &mut String,
            parse_attr_val: &mut bool,
            html_attrs: &mut HashMap<String, Option<RustHtmlToken>>) {

            // println!("{}={}", html_attr_key, html_attr_val);
            if html_attr_val.len() > 0 {
                html_attrs.insert(html_attr_key.clone(), Some(RustHtmlToken::Literal(Literal::string(&html_attr_val.clone()))));
            } else {
                html_attrs.insert(html_attr_key.clone(), None);
            }
            
            *parse_attr_val = false;
            *html_attr_key = String::new();
            *html_attr_val = String::new();
        }

        // let first_token_option = it.next();
        // if let Some(first_token) = first_token_option {    
        let mut prev_c = '<';
        loop {
            let token_option = it.next();
            match token_option {
                Some(token) => {
                    // println!("convert_html_to_rusthtmltoken (parse_attrs = {}): {:?}", parse_attrs, token);
                    match token {
                        TokenTree::Ident(ident) => {
                            if parse_attrs {
                                if parse_attr_val {
                                    html_attr_val.push_str(&ident.to_string());
                                    parse_attr_val = false;
                                } else {
                                    html_attr_key.push_str(&ident.to_string());
                                }
                            } else {
                                tag_name.push_str(&ident.to_string());
                                parse_attrs = true;
                            }
                        },
                        TokenTree::Literal(literal) => {
                            if parse_attrs {
                                if parse_attr_val {
                                    html_attr_val.push_str(&literal.to_string());
                                    on_kvp_defined(&mut html_attr_key, &mut html_attr_val, &mut parse_attr_val, &mut html_attrs);
                                } else {
                                    html_attr_key.push_str(&literal.to_string());
                                    parse_attr_val = true;
                                }
                            } else {
                                tag_name.push_str(&literal.to_string());
                                parse_attrs = true;
                            }
                        },
                        TokenTree::Punct(punct) => {
                            let c = punct.as_char();
                            if parse_attrs {
                                match c {
                                    '>' => {
                                        // println!("end tag body");
                                        break;
                                    },
                                    '=' => {
                                        parse_attr_val = true;
                                    },
                                    '/' => {
                                        let expect_closing_punct = it.next().unwrap();
                                        match expect_closing_punct {
                                            TokenTree::Punct(closing_punct) => {
                                                if closing_punct.as_char() == '>' {
                                                    is_self_contained_tag = true;
                                                    break;
                                                } else {
                                                    panic!("Unexpected character '{}' (expected '>', prev: '{}')", closing_punct, c)
                                                }
                                            },
                                            _ => panic!("Unexpected token after /: {}", c),
                                        }
                                        // // must peek next char to make sure this ends the tag,
                                        // // otherwise it gets picked up during </a> and attributes
                                        // if peek_c == '>' {
                                        //     println!("is_self_contained_tag = true");
                                        //     is_self_contained_tag = true;
                                        // } else if prev_c != '<' {
                                        //     panic!("Unexpected character '{}' (expected '>', prev_c: '{}')", peek_c, prev_c)
                                        // }
                                    },
                                    '"' => {
                                        if html_attr_key.len() > 0 {
                                            parse_attr_val = true;
                                        } else if html_attr_val.len() > 0 {
                                            on_kvp_defined(&mut html_attr_key, &mut html_attr_val, &mut parse_attr_val, &mut html_attrs);
                                            // let kvp = self.parse_html_attr(data, parse_ctx)?;
                                            // if kvp.1.len() > 0 {
                                            //     html_attrs.insert(kvp.0, Some(kvp.1));
                                            // } else {
                                            //     html_attrs.insert(kvp.0, None);
                                            // }
                                        }
                                    },
                                    '-' => {
                                        if parse_attr_val {
                                            html_attr_val.push_str(format!("{}", c).as_str());
                                        } else {
                                            html_attr_key.push_str(format!("{}", c).as_str());
                                        }
                                    },
                                    _ => {
                                        panic!("Unexpected punct {}", c);
                                    }
                                }
                            } else {
                                match c {
                                    '>' => {
                                        break;
                                    },
                                    '/' => {
                                        if tag_name.len() > 0 {
                                            let expect_closing_punct = it.next().unwrap();
                                            match expect_closing_punct {
                                                TokenTree::Punct(closing_punct) => {
                                                    if closing_punct.as_char() == '>' {
                                                        break;
                                                    } else {
                                                        panic!("Unexpected character '{}' (expected '>', prev: '{}')", closing_punct, c)
                                                    }
                                                },
                                                _ => panic!("Unexpected token after /: {:?}", expect_closing_punct),
                                            }
                                        } else {
                                            is_opening_tag = false;
                                        }
                                    },
                                    _ => {
                                        tag_name.push(c);
                                    }
                                }
                            }
                            prev_c = c;
                        },
                        _ => panic!("Unexpected token: {}", token),
                    }
                },
                _ => panic!("Unexpected token"),
            }
        }

        if html_attr_key.len() > 0 {
            on_kvp_defined(&mut html_attr_key, &mut html_attr_val, &mut parse_attr_val, &mut html_attrs);
        }
        // } else {
        //     return Ok(true);
        // }

        match tag_name.as_str() {
            "input" => {
                is_self_contained_tag = true;
            },
            _ => {}
        }

        // println!("tag: {}, attrs: {:?}", tag_name, html_attrs);
        if is_opening_tag {
            output.push(RustHtmlToken::HtmlTagStart {
                tag: tag_name,
                attributes: html_attrs,
                is_self_contained_tag: is_self_contained_tag
            });
        } else {
            output.push(RustHtmlToken::HtmlTagEnd(tag_name));
        }
        
        Ok(false)
    }

    pub fn convert_rusthtml_directive_to_rusthtmltoken(self: &Self, token: TokenTree, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<bool, RustHtmlError>  {
        // println!("convert_rusthtml_directive_to_rusthtmltoken: {:?}", token);
        match token {
            TokenTree::Ident(ident) => {
                self.convert_rusthtml_directive_identifier_to_rusthtmltoken(ident, output, it);
            },
            TokenTree::Group(group) => {
                self.convert_rusthtml_directive_group_to_rusthtmltoken(group, output, it);
            },
            _ => {
                return Err(RustHtmlError::from_string(format!("unexpected directive ident: {:?}", token)));
                //panic!("{}", format!("unexpected directive ident: '{:?}'", token));
            }
        }
        Ok(true)
    }

    pub fn convert_rusthtml_directive_group_to_rusthtmltoken(self: &Self, group: Group, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<(), RustHtmlError> {
        let mut inner_tokens = vec![];
        self.loop_next_and_convert(false, &mut inner_tokens, group.stream())?;
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
                _ => panic!("unexpected delimiter: {:?}", delimiter)
            }
        }
        Ok(())
    }

    pub fn convert_rusthtml_directive_identifier_to_rusthtmltoken(self: &Self, identifier: Ident, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<(), RustHtmlError> {
        // println!("convert_rusthtml_directive_identifier_to_rusthtmltoken: {}", identifier);
        match identifier.to_string().as_str() {
            "model" => {
                // expecting type identifier
                let type_ident = self.parse_type_identifier(it);
                self.model_type.replace(Some(type_ident));
            },
            "for" => {
                panic!("for not implemented");
            },
            "if" => {
                panic!("if not implemented");
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
                        // unexpected token
                        panic!("{}", format!("unexpected token after functions directive: {:?}", group_token));
                    }
                }
            },
            "name" => {
                // expecting string
                let expect_string_token = it.next().unwrap();
                match expect_string_token {
                    TokenTree::Literal(literal) => {
                        let param_value = literal.to_string();
                        self.params.borrow_mut().insert(identifier.to_string().clone(), param_value.clone());
                        // panic!("{}", format!("yummy directive ident: {:?} = {}", identifier, param_value));
                    },
                    _ => {
                        panic!("{}", format!("unexpected token after {} directive: {:?}", identifier, expect_string_token));
                    }
                }
            },
            "RenderSection" | "RenderSectionOptional" | "RenderBody" |
            "get_ViewData" => {
                self.insert_self_dot(output);
                self.parse_identifier_expression(identifier, output, it)?;
                output.push(RustHtmlToken::ReservedChar(';', Punct::new(';', Spacing::Alone)));
            },
            "ViewData" | "ViewPath" => {
                let mut inner_tokens = vec![];
                self.insert_self_dot(&mut inner_tokens);
                self.parse_identifier_expression(identifier, &mut inner_tokens, it)?;
                output.push(RustHtmlToken::AppendToHtml(inner_tokens));
            }
            _ => {
                // resolve identifier expression, may not be special hard coded directive like ViewData...
                self.parse_identifier_expression(identifier, output, it)?;
                output.push(RustHtmlToken::ReservedChar(';', Punct::new(';', Spacing::Alone)));
            }
        }
        Ok(())
    }

    pub fn parse_identifier_expression(self: &Self, identifier: Ident, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<(), RustHtmlError> {
        output.push(RustHtmlToken::Identifier(identifier.clone()));
        loop {
            let param_value_token_option = it.peek();
            if let Some(param_value_token) = param_value_token_option {
                match param_value_token {
                    TokenTree::Literal(literal) => {
                        output.push(RustHtmlToken::Literal(literal.clone()));
                        it.next();
                    },
                    TokenTree::Ident(ident) => {
                        output.push(RustHtmlToken::Identifier(ident.clone()));
                        it.next();
                    },
                    TokenTree::Group(group) => {
                        let delimeter = group.delimiter();
                        output.push(RustHtmlToken::Group(delimeter, group.clone()));
                        it.next();

                        // a function call
                        if delimeter == Delimiter::Parenthesis {
                            break;
                        }
                    },
                    // TokenTree::Punct
                    _ => {
                        break;
                    }
                }
            } else {
                break;
            }
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
                _ => self.convert_copy(token, output),
            }
        }
        Ok(())
    }

    pub fn convert_copy(self: &Self, token: TokenTree, output: &mut Vec<RustHtmlToken>) {
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
                panic!("unexpected token: {:?}", token);
            },
        }
    }

    pub fn parse_type_identifier(self: &Self, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Vec<TokenTree> {
        let mut type_parts: Vec<TokenTree> = vec![];
        loop
        {
            let ident_token = it.peek().unwrap();
            match ident_token.clone() {
                TokenTree::Ident(ident) => {
                    type_parts.push(it.next().unwrap().clone());
                    for _ in 0..2 {
                        let peek_after_ident = it.peek().unwrap().clone();
                        match peek_after_ident.clone() {
                            TokenTree::Punct(punct) => {
                                match punct.as_char() {
                                    ':' => {
                                        type_parts.push(it.next().unwrap().clone());
                                    },
                                    _ => {
                                        break;
                                    }
                                }
                            },
                            _ => {
                                break;
                            }
                        }
                    }
                },
                TokenTree::Punct(..) => break,
                _ => {
                    // unexpected token
                    panic!("{}", format!("unexpected token after model directive: {:?}", ident_token));
                    //return TokenStream::from(quote! { compile_error!("{}", format!(" '@{}': {:?}", identifier, ident_token)); });
                }
            }
        }
        // panic!("type_parts: {:?}", type_parts);
        type_parts
    }

    pub fn convert_rusthtmltoken_to_tokentree<'a>(self: &Self, token: &RustHtmlToken, output: &mut Vec<TokenTree>, it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<bool, RustHtmlError> {
        match token {
            RustHtmlToken::Identifier(ident) => output.push(TokenTree::Ident(ident.clone())),
            RustHtmlToken::Literal(literal) => output.push(TokenTree::Literal(literal.clone())),
            RustHtmlToken::ReservedChar(_, punct) => output.push(TokenTree::Punct(punct.clone())),
            RustHtmlToken::Group(delimiter, group) => output.push(TokenTree::Group(group.clone())),
            RustHtmlToken::GroupParsed(delimiter, inner_tokens) => 
                self.convert_rusthtmlgroupparsed_to_tokentree(delimiter, inner_tokens, output, it)?,
            RustHtmlToken::HtmlTagStart { tag, attributes, is_self_contained_tag } =>
                self.convert_rusthtmltagstart_to_tokentree(tag, attributes, is_self_contained_tag, output, it)?,
            RustHtmlToken::HtmlTagEnd(tag) =>
                self.convert_rusthtmltagend_to_tokentree(tag, output, it)?,
            RustHtmlToken::HtmlTextNode(text_node) => 
                self.convert_rusthtmltextnode_to_tokentree(text_node, output, it)?,
            RustHtmlToken::AppendToHtml(inner) =>
                self.convert_rusthtmlappendhtml_to_tokentree(inner, output, it)?,
            _ => { return Err(RustHtmlError::from_string(format!("Could not handle token {:?}", token))); }
        }
        Ok(false)
    }

    pub fn convert_rusthtmltagstart_to_tokentree<'a>(self: &Self, tag: &String, attributes: &HashMap<String, Option<RustHtmlToken>>, is_self_contained_tag: &bool, output: &mut Vec<TokenTree>, it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        let void_tag_slash = if *is_self_contained_tag { "/" } else { "" };
        let mut attributes_as_string = String::new();
        for attr in attributes {
            attributes_as_string.push_str(attr.0);
            if let Some(some_val) = attr.1 {
                attributes_as_string.push_str("=");
                match some_val {
                    RustHtmlToken::Literal(literal) => {
                        let literal_as_string = literal.to_string();
                        if literal_as_string.starts_with("\"\\\"") {
                            let v = &literal_as_string[2..literal_as_string.len()-3];
                            attributes_as_string.push_str(&v);
                            attributes_as_string.push('"');
                        } else {
                            let v: serde_json::Value = serde_json::from_str(&literal_as_string).unwrap();
                            println!("attributes_as_string val: {}", v);
                            attributes_as_string.push_str(&v.to_string());
                        }
                    },
                    _ => panic!("ahhhhhhhh"),
                }
            }
            attributes_as_string.push_str(" ");
        }

        if attributes_as_string.len() > 0 {
            attributes_as_string.insert(0, ' ');
            attributes_as_string.remove(attributes_as_string.len() - 1);
        }

        let tag_as_html = format!("<{}{}{}>", tag, attributes_as_string, void_tag_slash);
        output.push(TokenTree::Group(Group::new(Delimiter::None, quote! { self.append_html(#tag_as_html); })));

        Ok(())
    }

    pub fn convert_rusthtmltagend_to_tokentree<'a>(self: &Self, tag: &String, output: &mut Vec<TokenTree>, it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        let tag_as_html = format!("</{}>", tag);
        output.push(TokenTree::Group(Group::new(Delimiter::None, quote! { self.append_html(#tag_as_html); })));
        Ok(())
    }

    pub fn convert_rusthtmltextnode_to_tokentree<'a>(self: &Self, text_node: &String, output: &mut Vec<TokenTree>, it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        // combine text nodes
        let mut text_node_content = String::new();
        text_node_content.push_str(text_node);
        loop {
            let peek_token_option = it.peek();
            if let Some(peek_token) = peek_token_option {
                if let RustHtmlToken::HtmlTextNode(text) = peek_token {
                    text_node_content.push_str(&text);
                    it.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        output.push(TokenTree::Group(Group::new(Delimiter::None, quote! { self.append_html(#text_node_content); })));

        Ok(())
    }

    pub fn convert_rusthtmlgroupparsed_to_tokentree<'a>(self: &Self, delimiter: &Delimiter, inner_tokens: &Vec<RustHtmlToken>, output: &mut Vec<TokenTree>, it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        // println!("inner_tokens: {:?}", inner_tokens);
        let mut group = vec![];
        let mut inner_it = inner_tokens.iter().peekable();
        self.convert_rusthtmltokens_to_plain_rust(&mut group, &mut inner_it);
        // println!("group: {:?}", group);
        output.push(TokenTree::Group(Group::new(delimiter.clone(), TokenStream::from_iter(group.iter().cloned()))));
        
        Ok(())
    }

    pub fn convert_rusthtmlappendhtml_to_tokentree<'a>(self: &Self, inner: &Vec<RustHtmlToken>, output: &mut Vec<TokenTree>, it: &mut Peekable<impl Iterator<Item = &'a RustHtmlToken>>) -> Result<(), RustHtmlError> {
        let mut inner_tokens = vec![];
        let mut inner_it = inner.iter().peekable();
        self.convert_rusthtmltokens_to_plain_rust(&mut inner_tokens, &mut inner_it);
        let inner_tokenstream = TokenStream::from_iter(inner_tokens.iter().cloned());
        output.push(TokenTree::Group(Group::new(Delimiter::None, quote! { self.append_html(#inner_tokenstream); })));

        Ok(())
    }
}
