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

use crate::rusthtml_node::RustHtmlNode;
use crate::rusthtml_lang_part::RustHtmlLangPart;
use crate::rusthtml_token::RustHtmlToken;
use crate::rusthtml_error::RustHtmlError;

pub struct RustHtmlParser {
    lang_parts: Vec<Rc<dyn RustHtmlLangPart>>,
    lang_part_stack: RefCell<Vec<Rc<dyn RustHtmlLangPart>>>,

    punctuation_scope_stack: RefCell<Vec<char>>,
    params: RefCell<HashMap<String, String>>,
    functions_section: RefCell<Option<Group>>,
    model_type: RefCell<Option<TokenStream>>,
}
impl RustHtmlParser {
    pub fn new() -> Self {
        Self {
            lang_parts: vec![
                Rc::new(crate::rusthtml_lang_parts::rust_html::RustHtml { }),
                Rc::new(crate::rusthtml_lang_parts::directive::Directive { }),
            ],
            lang_part_stack: RefCell::new(vec![]),
            punctuation_scope_stack: RefCell::new(vec![]),
            params: RefCell::new(HashMap::new()),
            functions_section: RefCell::new(None),
            model_type: RefCell::new(None),
        }
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

    pub fn get_functions_section(self: &Self) -> Option<Group> {
        if let Some(has_functions) = self.functions_section.borrow().as_ref() {
            Some(has_functions.clone())
        } else {
            None
        }
    }

    pub fn get_model_ident(self: &Self) -> Option<TokenStream> {
        if let Some(has_model) = self.model_type.borrow().as_ref() {
            Some(has_model.clone())
        } else {
            None
        }
    }

    pub fn parse_to_tokenstream(self: &Self, input: TokenStream) -> Result<TokenStream, RustHtmlError> {
        let rusthtml_tokens = self.convert_tokenstream_to_rusthtmltokens(input)?;
        let rust_output = self.parse_rusthtmltokens_to_plain_rust(rusthtml_tokens)?;
        Ok(TokenStream::from_iter(rust_output))
    }

    pub fn parse_to_ast(self: &Self, input: TokenStream) -> Result<syn::Item, RustHtmlError> {
        let ts = self.parse_to_tokenstream(input)?;
        let ast = syn::parse2(ts).unwrap();
        Ok(ast)
    }

    pub fn convert_tokenstream_to_rusthtmltokens(self: &Self, input: TokenStream) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut rusthtml_tokens = Vec::new();
        let mut it = input.into_iter().peekable();
        loop {
            if self.next_and_convert(&mut rusthtml_tokens, it.by_ref())? {
                break;
            }
        }
        println!("done with file");
        Ok(rusthtml_tokens)
    }

    pub fn next_and_convert(self: &Self, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<bool, RustHtmlError> {
        let token_option = it.next();
        println!("next_and_convert: {:?}", token_option);
        if let Some(token) = token_option {
            if self.convert_tokentree_to_rusthtmltoken(token, output, it)? {
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
        let mut it = rusthtml_tokens.iter();
        loop 
        {
            if self.convert_rusthtmltokens_to_plain_rust(&mut rust_output, &mut it)? {
                break;
            }
        }
        Ok(rust_output)
    }

    pub fn convert_rusthtmltokens_to_plain_rust(self: &Self, output: &mut Vec<TokenTree>, it: &mut dyn Iterator<Item = &RustHtmlToken>) -> Result<bool, RustHtmlError> { // , Option<Vec<TokenTree>>)
        let html_ident = Ident::new("html", Span::call_site());

        let mut should_break_outer_loop = false;
        let mut inner_statements = vec![];
        loop 
        {
            let mut inner_statement_rust = vec![];
            let token_option = it.next();
            if let Some(token) = token_option {
                let break_loop = self.convert_rusthtmltoken_to_tokentree(token, &mut inner_statement_rust, it)?;
                for inner_statement_token in inner_statement_rust {
                    let append_html = quote! { #html_ident.push_str(#inner_statement_token); };
                    inner_statements.push(TokenTree::Group(Group::new(Delimiter::None, append_html)));
                }
                if break_loop {
                    break;
                }
            } else {
                should_break_outer_loop = true;
                break;
            }
        }

        let inner_rust = TokenTree::Group(Group::new(Delimiter::None, TokenStream::from_iter(inner_statements.into_iter())));
        let init_html_statement = quote! { let mut #html_ident = String::new(); #inner_rust HtmlString::new_from_html(#html_ident) };
        output.push(TokenTree::Group(Group::new(Delimiter::Brace, init_html_statement)));
        Ok(should_break_outer_loop)
    }

    pub fn convert_tokentree_to_rusthtmltoken(self: &Self, token: TokenTree, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<bool, RustHtmlError> {
        match token {
            TokenTree::Ident(ident) => {
                println!("Literal: {}", ident.to_string());
                output.push(RustHtmlToken::Literal(Literal::string(&ident.to_string())));
            },
            TokenTree::Literal(literal) => {
                println!("Literal: {}", literal.to_string());
                output.push(RustHtmlToken::Literal(literal.clone()));
            },
            TokenTree::Punct(punct) => {
                let c = punct.as_char();
                match c {
                    '@' => {
                        println!("processing directive");
                        // directive, not included in output
                        let directive_ident = it.next().unwrap();
                        self.convert_rusthtml_directive_to_rusthtmltoken(directive_ident, output, it)?;
                    },
                    '<' => {
                        if self.convert_html_to_rusthtmltoken(output, it)? {
                            println!("Do not continue after tag");
                            return Ok(true); // do not continue
                        }
                        println!("Continue after tag");
                    },
                    _ => {
                        output.push(RustHtmlToken::Literal(Literal::string(&punct.as_char().to_string())));
                        println!("beep: '{}'", punct.as_char());
                    },
                }
            },
            TokenTree::Group(group) => {
                println!("group");
                output.push(RustHtmlToken::Group(group.delimiter(), group.clone()));
            },
            _ => { 
                panic!("Could not handle token: {:?}", token);
                //return Err(RustHtmlError::from_string(format!("Could not handle token: {:?}", token)));
            }
        }
        Ok(false) // continue
    }

    pub fn convert_html_to_rusthtmltoken(self: &Self, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<bool, RustHtmlError> {
        let mut tag_name = String::new();
        let mut html_attrs: HashMap<String, Option<RustHtmlToken>> = HashMap::new();
        let mut html_attr_key = String::new();
        let mut html_attr_val = String::new();
        let mut parse_attrs = false;
        let mut parse_attr_val = false;
        let mut is_self_contained_tag = false;
        let mut is_opening_tag = false;

        let first_token_option = it.next();
        if let Some(first_token) = first_token_option {    
            let mut prev_c = '<';
            loop {
                let token_option = it.next();
                match token_option {
                    Some(token) => {
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
                                    tag_name = ident.to_string();
                                    parse_attrs = true;
                                }
                            },
                            TokenTree::Literal(literal) => {
                                if parse_attrs {
                                    if parse_attr_val {
                                        html_attr_val.push_str(&literal.to_string());
                                        parse_attr_val = false;
                                    } else {
                                        html_attr_key.push_str(&literal.to_string());
                                    }
                                } else {
                                    tag_name = literal.to_string();
                                    parse_attrs = true;
                                }
                            },
                            TokenTree::Punct(punct) => {
                                let c = punct.as_char();
                                if parse_attrs {
                                    match c {
                                        '>' => {
                                            println!("end tag body");
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
                                                        break;
                                                    } else {
                                                        panic!("Unexpected character '{}' (expected '>', prev: '{}')", closing_punct, c)
                                                    }
                                                },
                                                _ => panic!("Unexpected token after /"),
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
                                            } else if parse_attr_val {
                                                parse_attr_val = false;

                                                println!("{}={}", html_attr_key, html_attr_val);
                                                if html_attr_val.len() > 0 {
                                                    html_attrs.insert(html_attr_key, Some(RustHtmlToken::Literal(Literal::string(&html_attr_val))));
                                                } else {
                                                    html_attrs.insert(html_attr_key, None);
                                                }
                                                
                                                html_attr_key = String::new();
                                                html_attr_val = String::new();
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
                                        ' ' => {},
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
                                            let expect_closing_punct = it.next().unwrap();
                                            match expect_closing_punct {
                                                TokenTree::Punct(closing_punct) => {
                                                    if closing_punct.as_char() == '>' {
                                                        break;
                                                    } else {
                                                        panic!("Unexpected character '{}' (expected '>', prev: '{}')", closing_punct, c)
                                                    }
                                                },
                                                _ => panic!("Unexpected token after /"),
                                            }
                                        },
                                        ' ' => {
                                            parse_attrs = true;
                                        }
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
        } else {
            return Ok(true);
        }

        match tag_name.as_str() {
            "input" => {
                is_self_contained_tag = true;
            },
            _ => {}
        }

        println!("tag: {}", tag_name);

        output.push(RustHtmlToken::HtmlTagStart {
            tag: tag_name,
            attributes: html_attrs,
            is_self_contained_tag: is_self_contained_tag
        });
        
        Ok(false)
    }

    pub fn convert_rusthtml_directive_to_rusthtmltoken(self: &Self, token: TokenTree, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<bool, RustHtmlError>  {
        match token {
            TokenTree::Ident(ident) => {
                self.convert_rusthtml_directive_identifier_to_rusthtmltoken(ident.to_string(), output, it);
            },
            TokenTree::Group(group) => {
                self.convert_rusthtml_directive_group_to_rusthtmltoken(group, it);
            },
            _ => {
                return Err(RustHtmlError::from_string(format!("unexpected directive ident: {:?}", token)));
                //panic!("{}", format!("unexpected directive ident: '{:?}'", token));
            }
        }
        Ok(true)
    }

    pub fn convert_rusthtml_directive_group_to_rusthtmltoken(self: &Self, group: Group, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> () {
    }

    pub fn convert_rusthtml_directive_identifier_to_rusthtmltoken(self: &Self, identifier: String, output: &mut Vec<RustHtmlToken>, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<(), RustHtmlError> {
        println!("convert_rusthtml_directive_identifier_to_rusthtmltoken: {}", identifier);
        match identifier.as_str() {
            "for" => {
                panic!("for not implemented");
            },
            "model" => {
                // expecting type identifier
                let type_ident = self.extract_type_identifier(it);
                self.model_type.replace(Some(type_ident));
            },
            "let" => {
                output.push(RustHtmlToken::Group(Delimiter::None, Group::new(Delimiter::None, self.parse_let(it)?)));
            },
            "functions" => {
                // expecting group
                let group_token = it.next().unwrap();
                match group_token {
                    TokenTree::Group(group) => {
                        self.functions_section.replace(Some(group));
                    },
                    _ => {
                        // unexpected token
                        panic!("{}", format!("unexpected token after directive '@{}': {:?}", identifier, group_token));
                    }
                }
            },
            _ => {
                let param_value_token = it.next().unwrap();
                match param_value_token {
                    TokenTree::Literal(literal) => {
                        let param_value = literal.to_string();
                        self.params.borrow_mut().insert(identifier.clone(), param_value.clone());
                        // panic!("{}", format!("yummy directive ident: {:?} = {}", identifier, param_value));
                    },
                    _ => {
                        // unexpected token
                        panic!("{}", format!("unexpected token after directive '@{}': {:?}", identifier, param_value_token));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn parse_let(self: &Self, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<TokenStream, RustHtmlError> {
        let mut tokens: Vec<TokenTree> = vec![];
        loop
        {
            let token = it.next().unwrap();
            match token.clone() {
                TokenTree::Punct(punct) => {
                    if punct.as_char() == ';' {
                        break;
                    } else {
                        tokens.push(token.clone());
                    }
                },
                _ => tokens.push(token.clone()),
            }
        }
        Ok(TokenStream::from_iter(tokens.into_iter()))
    }

    pub fn extract_type_identifier(self: &Self, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> TokenStream {
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
                    panic!("{}", format!("unexpected token after directive '@model': {:?}", ident_token));
                    //return TokenStream::from(quote! { compile_error!("{}", format!("unexpected token after directive '@{}': {:?}", identifier, ident_token)); });
                }
            }
        }
        // panic!("type_parts: {:?}", type_parts);
        TokenStream::from_iter(type_parts)
    }

    pub fn convert_rusthtmltoken_to_tokentree(self: &Self, token: &RustHtmlToken, output: &mut Vec<TokenTree>, it: &mut dyn Iterator<Item = &RustHtmlToken>) -> Result<bool, RustHtmlError> {
        match token {
            RustHtmlToken::Identifier(ident) => output.push(TokenTree::Ident(ident.clone())),
            RustHtmlToken::Literal(literal) => output.push(TokenTree::Literal(literal.clone())),
            RustHtmlToken::ReservedChar(_, punct) => output.push(TokenTree::Punct(punct.clone())),
            RustHtmlToken::Group(_, group) => output.push(TokenTree::Group(group.clone())),
            RustHtmlToken::HtmlTagStart { tag, attributes, is_self_contained_tag } => {
                let void_tag_slash = if *is_self_contained_tag { "/" } else { "" };
                let attributes_as_string = "";
                let tag_as_html = format!("<{} {} {}>", tag, attributes_as_string, void_tag_slash);
                output.push(TokenTree::Literal(Literal::string(tag_as_html.as_str())));
            },
            _ => { return Err(RustHtmlError::from_string(format!("Could not handle token {:?}", token))); }
        }
        Ok(true)
    }
}
