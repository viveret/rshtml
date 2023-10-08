// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use core_macro_lib::nameof_member_fn;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, TokenStream, TokenTree};

use crate::view::rusthtml::rusthtml_token::{RustHtmlToken, RustHtmlIdentAndPunctAndGroupOrLiteral, RustHtmlIdentOrPunctOrGroup };
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use super::html_tag_parse_context::HtmlTagParseContext;
use super::ihtml_tag_parse_context::IHtmlTagParseContext;
use super::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
use super::peekable_tokentree::{IPeekableTokenTree, PeekableTokenTree};
use super::rusthtml_directive_result::RustHtmlDirectiveResult;
use super::irusthtml_parser_context::IRustHtmlParserContext;


// this implements the IRustToRustHtml trait.
#[derive(Clone)]
pub struct RustToRustHtmlConverter {
    // the context for the RustHtml parser.
    pub context: Rc<dyn IRustHtmlParserContext>,
}

impl RustToRustHtmlConverter {
    // create a new instance of the RustToRustHtml parser.
    // context: the context for the RustHtml parser.
    pub fn new(context: Rc<dyn IRustHtmlParserContext>) -> Self {
        Self {
            context: context,
        }
    }

    fn peek_reserved_chars_in_str(self: &Self, arg: &'static str, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<bool, RustHtmlError> {
        for c in arg.chars() {
            if !self.peek_reserved_char(c, output, it.clone(), is_raw_tokenstream)? {
                return Ok(false);
            }
        }
    
        Ok(true)
    }

    pub fn peek_reserved_char(self: &Self, expected_char: char, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, _is_raw_tokenstream: bool) -> Result<bool, RustHtmlError> {
        if let Some(next_token) = it.peek() {
            match next_token {
                TokenTree::Punct(next_punct) => {
                    if next_punct.as_char() == expected_char {
                        // this is the expected char, so consume it
                        it.next();
                        output.push(RustHtmlToken::ReservedChar(next_punct.as_char(), next_punct));
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                },
                _ => {
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }
}

impl IRustToRustHtmlConverter for RustToRustHtmlConverter {
    // parse a token stream to RustHtml tokens.
    // is_in_html_mode: whether we are in HTML mode or not.
    // it: the token stream to parse.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: the RustHtml tokens.
     fn parse_tokenstream_to_rusthtmltokens(self: &Self, is_in_html_mode: bool, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut rusthtml_tokens = Vec::new();
        self.loop_next_and_convert(is_in_html_mode, &mut rusthtml_tokens, it, is_raw_tokenstream)?;
        Ok(rusthtml_tokens)
    }

    // loop through the token stream and convert it to RustHtml tokens.
    // is_in_html_mode: whether we are in HTML mode or not.
    // output: the destination for the RustHtml tokens.
    // it: the token stream to parse.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: nothing or error.
    fn loop_next_and_convert(self: &Self, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError> {
        loop {
            if self.next_and_convert(is_in_html_mode, output, it.clone(), is_raw_tokenstream)? {
                break;
            }
        }
        Ok(())
    }

    // iterate the iterator by one step (next) and convert a token tree to RustHtml tokens.
    // is_in_html_mode: whether we are in HTML mode or not.
    // output: the destination for the RustHtml tokens.
    // it: the token stream to parse.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: whether we should break the outer loop or not, or an error.
    fn next_and_convert(self: &Self, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<bool, RustHtmlError> {
        let token_option = it.next();

        if let Some(token) = token_option {
            if self.convert_tokentree_to_rusthtmltoken(token, is_in_html_mode, output, it, is_raw_tokenstream)? {
                return Ok(true); // break outer loop
            }
        } else {
            return Ok(true); // break outer loop
        }

        Ok(false)
    }
    
    // converts a tokentree to a RustHtml token.
    // token: the token to convert.
    // is_in_html_mode: whether we are in HTML mode or not.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: whether we should break the outer loop or not, or an error.
    fn convert_tokentree_to_rusthtmltoken(self: &Self, token: TokenTree, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<bool, RustHtmlError> {
        match token.clone() {
            TokenTree::Ident(ident) => {
                if is_in_html_mode {
                    self.context.add_operation_to_ooo_log(format!("convert_tokentree_to_rusthtmltoken: {:?}", ident));
                    output.push(RustHtmlToken::HtmlTextNode(ident.to_string(), token.span().clone()));
                } else {
                    output.push(RustHtmlToken::Identifier(ident));
                }
            },
            TokenTree::Literal(literal) => {
                self.context.add_operation_to_ooo_log(format!("convert_tokentree_to_rusthtmltoken({:?})", literal));
                if is_in_html_mode {
                    output.push(RustHtmlToken::HtmlTextNode(literal.to_string(), token.span().clone()));
                } else {
                    output.push(RustHtmlToken::Literal(Some(literal), None));
                }
            },
            TokenTree::Punct(punct) => {
                if self.convert_punct_to_rusthtmltoken(punct, is_in_html_mode, output, it, is_raw_tokenstream)? {
                    return Ok(true);
                }
            },
            TokenTree::Group(group) => {
                self.convert_group_to_rusthtmltoken(group, false, is_in_html_mode, output, is_raw_tokenstream)?;
            },
        }
        Ok(false) // continue
    }
    
    // convert a Rust punctuation to a RustHtml token.
    // punct: the punctuation to convert.
    // is_in_html_mode: whether we are in HTML mode or not.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: whether we should break the outer loop or not, or an error.
    fn convert_punct_to_rusthtmltoken(self: &Self, punct: Punct, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<bool, RustHtmlError> {
        let c = punct.as_char();
        self.context.add_operation_to_ooo_log(format!("convert_punct_to_rusthtmltoken: {}", c));
        match c {
            '@' => {
                self.convert_rust_entry_to_rusthtmltoken(c, punct, is_in_html_mode, output, it, is_raw_tokenstream)?;
            },
            '<' => {
                self.convert_html_entry_to_rusthtmltoken(c, punct, true, output, it, is_raw_tokenstream)?;
            },
            '}' if !is_in_html_mode => {
                return Ok(true); // do not continue
            },
            '>' if !is_in_html_mode => {
                output.push(RustHtmlToken::ReservedChar(c, punct));
            },
            '|' if !is_in_html_mode => {
                output.push(RustHtmlToken::ReservedChar(c, punct));

                // peek ahead to see if this is a || -> or something else
                if self.peek_reserved_chars_in_str("|->", output, it.clone(), is_raw_tokenstream)? {
                    // peek for HtmlString identifier that signals the function will return HtmlString
                    if let Some(next_token) = it.peek() {
                        match next_token {
                            TokenTree::Ident(next_ident) => {
                                if next_ident.to_string() == "HtmlString" {
                                    // this is a function that returns HtmlString
                                    it.next();
                                    output.push(RustHtmlToken::Identifier(next_ident));

                                    // parse the rest of the function, which should be in a {} group
                                    if let Some(group_token) = it.next() {
                                        match group_token {
                                            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                                                self.convert_group_to_rusthtmltoken(group, true, is_in_html_mode, output, is_raw_tokenstream)?;
                                                return Ok(false);
                                            },
                                            _ => {
                                                return Err(RustHtmlError::from_string(format!("Expected {{ after |->")));
                                            }
                                        }
                                    } else {
                                        return Err(RustHtmlError::from_string(format!("Expected {{ after |->")));
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }
            },
            _ => {
                if is_in_html_mode {
                    output.push(RustHtmlToken::HtmlTextNode(punct.as_char().to_string(), punct.span().clone()));
                } else {
                    output.push(RustHtmlToken::ReservedChar(c, punct.clone()));
                }
            },
        }

        Ok(false)
    }

    // convert a Rust entry to a RustHtml token.
    // punct: the punctuation to convert.
    // is_in_html_mode: whether we are in HTML mode or not.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: nothing or an error.
    fn convert_rust_entry_to_rusthtmltoken(self: &Self, _c: char, _punct: Punct, _is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError> {
        if let Some(directive_token) = it.next() {
            self.convert_rust_directive_to_rusthtmltoken(directive_token, None, output, it, is_raw_tokenstream)?;
        }
        Ok(())
    }

    // convert a Rust HTML entry to a RustHtml token.
    // punct: the punctuation to convert.
    // is_in_html_mode: whether we are in HTML mode or not.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: nothing or an error.
    fn convert_html_entry_to_rusthtmltoken(self: &Self, c: char, punct: Punct, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError> {
        if is_in_html_mode || self.is_start_of_current_expression(output) {
            // the below context is orphaned by not passing the parent html tag parse context.
            // this is usually fine. but we need to pass the main context to call add_operation_to_ooo_log
            let ctx = Rc::new(HtmlTagParseContext::new(Some(self.context.clone())));
            let mut output_inner = vec![];
            // it.enable_log_next("convert_html_entry_to_rusthtmltoken");
            loop {
                let token_option = it.next();
                if let Some(token) = token_option {
                    if self.next_and_parse_html_tag(&token, ctx.clone(), &mut output_inner, it.clone(), is_raw_tokenstream)? {
                        // println!("convert_html_entry_to_rusthtmltoken: breaking on {:?}", token);
                        break;
                    }
                } else {
                    break;
                }
            }
            // it.disable_log_next();

            let mut add_inner = true;
            if ctx.is_opening_tag() && !ctx.is_void_tag() && !ctx.is_self_contained_tag() {
                // parse inner elements / code until we find closing tag
                self.context.htmltag_scope_stack_push(ctx.tag_name_as_str());
                loop {
                    if self.next_and_convert(true, &mut output_inner, it.clone(), is_raw_tokenstream)? {
                        break;
                    }
                    match output_inner.last().unwrap() {
                        RustHtmlToken::HtmlTagEnd(tag_end, _tag_end_tokens) => {
                            if tag_end == &ctx.tag_name_as_str() {
                                break;
                            }
                        },
                        _ => {
                        }
                    }
                }
                let last_scope_from_stack = self.context.htmltag_scope_stack_pop().unwrap();
                if last_scope_from_stack != ctx.tag_name_as_str() {
                    return Err(RustHtmlError::from_string(format!("Mismatched HTML tags (found {} but expected {})", last_scope_from_stack, ctx.tag_name_as_str())));
                }

                if let Some(output_inner_last) = output_inner.last() {
                    if let RustHtmlToken::HtmlTagEnd(_tag_end, _tag_end_tokens) = output_inner_last {
                        add_inner = self.on_html_node_parsed(ctx, &mut output_inner)?;
                    }
                }
            }

            if add_inner {
                output.extend_from_slice(&output_inner);
            }
        } else {
            output.push(RustHtmlToken::ReservedChar(c, punct));
        }

        Ok(())
    }

    // convert a Rust group to a RustHtml token.
    // group: the group to convert.
    // is_in_html_mode: whether we are in HTML mode or not.
    // output: the destination for the RustHtml tokens.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: nothing or an error.
    fn convert_group_to_rusthtmltoken(self: &Self, group: Group, expect_return_html: bool, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError> {
        let delimiter = group.delimiter();
        let it = Rc::new(PeekableTokenTree::new(group.stream()));
        if is_in_html_mode {
            self.context.add_operation_to_ooo_log(format!("convert_group_to_rusthtmltoken: {:?}", group));
            let c_start = self.get_opening_delim(delimiter);
            let c_end = self.get_closing_delim(delimiter);

            output.push(RustHtmlToken::HtmlTextNode(c_start.to_string(), group.span()));
            self.loop_next_and_convert(true, output, it, is_raw_tokenstream)?;
            output.push(RustHtmlToken::HtmlTextNode(c_end.to_string(), group.span()));
        } else {
            if delimiter == Delimiter::Brace {
                let mut inner_tokens = vec![];
                
                // prefix and postfix with html_output decorators
                if expect_return_html {
                    self.loop_next_and_convert(is_in_html_mode, &mut inner_tokens, Rc::new(PeekableTokenTree::new(quote::quote! { let html_output = HtmlBuffer::new(); }.into())), false)?;
                }
                
                self.loop_next_and_convert(false, &mut inner_tokens, it, is_raw_tokenstream)?;
                
                if expect_return_html {
                    self.loop_next_and_convert(is_in_html_mode, &mut inner_tokens, Rc::new(PeekableTokenTree::new(quote::quote! { html_output.collect_html() }.into())), false)?;
                }

                output.push(RustHtmlToken::GroupParsed(delimiter, inner_tokens));
            } else {
                output.push(RustHtmlToken::Group(delimiter, group));
            }
        }

        Ok(())
    }

    // get the delimiter as a string containing the opening delimiter.
    // delimiter: the delimiter to get the opening char for.
    // returns: the opening delimiter.
    fn get_opening_delim(self: &Self, delimiter: Delimiter) -> &'static str {
        match delimiter {
            Delimiter::Brace => "{",
            Delimiter::Bracket => "[",
            Delimiter::Parenthesis => "(",
            Delimiter::None => "",
        }
    }

    // get the delimiter as a string containing the closing delimiter.
    // delimiter: the delimiter to get the closing char for.
    // returns: the closing delimiter.
    fn get_closing_delim(self: &Self, delimiter: Delimiter) -> &'static str {
        match delimiter {
            Delimiter::Brace => "}",
            Delimiter::Bracket => "]",
            Delimiter::Parenthesis => ")",
            Delimiter::None => "",
        }
    }

    // convert a RustHtml language directive in Rust to a RustHtml token.
    // token: the token to convert.
    // prefix_token_option: the prefix token, if any.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: whether we should break the outer loop or not, or an error.
    fn convert_rust_directive_to_rusthtmltoken(self: &Self, token: TokenTree, prefix_token_option: Option<RustHtmlToken>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<bool, RustHtmlError>  {
        match token {
            TokenTree::Ident(ref ident) => {
                self.convert_rust_directive_identifier_to_rusthtmltoken(ident, &token, prefix_token_option, output, it, is_raw_tokenstream)?;
            },
            TokenTree::Group(group) => {
                self.convert_rust_directive_group_to_rusthtmltoken(group, prefix_token_option, output, is_raw_tokenstream)?;
            },
            TokenTree::Literal(literal) => {
                output.push(RustHtmlToken::AppendToHtml(vec![RustHtmlToken::Literal(Some(literal.clone()), None)]));
                // self.convert_rusthtml_literal_to_rusthtmltoken(group, output, it);
            },
            TokenTree::Punct(punct) => {
                let c = punct.as_char();
                match c {
                    '@' => {
                        // escape '@'
                        output.push(RustHtmlToken::AppendToHtml(vec![RustHtmlToken::ReservedChar(c, punct.clone())]));
                    },
                    '&' => {
                        let prefix_token = RustHtmlToken::ReservedChar(c, punct.clone());
                        
                        let next_token = it.next();
                        if let Some(token) = next_token {
                            return self.convert_rust_directive_to_rusthtmltoken(token, Some(prefix_token), output, it, is_raw_tokenstream);
                        }
                    },
                    _ => {
                        return Err(RustHtmlError::from_string(format!("unexpected directive char: {}", c)));
                    }
                }
            },
        }
        Ok(true)
    }
    
    // convert a RustHtml language directive group in Rust to a RustHtml token.
    // group: the group to convert.
    // prefix_token_option: the prefix token, if any.
    // output: the destination for the RustHtml tokens.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: nothing or an error.
    fn convert_rust_directive_group_to_rusthtmltoken(self: &Self, group: Group, _prefix_token_option: Option<RustHtmlToken>, output: &mut Vec<RustHtmlToken>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError> {
        let mut inner_tokens = vec![];
        let it = Rc::new(PeekableTokenTree::new(group.stream()));
        self.loop_next_and_convert(false, &mut inner_tokens, it, is_raw_tokenstream)?;
        if inner_tokens.len() > 0 {
            let delimiter = group.delimiter();
            match delimiter {
                Delimiter::Brace => {
                    output.extend_from_slice(&inner_tokens);
                },
                Delimiter::Parenthesis => {
                    output.push(RustHtmlToken::AppendToHtml(inner_tokens));
                },
                _ => {
                    return Err(RustHtmlError::from_string(format!("unexpected delimiter: {:?}", delimiter)));
                },
            }
        }
        Ok(())
    }

    // convert a RustHtml language directive identifier in Rust to a RustHtml token.
    // identifier: the identifier to convert.
    // prefix_token_option: the prefix token, if any.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: nothing or an error.
    fn convert_rust_directive_identifier_to_rusthtmltoken(self: &Self, identifier: &Ident, ident_token: &TokenTree, prefix_token_option: Option<RustHtmlToken>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError> {
        if let Some(directive) = self.context.try_get_directive(identifier.to_string()) {
            let r = directive.execute(&identifier, ident_token, Rc::new(self.clone()), output, it);
            match r {
                Ok(r) => {
                    match r {
                        RustHtmlDirectiveResult::OkContinue => { },
                        RustHtmlDirectiveResult::OkBreak => { },
                        RustHtmlDirectiveResult::OkBreakAppendHtml => output.push(RustHtmlToken::AppendToHtml(vec![])),
                    }
                },
                Err(RustHtmlError(e)) => {
                    return Err(RustHtmlError::from_string(format!("error executing directive: {}", e)));
                }
            }
        } else {
            let mut inner_tokens = vec![];
            if let Some(prefix_token) = prefix_token_option {
                inner_tokens.push(prefix_token);
            }
            self.parse_identifier_expression(true, identifier, ident_token, true, &mut inner_tokens, it, is_raw_tokenstream)?;
            output.push(RustHtmlToken::AppendToHtml(inner_tokens));
        }
        Ok(())
    }

    // convert a Rust identifier expression to a path string relative to the current working directory.
    // identifier: the identifier to convert.
    // it: the iterator to use.
    // returns: the path string or an error.
    fn next_path_str(self: &Self, identifier: &Ident, _identifier_token: &TokenTree, it: Rc<dyn IPeekableTokenTree>, _is_raw_tokenstream: bool) -> Result<String, RustHtmlError> {
        let mut path = std::path::PathBuf::new();
        let cwd = std::env::current_dir().unwrap();
        path.push(cwd);
        let relative_path = self.parse_string_with_quotes(false, identifier.clone(), it)?;
        path.push(relative_path.clone());

        Ok(path.to_str().unwrap().to_string())
    }

    fn peek_path_str(self: &Self, identifier: &Ident, _identifier_token: &TokenTree,  it: Rc<dyn IPeekableTokenTree>, _is_raw_tokenstream: bool) -> Result<String, RustHtmlError> {
        let mut path = std::path::PathBuf::new();
        let cwd = std::env::current_dir().unwrap();
        path.push(cwd);
        let relative_path = self.parse_string_with_quotes(true, identifier.clone(), it)?;
        path.push(relative_path.clone());

        Ok(path.to_str().unwrap().to_string())
    }

    // expand an external token stream into RustHtml tokens.
    // path: the path to the external token stream.
    // output: the destination for the RustHtml tokens.
    // returns: nothing or an error.
    fn expand_external_tokenstream(self: &Self, path: &String, output: &mut Vec<RustHtmlToken>) -> Result<(), RustHtmlError> {
        match std::fs::read_to_string(path) {
            Ok(input_str) => {
                self.expand_external_rshtml_string(&input_str, output)
            },
            Err(_e) => {
                let parent_path = std::path::Path::new(path).parent().unwrap();
                match std::fs::read_to_string(parent_path) {
                    Ok(input_str) => {
                        self.expand_external_rshtml_string(&input_str, output)
                    },
                    Err(e) => {
                        Err(RustHtmlError::from_string(format!("Cannot read {}: {}", path, e)))
                    },
                }
            },
        }
    }

    // expand an external token stream into RustHtml tokens.
    // path: the path to the external token stream.
    // output: the destination for the RustHtml tokens.
    // returns: nothing or an error.
    fn expand_external_rshtml_string(self: &Self, input_str: &String, output: &mut Vec<RustHtmlToken>) -> Result<(), RustHtmlError> {
        let input_result = TokenStream::from_str(input_str.as_str());
        
        match input_result {
            Ok(input) => {
                let peekable = Rc::new(PeekableTokenTree::new(input));
                let rusthtml_tokens = self.parse_tokenstream_to_rusthtmltokens(true, peekable, true)?;
                output.extend_from_slice(&rusthtml_tokens);
                Ok(())
            },
            Err(e) => {
                Err(RustHtmlError::from_string(format!("{}", e)))
            },
        }
    }

    // returns if the current output is the start of a new expression or not.
    // output: the destination for the RustHtml tokens.
    // returns: if the current output is the start of a new expression or not.
    fn is_start_of_current_expression(self: &Self, output: &mut Vec<RustHtmlToken>) -> bool {
        if output.len() == 0 {
            true
        } else {
            let last = output.last().unwrap();
            match last {
                RustHtmlToken::ReservedChar(c, _punct) => *c == ';',
                RustHtmlToken::Group(..) => true,
                _ => false,
            }
        }
    }

    // parse a Rust string literal with quotes.
    // identifier: the identifier to convert.
    // it: the iterator to use.
    // returns: the string or an error.
    fn parse_string_with_quotes(self: &Self, peek_or_next: bool, identifier: Ident, it: Rc<dyn IPeekableTokenTree>) -> Result<String, RustHtmlError> {
        let r = if peek_or_next { it.peek() } else { it.next() };
        if let Some(expect_string_token) = r {
            match expect_string_token {
                TokenTree::Literal(literal) => Ok(snailquote::unescape(&literal.to_string()).unwrap()),
                _ => Err(RustHtmlError::from_string(format!("unexpected token after {} directive: {:?}", identifier, expect_string_token))),
            }
        } else {
            Err(RustHtmlError::from_string(format!("unexpected end of token stream after {} directive", identifier)))
        }
    }

    // parse Rust identifier expression and convert it to RustHtml tokens.
    // identifier: the identifier to convert.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn parse_identifier_expression(self: &Self, add_first_ident: bool, _identifier: &Ident, identifier_token: &TokenTree, last_token_was_ident: bool, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError> {
        match self.extract_identifier_expression(add_first_ident, identifier_token, last_token_was_ident, it, is_raw_tokenstream) {
            Ok(tokens) => {
                for token in tokens.iter() {
                    match token {
                        TokenTree::Literal(literal) => {
                            output.push(RustHtmlToken::Literal(Some(literal.clone()), None));
                        },
                        TokenTree::Ident(ident) => {
                            output.push(RustHtmlToken::Identifier(ident.clone()));
                        },
                        TokenTree::Group(group) => {
                            let delimiter = group.delimiter();
                            let mut inner_tokens = vec![];
                            self.loop_next_and_convert(false, &mut inner_tokens, Rc::new(PeekableTokenTree::new(group.stream())), is_raw_tokenstream)?;
                            output.push(RustHtmlToken::GroupParsed(delimiter, inner_tokens));
                        },
                        TokenTree::Punct(punct) => {
                            output.push(RustHtmlToken::ReservedChar(punct.as_char(), punct.clone()));
                        },
                    }
                }
                Ok(())
            },
            Err(e) => {
                Err(RustHtmlError::from_string(format!("error parsing identifier expression: {}", e)))
            }
        }
    }

    // get the next token and parse it as a literal or identifier expression that can be converted to RustHtml tokens.
    // identifier: the identifier to convert.
    // it: the iterator to use.
    // returns: the converted tokens or an error.
    fn convert_string_or_ident(self: &Self, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<RustHtmlIdentAndPunctAndGroupOrLiteral, RustHtmlError> {
        if let Some(expect_string_or_ident_token) = it.next() {
            match expect_string_or_ident_token {
                TokenTree::Literal(literal) => {
                    Ok(RustHtmlIdentAndPunctAndGroupOrLiteral::Literal(literal.clone()))
                },
                TokenTree::Ident(ref ident2) => {
                    let mut inner_tokens = vec![];
                    self.parse_identifier_expression(true, ident2, &expect_string_or_ident_token, true, &mut inner_tokens, it, is_raw_tokenstream)?;
                    Ok(RustHtmlIdentAndPunctAndGroupOrLiteral::IdentAndPunctAndGroup(self.convert_rusthtmltokens_to_ident_or_punct_or_group(inner_tokens)?))
                },
                _ => {
                    Err(RustHtmlError::from_string(format!("convert_string_or_ident did not find string or ident")))
                }
            }
        } else {
            Err(RustHtmlError::from_string(format!("convert_string_or_ident did not find string or ident")))
        }
    }

    // convert RustHtml tokens to a RustHtml identifier or punct or group.
    // tokens: the tokens to convert.
    // returns: the converted tokens or an error.
    fn convert_rusthtmltokens_to_ident_or_punct_or_group(self: &Self, tokens: Vec<RustHtmlToken>) -> Result<Vec<RustHtmlIdentOrPunctOrGroup>, RustHtmlError> {
        if tokens.len() == 0 {
            return Err(RustHtmlError::from_string(format!("tokens was empty")));
        }

        Ok(tokens.iter()
            .map(|x| match x {
                RustHtmlToken::Identifier(ident) => RustHtmlIdentOrPunctOrGroup::Ident(ident.clone()),
                RustHtmlToken::ReservedChar(_, punct) => RustHtmlIdentOrPunctOrGroup::Punct(punct.clone()),
                RustHtmlToken::Group(_, group) => RustHtmlIdentOrPunctOrGroup::Group(group.clone()),
                RustHtmlToken::GroupParsed(delimiter, tokens) => RustHtmlIdentOrPunctOrGroup::Group(Group::new(delimiter.clone(), tokens.iter().map(|x| match x {
                    RustHtmlToken::Identifier(ident) => TokenTree::Ident(ident.clone()),
                    RustHtmlToken::ReservedChar(_, punct) => TokenTree::Punct(punct.clone()),
                    RustHtmlToken::Group(_, group) => TokenTree::Group(group.clone()),
                    _ => panic!("convert_rusthtmltokens_to_ident_or_punct_or_group Unexpected token {:?}", x),
                }).collect())),
                _ => panic!("convert_rusthtmltokens_to_ident_or_punct_or_group Unexpected token {:?}", x),
            })
            .collect())
    }

    // iterate the iterator by one step (next) and convert a token tree to RustHtml tokens in the context of a HTML tag.
    // token_option: the token to convert.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: whether we should break the outer loop or not, or an error.
    fn next_and_parse_html_tag(
        self: &Self,
        token: &TokenTree,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        output: &mut Vec<RustHtmlToken>,
        it: Rc<dyn IPeekableTokenTree>,
        is_raw_tokenstream: bool,
    ) -> Result<bool, RustHtmlError> {
        match token {
            TokenTree::Ident(ident) => {
                // println!("next_and_parse_html_tag: {:?}", token);
                self.convert_html_ident_to_rusthtmltoken(&ident, parse_ctx, output, it, is_raw_tokenstream)?;
            },
            TokenTree::Literal(literal) => {
                self.convert_html_literal_to_rusthtmltoken(&literal, parse_ctx, output, is_raw_tokenstream)?;
            },
            TokenTree::Punct(punct) => {
                return self.convert_html_punct_to_rusthtmltoken(&punct, parse_ctx, output, it, is_raw_tokenstream);
            },
            _ => {
                return Err(RustHtmlError::from_string(format!("next_and_parse_html_tag Unexpected token {:?}", token)));
            },
        }
        Ok(false)
    }

    // convert a Rust identifier to a RustHtml token in the context of a HTML tag.
    // ident: the identifier to convert.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_html_ident_to_rusthtmltoken(
        self: &Self, 
        ident: &Ident,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        output: &mut Vec<RustHtmlToken>, 
        it: Rc<dyn IPeekableTokenTree>,
        _is_raw_tokenstream: bool,
    ) -> Result<(), RustHtmlError> {
        self.context.add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::convert_html_ident_to_rusthtmltoken), ident.to_string()));
        if parse_ctx.is_parsing_attrs() {
            if parse_ctx.is_parsing_attr_val() {
                parse_ctx.html_attr_val_ident_push(ident);
                self.on_kvp_defined(parse_ctx, output)?;
            } else {
                parse_ctx.html_attr_key_ident_push(ident);
                parse_ctx.html_attr_key_push_str(&ident.to_string());
            }
        } else {
            parse_ctx.tag_name_push_ident(ident);
            let mut last_token_was_ident = true;
            loop {
                if let Some(next_token) = it.peek() {
                    match next_token {
                        TokenTree::Punct(ref punct) if punct.as_char() == '-' => {
                            parse_ctx.tag_name_push_punct(punct);
                            it.next();
                            last_token_was_ident = false;
                        },
                        TokenTree::Ident(ref ident) if last_token_was_ident == false => {
                            parse_ctx.tag_name_push_ident(ident);
                            it.next();
                            last_token_was_ident = true;
                        },
                        _ => {
                            parse_ctx.on_html_tag_name_parsed(output);
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }

        Ok(())
    }

    // convert a Rust literal to a RustHtml token in the context of a HTML tag.
    // literal: the literal to convert.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // returns: nothing or an error.
    fn convert_html_literal_to_rusthtmltoken(
        self: &Self, 
        literal: &Literal,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        output: &mut Vec<RustHtmlToken>, 
        _is_raw_tokenstream: bool,
    ) -> Result<(), RustHtmlError> {
        self.context.add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::convert_html_literal_to_rusthtmltoken), literal.to_string()));
        if parse_ctx.is_parsing_attrs() {
            if parse_ctx.is_parsing_attr_val() {
                if parse_ctx.is_key_defined() {
                    parse_ctx.set_html_attr_val_literal(literal);
                    self.on_kvp_defined(parse_ctx, output)?;
                } else {
                    panic!("was supposed to call on_kvp_defined but key was None (literal: {:?})", literal);
                }
            } else {
                parse_ctx.set_html_attr_key_literal(literal);
                let s = snailquote::unescape(&literal.to_string()).unwrap();
                parse_ctx.html_attr_key_push_str(&s);
                parse_ctx.set_parse_attr_val(true);
            }
        } else {
            return Err(RustHtmlError(Cow::Owned(format!("Cannot use literal for tag name"))))
        }

        Ok(())
    }

    // convert a Rust punct to a RustHtml token in the context of a HTML tag.
    // punct: the punct to convert.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: whether we should break the outer loop or not, or an error.
    fn convert_html_punct_to_rusthtmltoken(
        self: &Self, 
        punct: &Punct,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        output: &mut Vec<RustHtmlToken>, 
        it: Rc<dyn IPeekableTokenTree>,
        is_raw_tokenstream: bool,
    ) -> Result<bool, RustHtmlError> {
        let c = punct.as_char();
        if parse_ctx.is_parsing_attrs() {
            parse_ctx.get_main_context().add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::convert_html_punct_to_rusthtmltoken), c));
            match c {
                '>' => {
                    return self.on_html_tag_parsed(None, parse_ctx, output);
                },
                '=' => {
                    if parse_ctx.is_key_defined() {
                        parse_ctx.set_equals_punct(punct);
                    } else {
                        // need some context here
                        let next_token = it.peek().unwrap();
                        return Err(RustHtmlError::from_string(format!("convert_html_punct_to_rusthtmltoken Unexpected '=' before {:?} (key was None)", next_token)));
                    }
                },
                '/' => {
                    let expect_closing_punct = it.next().unwrap();
                    match expect_closing_punct {
                        TokenTree::Punct(closing_punct) => {
                            if closing_punct.as_char() == '>' {
                                parse_ctx.set_is_self_contained_tag(true);
                                return self.on_html_tag_parsed(Some(&punct), parse_ctx, output);
                            } else {
                                return Err(RustHtmlError::from_string(format!("convert_html_punct_to_rusthtmltoken Unexpected character '{}' (expected '>', prev: '{}')", closing_punct, c)));
                            }
                        },
                        _ => {
                            return Err(RustHtmlError::from_string(format!("convert_html_punct_to_rusthtmltoken Unexpected token after /: {}", c)));
                        },
                    }
                },
                '"' => {
                    if parse_ctx.has_html_attr_key() {
                        parse_ctx.set_parse_attr_val(true);
                    } else if parse_ctx.has_html_attr_val() {
                        self.on_kvp_defined(parse_ctx, output)?;
                    }
                },
                '-' => {
                    if parse_ctx.is_parsing_attr_val() {
                        parse_ctx.html_attr_val_ident_push_punct(punct);
                    } else {
                        parse_ctx.html_attr_key_ident_push_punct(punct);
                        parse_ctx.html_attr_key_push_str(format!("{}", c).as_str());
                    }
                },
                '@' => {
                    // escaping the html to insert value
                    let directive_token = it.next().unwrap();

                    // fixme: this needs to be fixed, it is not checking directive logic
                    match directive_token {
                        TokenTree::Ident(ref ident) => {
                            let mut rust_ident_exp = vec![];
                            self.parse_identifier_expression(true, ident, &directive_token, false, &mut rust_ident_exp, it, is_raw_tokenstream)?;
                            parse_ctx.set_html_attr_val_rust(rust_ident_exp);
                        },
                        TokenTree::Literal(ref literal) => {
                            parse_ctx.set_html_attr_val_literal(literal);
                        },
                        _ => {
                            return Err(RustHtmlError::from_string(format!("Unexpected directive token after '@' in html attribute val parse: {:?}", directive_token)));
                        }
                    }

                    // can't just call this, need to wrap in if
                    if parse_ctx.is_kvp_defined() {
                        self.on_kvp_defined(parse_ctx, output)?;
                    }
                },
                _ => {
                    let current_val = if parse_ctx.has_html_attr_val_ident() {
                        format!("ignoring {:?}", parse_ctx.get_html_attr_val_ident())
                    } else {
                        parse_ctx.get_html_attr_val_literal().as_ref().unwrap().to_string()
                    };
                    return Err(RustHtmlError::from_string(format!(
                        "Unexpected punct '{}' while parsing HTML tag '{}' attributes \
                        (read {:?}, current key: {}, current val: {:?})", c, parse_ctx.tag_name_as_str(),
                        parse_ctx.get_html_attrs(), parse_ctx.get_html_attr_key(), current_val)));
                }
            }
        } else {
            match c {
                '>' => {
                    return self.on_html_tag_parsed(None, parse_ctx, output);
                },
                '/' => {
                    if parse_ctx.has_tag_name() {
                        let expect_closing_punct = it.next().unwrap();
                        return match expect_closing_punct {
                            TokenTree::Punct(closing_punct) => {
                                if closing_punct.as_char() == '>' {
                                    parse_ctx.set_is_self_contained_tag(true);
                                    return self.on_html_tag_parsed(Some(&punct), parse_ctx, output);
                                } else {
                                    Err(RustHtmlError::from_string(format!("Unexpected character '{}' (expected '>', prev: '{}')", closing_punct, c)))
                                }
                            },
                            _ => {
                                Err(RustHtmlError::from_string(format!("convert_html_punct_to_rusthtmltoken Unexpected token after / (tag_name = {}): {:?}", parse_ctx.tag_name_as_str(), expect_closing_punct)))
                            },
                        };
                    } else {
                        parse_ctx.set_is_opening_tag(false);
                    }
                },
                '-' | '_' | '!' => {
                    parse_ctx.tag_name_push_punct(punct);
                },
                _ => {
                    return Err(RustHtmlError::from_string(format!("Unexpected character '{}'", c)));
                },
            }
        }
        Ok(false) // do not break
    }

    // called when a HTML tag attribute key/value pair is defined.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // returns: nothing.
    fn on_kvp_defined(
        self: &Self,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        output: &mut Vec<RustHtmlToken>,
    ) -> Result<(), RustHtmlError> {
        let r = parse_ctx.on_kvp_defined();
        match r {
            Ok(x) => {
                output.extend_from_slice(&x);
                Ok(())
            },
            Err(RustHtmlError(e)) => {
                Err(RustHtmlError::from_string(format!("error on_kvp_defined: {}", e)))
            }
        }
    }

    // parse a Rust type identifier from a stream of tokens.
    // it: the iterator to use.
    // returns: the type identifier or an error.
    fn parse_type_identifier(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<TokenTree>, RustHtmlError> {
        // let mut type_parts: Vec<TokenTree> = vec![];
        // loop
        // {
        //     let ident_token = it.peek().unwrap();
        //     match ident_token.clone() {
        //         TokenTree::Ident(ident) => {
        //             if ident.to_string() == "as" {
        //                 break;
        //             }

        //             type_parts.push(it.next().unwrap().clone());

        //             // might have generics
        //             if let Some(generic_start_token) = it.peek() {
        //                 match generic_start_token {
        //                     TokenTree::Punct(punct) => {
        //                         if punct.as_char() == '<' {
        //                             type_parts.push(it.next().unwrap().clone());

        //                             let mut punct_stack = vec![];
        //                             loop {
        //                                 let peek_token = it.peek();
        //                                 if let Some(token) = peek_token {
        //                                     match token {
        //                                         TokenTree::Punct(punct) => {
        //                                             let c = punct.as_char();
        //                                             match c {
        //                                                 '<' => {
        //                                                     punct_stack.push(punct.clone());
        //                                                     type_parts.push(it.next().unwrap().clone());
        //                                                 },
        //                                                 '>' => {
        //                                                     type_parts.push(it.next().unwrap().clone());
        //                                                     if punct_stack.len() > 0 {
        //                                                         punct_stack.pop();
        //                                                     } else {
        //                                                         break;
        //                                                     }
        //                                                 },
        //                                                 _ => {
        //                                                     type_parts.push(it.next().unwrap().clone());
        //                                                 }
        //                                             }
        //                                         },
        //                                         _ => {
        //                                             type_parts.push(it.next().unwrap().clone());
        //                                         }
        //                                     }
        //                                 } else {
        //                                     break;
        //                                 }
        //                             }
        //                         }
        //                     },
        //                     _ => {
        //                     }
        //                 }
        //             }

        //             // peek for next 3 punct tokens
        //             // if it is a colon, then push it
        //             let mut colons = vec![];
        //             for i in 0..3 {
        //                 if let Some(peek_colon) = it.peek_nth(i) {
        //                     match &peek_colon {
        //                         TokenTree::Punct(punct) => {
        //                             match punct.as_char() {
        //                                 ':' => {
        //                                     colons.push(peek_colon);
        //                                 },
        //                                 _ => break,
        //                             }
        //                         },
        //                         _ => break,
        //                     }
        //                 } else {
        //                     break;
        //                 }
        //             }

        //             // if only one is colon, then break
        //             // if none are colon, then break
        //             // if two then push them to type_parts
        //             // if more than two then error
        //             match colons.len() {
        //                 0 => break,
        //                 1 => break,
        //                 2 => {
        //                     it.next();
        //                     it.next();
        //                     type_parts.extend_from_slice(&colons);
        //                 },
        //                 _ => {
        //                     return self.panic_or_return_error(format!("unexpected colon count: {}", colons.len()));
        //                 }
        //             }

        //             // check that this is not a generic type
        //             // if it is, then add to output.
        //             let peek_token = it.peek();
        //             if let Some(token) = peek_token {
        //                 match token {
        //                     TokenTree::Punct(punct) => {
        //                         if punct.as_char() == '<' {
        //                             type_parts.push(it.next().unwrap().clone());
        //                             loop {
        //                                 let peek_token = it.peek();
        //                                 if let Some(token) = peek_token {
        //                                     match token {
        //                                         TokenTree::Punct(punct) => {
        //                                             if punct.as_char() == '>' {
        //                                                 type_parts.push(it.next().unwrap().clone());
        //                                                 break;
        //                                             } else {
        //                                                 type_parts.push(it.next().unwrap().clone());
        //                                             }
        //                                         },
        //                                         _ => {
        //                                             type_parts.push(it.next().unwrap().clone());
        //                                         }
        //                                     }
        //                                 } else {
        //                                     break;
        //                                 }
        //                             }
        //                         }
        //                     },
        //                     _ => {
        //                     }
        //                 }
        //             }
        //         },
        //         TokenTree::Punct(punct) => {
        //             match punct.as_char() {
        //                 '_' | '<' | '>' => type_parts.push(it.next().unwrap().clone()),
        //                 _ =>  break,
        //             }
        //         },
        //         _ => {
        //             return self.panic_or_return_error(format!("unexpected token after model directive: {:?}", ident_token));
        //         }
        //     }
        // }
        // Ok(type_parts)
        let new_parser = RustHtmlParserRust::new();
        match new_parser.parse_type_identifier(it) {
            Ok(x) => Ok(x),
            Err(e) => Err(RustHtmlError::from_string(format!("error parsing type identifier: {}", e))),
        }
    }

    // called when a HTML tag is parsed.
    // punct: the punct token.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // returns: whether we should break the outer loop or not, or an error.
    fn on_html_tag_parsed(
        self: &Self,
        punct: Option<&Punct>,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        output: &mut Vec<RustHtmlToken>
    ) -> Result<bool, RustHtmlError> {
        if parse_ctx.is_opening_tag() {
            if parse_ctx.is_kvp_defined() {
                self.on_kvp_defined(parse_ctx.clone(), output)?;
            }
        }

        for tag_helper in self.context.get_tag_parsed_handler() {
            if tag_helper.matches(parse_ctx.tag_name_as_str().as_str(), parse_ctx.is_opening_tag()) {
                match tag_helper.on_tag_parsed(parse_ctx.clone(), self.context.clone(), output) {
                    Ok(should_break) => {
                        if should_break {
                            break;
                        }
                    },
                    Err(e) => {
                        return Err(RustHtmlError::from_string(format!("error while processing tag helper: {}", e)));
                    }
                }
                break;
            }
        }

        if parse_ctx.is_opening_tag() {
            output.push(
                if parse_ctx.is_void_tag() {
                    RustHtmlToken::HtmlTagCloseVoidPunct(punct.map(|punct| (punct.as_char(), punct.clone())))
                } else if parse_ctx.is_self_contained_tag() {
                    RustHtmlToken::HtmlTagCloseSelfContainedPunct
                } else {
                    RustHtmlToken::HtmlTagCloseStartChildrenPunct
                }
            );
            return Ok(true); // parse_ctx.is_void_tag() break if void tag, no children
        } else {
            return Ok(true); // break when closing
        }
    }

    // called when a HTML node is parsed.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // returns: whether we should break the outer loop or not, or an error.
    fn on_html_node_parsed(
        self: &Self,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        output: &mut Vec<RustHtmlToken>
    ) -> Result<bool, RustHtmlError> {
        for node_helper in self.context.get_node_parsed_handler() {
            if node_helper.matches(parse_ctx.tag_name_as_str().as_str()) {
                match node_helper.on_node_parsed(parse_ctx, self.context.clone(), output) {
                    Ok(should_break) => {
                        if should_break {
                            break;
                        }
                    },
                    Err(e) => {
                        return Err(RustHtmlError::from_string(format!("error while processing tag helper: {}", e)));
                    }
                }
                break;
            }
        }
        Ok(true)
    }

    // convert a Rust group, identifier, or literal to RustHtml tokens.
    // token: the token to convert.
    // output: the destination for the RustHtml tokens.
    // returns: nothing or an error.
    fn convert_copy(self: &Self, token: TokenTree, output: &mut Vec<RustHtmlToken>) -> Result<(), RustHtmlError> {
        output.push(match token.clone() {
            TokenTree::Literal(literal) => RustHtmlToken::Literal(Some(literal), None),
            TokenTree::Ident(ident) => RustHtmlToken::Identifier(ident),
            TokenTree::Group(group) => RustHtmlToken::Group(group.delimiter(), group),
            _ => {
                return Err(RustHtmlError::from_string(format!("unexpected token: {:?}", token)));
            },
        });
        Ok(())
    }

    // convert a RustHtml identifier or punct or group or literal to Rust tokens.
    // tag: the tag to convert.
    // returns: the converted tokens or an error.
    fn convert_ident_and_punct_and_group_or_literal_to_tokenstream(self: &Self, tag: &RustHtmlIdentAndPunctAndGroupOrLiteral) -> Result<TokenStream, RustHtmlError> {
        Ok(TokenStream::from_iter(match tag {
            RustHtmlIdentAndPunctAndGroupOrLiteral::IdentAndPunctAndGroup(ident_and_punct) => {
                if ident_and_punct.len() == 0 {
                    return Err(RustHtmlError::from_string(format!("ident_and_punct was empty")));
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

    fn get_context(self: &Self) -> Rc<dyn IRustHtmlParserContext> {
        self.context.clone()
    }

    // need to differentiate between type and value identifier.
    // or at least allow '.' where we allow ':'
    // also need to allow optional add identifier token in case of *html*.link
    fn extract_identifier_expression(self: &Self, add_first_ident: bool, identifier_token: &TokenTree, last_token_was_ident: bool, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<TokenTree>, RustHtmlError> {
        // let mut output = vec![];
        // if add_first_ident {
        //     output.push(identifier_token.clone());
        // }
        // // this needs to be an argument
        // let mut last_token_was_ident = last_token_was_ident;
        // loop {
        //     let token_option = it.peek();
        //     if let Some(token) = token_option {
        //         match token {
        //             TokenTree::Literal(_literal) => {
        //                 output.push(it.next().unwrap());
        //             },
        //             TokenTree::Ident(_ident) => {
        //                 if last_token_was_ident {
        //                     break;
        //                 } else {
        //                     output.push(it.next().unwrap());
        //                     last_token_was_ident = true;
        //                     continue;
        //                 }
        //             },
        //             TokenTree::Group(group) => {
        //                 output.push(it.next().unwrap());
        //                 // not a function call or index
        //                 match group.delimiter() {
        //                     Delimiter::Brace |
        //                     Delimiter::Parenthesis => break,
        //                     _ => {}
        //                 }
        //             },
        //             TokenTree::Punct(punct) => {
        //                 let c = punct.as_char();
        //                 match c {
        //                     '.' | '?' | '!' | '_' | ':' | '&' => {
        //                         if last_token_was_ident {
        //                             output.push(it.next().unwrap());
        //                             last_token_was_ident = false;
        //                         } else {
        //                             break;
        //                         }
        //                     },
        //                     _ => {
        //                         break;
        //                     }
        //                 }
        //             },
        //         }
        //     } else {
        //         break;
        //     }

        //     last_token_was_ident = false;
        // }
        // Ok(output)
        let new_parser = RustHtmlParserRust::new();
        match new_parser.parse_rust_identifier_expression(add_first_ident, identifier_token, last_token_was_ident, it, is_raw_tokenstream) {
            Ok(x) => Ok(x),
            Err(e) => Err(RustHtmlError::from_string(format!("error extracting identifier expression: {}", e))),
        }
    }
}

// split all of RustToRustHtmlConverter into separate responsibilities:
// - parsing "HTML" tags
// - parsing Rust code
// - parsing RustHtml code

pub trait IRustHtmlParserAssignSharedParts {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>);
}

pub trait IRustHtmlParserAll {
    fn get_html_parser(&self) -> Rc<dyn IRustHtmlParserHtml>;
    fn get_rust_parser(&self) -> Rc<dyn IRustHtmlParserRust>;
    fn get_rust_or_html_parser(&self) -> Rc<dyn IRustHtmlParserRustOrHtml>;
}

pub trait IRustHtmlParserHtml: IRustHtmlParserAssignSharedParts {
    fn parse_html(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    fn parse_html_tag(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_html_node(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_html_attr_key(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_html_attr_val(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    fn parse_html_child_nodes(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
}

pub trait IRustHtmlParserRust: IRustHtmlParserAssignSharedParts {
    fn parse_rust(self: &Self, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    fn convert(self: &Self, token: TokenTree) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn convert_vec(self: &Self, tokens: Vec<TokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    fn parse_type_identifier(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn parse_rust_identifier_expression(self: &Self, add_first_ident: bool, identifier_token: &TokenTree, last_token_was_ident: bool, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn parse_rust_literal_expression(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_rust_group_expression(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_rust_punct_expression(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_rust_string_or_ident(self: &Self, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_rust_string_or_ident_or_punct_or_group(self: &Self, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_rust_string_or_ident_or_punct_or_group_or_literal(self: &Self, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;


    // assert that the next token is a punct. if it is, return nothing. otherwise, return the unexpected token.
    // c: the punct to expect.
    // it: the iterator to use.
    // returns: nothing or the unexpected token.
    fn expect_punct(self: &Self, c: char, it: Rc<dyn IPeekableTokenTree>) -> Result<(TokenTree, Punct), Option<TokenTree>>;
}

pub trait IRustHtmlParserRustOrHtml: IRustHtmlParserAssignSharedParts {
    fn parse_rust_or_html(self: &Self, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
}


// implementations

pub struct RustHtmlParserAll {
    html_parser: Rc<dyn IRustHtmlParserHtml>,
    rust_parser: Rc<dyn IRustHtmlParserRust>,
    rust_or_html_parser: Rc<dyn IRustHtmlParserRustOrHtml>,
}

impl RustHtmlParserAll {
    pub fn new(
        html_parser: Rc<dyn IRustHtmlParserHtml>,
        rust_parser: Rc<dyn IRustHtmlParserRust>,
        rust_or_html_parser: Rc<dyn IRustHtmlParserRustOrHtml>,
    ) -> Self {
        Self {
            html_parser,
            rust_parser,
            rust_or_html_parser,
        }
    }
}

impl IRustHtmlParserAll for RustHtmlParserAll {
    fn get_html_parser(&self) -> Rc<dyn IRustHtmlParserHtml> {
        self.html_parser.clone()
    }

    fn get_rust_parser(&self) -> Rc<dyn IRustHtmlParserRust> {
        self.rust_parser.clone()
    }

    fn get_rust_or_html_parser(&self) -> Rc<dyn IRustHtmlParserRustOrHtml> {
        self.rust_or_html_parser.clone()
    }
}

impl IRustHtmlParserAssignSharedParts for RustHtmlParserAll {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        self.html_parser.assign_shared_parser(parser.clone());
        self.rust_parser.assign_shared_parser(parser.clone());
        self.rust_or_html_parser.assign_shared_parser(parser.clone());
    }
}

pub struct RustHtmlParserHtml {
    parser: RefCell<Option<Rc<dyn IRustHtmlParserAll>>>,
}

impl RustHtmlParserHtml {
    pub fn new() -> Self {
        Self {
            parser: RefCell::new(None),
        }
    }
}

impl IRustHtmlParserAssignSharedParts for RustHtmlParserHtml {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        *self.parser.borrow_mut() = Some(parser);
    }
}

impl IRustHtmlParserHtml for RustHtmlParserHtml {
    // TODO: add tests
    fn parse_html(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableTokenTree>, _is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        loop {
            let next_token = it.peek();
            if let Some(token) = next_token {
                match token {
                    TokenTree::Group(group) => {
                        todo!()
                    },
                    TokenTree::Ident(ident) => {
                        todo!()
                    },
                    TokenTree::Literal(literal) => {
                        todo!()
                    },
                    TokenTree::Punct(punct) => {
                        todo!()
                    },
                }
            }
        }
        Ok(output)
    }

    fn parse_html_tag(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        // parse tag name
        // parse attributes (key value pairs)
        // parse closing puncts
        todo!()
    }

    // TODO: add tests
    fn parse_html_node(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        let mut parse_ctx = Rc::new(HtmlTagParseContext::new(Some(ctx))) as Rc<dyn IHtmlTagParseContext>;
        todo!();
        Ok(output)
    }

    // TODO: add tests
    fn parse_html_attr_key(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        // must be an identifier punct combo
        let mut output = vec![];
        let mut last_was_ident = false;
        loop {
            let token = it.peek();
            if let Some(ref token) = token {
                match token {
                    TokenTree::Ident(ident) => {
                        if last_was_ident {
                            break;
                        } else {
                            output.push(RustHtmlToken::ReservedIndent(ident.to_string(), ident.clone()));
                            it.next();
                            last_was_ident = true;
                        }
                    },
                    TokenTree::Punct(punct) => {
                        let c = punct.as_char();
                        match c {
                            '-' | '_' => {
                                output.push(RustHtmlToken::ReservedChar(punct.as_char(), punct.clone()));
                                it.next();
                                last_was_ident = false;
                            },
                            _ => {
                                return Err(RustHtmlError::from_string(format!("Unexpected token while parsing HTML tag attr key '{}' attribute key: {:?}", ctx.tag_name_as_str(), token)));
                            }
                        }
                    },
                    _ => {
                        return Err(RustHtmlError::from_string(format!("Unexpected token while parsing HTML tag attr key '{}' attribute key: {:?}", ctx.tag_name_as_str(), token)));
                    }
                }
            }
        }
        Ok(output)
    }

    // TODO: add tests
    fn parse_html_attr_val(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        // can only be literal or identifier punct combo
        let mut output = vec![];
        let mut last_was_ident = false;
        loop {
            let token = it.peek();
            if let Some(ref token) = token {
                match token {
                    TokenTree::Ident(ident) => {
                        if last_was_ident {
                            break;
                        } else {
                            output.push(RustHtmlToken::ReservedIndent(ident.to_string(), ident.clone()));
                            it.next();
                            last_was_ident = true;
                        }
                    },
                    TokenTree::Punct(punct) => {
                        let c = punct.as_char();
                        match c {
                            '-' | '_' => {
                                output.push(RustHtmlToken::ReservedChar(punct.as_char(), punct.clone()));
                                it.next();
                                last_was_ident = false;
                            },
                            _ => {
                                return Err(RustHtmlError::from_string(format!("Unexpected token while parsing HTML tag attr val '{}' attribute val: {:?}", ctx.tag_name_as_str(), token)));
                            }
                        }
                    },
                    TokenTree::Literal(literal) => {
                        output.push(RustHtmlToken::Literal(Some(literal.clone()), None));
                        it.next();
                        break;
                    },
                    _ => {
                        return Err(RustHtmlError::from_string(format!("Unexpected token while parsing HTML tag attr val '{}' attribute val: {:?}", ctx.tag_name_as_str(), token)));
                    }
                }
            }
        }
        Ok(output)
    }

    fn parse_html_child_nodes(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!()
    }

}

pub struct RustHtmlParserRust {
    parser: RefCell<Option<Rc<dyn IRustHtmlParserAll>>>,
}

impl RustHtmlParserRust {
    pub fn new() -> Self {
        Self {
            parser: RefCell::new(None),
        }
    }
}

impl IRustHtmlParserAssignSharedParts for RustHtmlParserRust {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        *self.parser.borrow_mut() = Some(parser);
    }
}

impl IRustHtmlParserRust for RustHtmlParserRust {
    fn parse_rust(self: &Self, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        loop {
            let next_token = it.peek();
            if let Some(token) = next_token {
                match token {
                    TokenTree::Ident(ident) => {
                        todo!("parse_rust_ident_expression");
                    },
                    TokenTree::Punct(punct) => {
                        output.extend_from_slice(&self.parse_rust_punct_expression(it.clone())?);
                    },
                    TokenTree::Literal(literal) => {
                        output.extend_from_slice(&self.parse_rust_literal_expression(it.clone())?);
                    },
                    TokenTree::Group(group) => {
                        output.extend_from_slice(&self.parse_rust_group_expression(it.clone())?);
                    },
                }
            } else {
                break;
            }
        }
        Ok(output)
    }

    fn parse_type_identifier(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let mut output = vec![];
        loop {
            let next_token = it.peek();
            if let Some(ref token) = next_token {
                match token {
                    TokenTree::Ident(ident) => {
                        output.push(it.next().unwrap());
                    },
                    TokenTree::Punct(punct) => {
                        match punct.as_char() {
                            '<' => {
                                output.push(it.next().unwrap());
                                let inner = self.parse_type_identifier(it.clone())?;
                                output.extend_from_slice(inner.as_slice());
                                
                                // assert that next token is '>'
                                match self.expect_punct('>', it) {
                                    Ok((t, _c)) => {
                                        output.push(t);
                                    },
                                    Err(None) => {
                                        return Err(RustHtmlError::from_string(format!("unexpected end of token stream")));
                                    },
                                    Err(Some(token)) => {
                                        return Err(RustHtmlError::from_string(format!("unexpected token: {:?}", token)));
                                    }
                                }
                                break;
                            },
                            ':' => {
                                output.push(it.next().unwrap());
                            },
                            _ => {
                                return Err(RustHtmlError::from_string(format!("unexpected punct: {:?}", token)));
                            }
                        }
                    },
                    _ => {
                        output.push(it.next().unwrap());
                    }
                }
            } else {
                break;
            }
        }
        Ok(output)
    }

    fn convert(self: &Self, token: TokenTree) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        match token {
            TokenTree::Ident(ident) => {
                output.push(RustHtmlToken::Identifier(ident));
            },
            TokenTree::Punct(punct) => {
                output.push(RustHtmlToken::ReservedChar(punct.as_char(), punct));
            },
            TokenTree::Literal(literal) => {
                output.push(RustHtmlToken::Literal(Some(literal), None));
            },
            TokenTree::Group(group) => {
                output.push(RustHtmlToken::Group(group.delimiter(), group));
            },
        }
        Ok(output)
    }

    fn convert_vec(self: &Self, tokens: Vec<TokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        for token in tokens {
            output.extend_from_slice(self.convert(token)?.as_slice());
        }
        Ok(output)
    }

    fn parse_rust_identifier_expression(self: &Self, add_first_ident: bool, identifier_token: &TokenTree, last_token_was_ident: bool, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<TokenTree>, RustHtmlError> {
        let mut output = vec![];
        if add_first_ident {
            output.push(identifier_token.clone());
        }
        // this needs to be an argument
        let mut last_token_was_ident = last_token_was_ident;
        loop {
            let token_option = it.peek();
            if let Some(token) = token_option {
                match token {
                    TokenTree::Literal(_literal) => {
                        output.push(it.next().unwrap());
                    },
                    TokenTree::Ident(_ident) => {
                        if last_token_was_ident {
                            break;
                        } else {
                            output.push(it.next().unwrap());
                            last_token_was_ident = true;
                            continue;
                        }
                    },
                    TokenTree::Group(group) => {
                        output.push(it.next().unwrap());
                        // not a function call or index
                        match group.delimiter() {
                            Delimiter::Brace |
                            Delimiter::Parenthesis => break,
                            _ => {}
                        }
                    },
                    TokenTree::Punct(punct) => {
                        let c = punct.as_char();
                        match c {
                            '.' | '?' | '!' | '_' | ':' | '&' => {
                                if last_token_was_ident {
                                    output.push(it.next().unwrap());
                                    last_token_was_ident = false;
                                } else {
                                    break;
                                }
                            },
                            _ => {
                                break;
                            }
                        }
                    },
                }
            } else {
                break;
            }

            last_token_was_ident = false;
        }
        Ok(output)
    }

    fn parse_rust_literal_expression(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!()
    }

    fn parse_rust_group_expression(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!()
    }

    fn parse_rust_punct_expression(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!()
    }

    fn parse_rust_string_or_ident(self: &Self, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!()
    }

    fn parse_rust_string_or_ident_or_punct_or_group(self: &Self, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!()
    }

    fn parse_rust_string_or_ident_or_punct_or_group_or_literal(self: &Self, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!()
    }

    fn expect_punct(self: &Self, c: char, it: Rc<dyn IPeekableTokenTree>) -> Result<(TokenTree, Punct), Option<TokenTree>> {
        if let Some(actual_c_token) = it.peek() {
            match actual_c_token { 
                TokenTree::Punct(ref punct) => {
                    let actual_c = punct.as_char();
                    if actual_c == c {
                        it.next();
                        Ok((actual_c_token.clone(), punct.clone()))
                    } else {
                        Err(Some(actual_c_token))
                    }
                },
                _ => Err(Some(actual_c_token.clone()))
            }
        } else {
            Err(None)
        }
    }
}