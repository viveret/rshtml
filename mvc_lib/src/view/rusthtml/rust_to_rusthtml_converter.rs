// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
use std::borrow::Cow;
use std::rc::Rc;
use std::str::FromStr;

use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, TokenStream, TokenTree};

use crate::core::panic_or_return_error::PanicOrReturnError;
use crate::view::rusthtml::rusthtml_token::{RustHtmlToken, RustHtmlIdentAndPunctOrLiteral, RustHtmlIdentOrPunct, RustHtmlIdentAndPunctAndGroupOrLiteral, RustHtmlIdentOrPunctOrGroup };
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use super::html_tag_parse_context::HtmlTagParseContext;
use super::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
use super::peekable_tokentree::{IPeekableTokenTree, PeekableTokenTree};
use super::rusthtml_directive_result::RustHtmlDirectiveResult;
use super::rusthtml_parser_context::IRustHtmlParserContext;


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

    // panic or return an error. if should_context.panic_or_return_error is true, then panic. otherwise, return an error.
    // message: the error message.
    // returns: an error with the message.
    pub fn panic_or_return_error<'a, T>(self: &Self, message: String) -> Result<T, RustHtmlError<'a>> {
        return PanicOrReturnError::panic_or_return_error(self.context.get_should_panic_or_return_error(), message);
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
                    output.push(RustHtmlToken::HtmlTextNode(ident.to_string(), token.span().clone()));
                } else {
                    output.push(RustHtmlToken::Identifier(ident));
                }
            },
            TokenTree::Literal(literal) => {
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
        match c {
            '@' => {
                self.convert_rust_entry_to_rusthtmltoken(c, punct, is_in_html_mode, output, it, is_raw_tokenstream)?;
            },
            '<' => {
                self.convert_html_entry_to_rusthtmltoken(c, punct, is_in_html_mode, output, it, is_raw_tokenstream)?;
            },
            '}' if !is_in_html_mode => {
                return Ok(true); // do not continue
            },
            '>' if !is_in_html_mode => {
                output.push(RustHtmlToken::ReservedChar(c, punct));
                // return self.panic_or_return_error(format!("Unexpected > (did you mean \"&gt;\"?)"));
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
                                                return self.panic_or_return_error(format!("Expected {{ after |->"));
                                            }
                                        }
                                    } else {
                                        return self.panic_or_return_error(format!("Expected {{ after |->"));
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

                // if punct.spacing() == proc_macro2::Spacing::Joint {
                //     output.push(RustHtmlToken::Space(' '));
                // }
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
            let mut ctx = HtmlTagParseContext::new();
            let mut output_inner = vec![];
            loop {
                let token_option = it.clone().next();
                if self.next_and_parse_html_tag(token_option, &mut ctx, &mut output_inner, it.clone(), is_raw_tokenstream)? {
                    break;
                }
            }

            let mut add_inner = true;
            if ctx.is_opening_tag && !ctx.is_void_tag() && !ctx.is_self_contained_tag {
                // parse inner elements / code until we find closing tag
                self.context.mut_htmltag_scope_stack().push(ctx.tag_name_as_str());
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
                let last_scope_from_stack = self.context.mut_htmltag_scope_stack().pop().unwrap();
                if last_scope_from_stack != ctx.tag_name_as_str() {
                    self.panic_or_return_error(format!("Mismatched HTML tags (found {} but expected {})", last_scope_from_stack, ctx.tag_name_as_str()))?;
                }

                if let Some(output_inner_last) = output_inner.last() {
                    if let RustHtmlToken::HtmlTagEnd(_tag_end, _tag_end_tokens) = output_inner_last {
                        add_inner = self.on_html_node_parsed(&ctx, &mut output_inner)?;
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
            TokenTree::Ident(ident) => {
                self.convert_rust_directive_identifier_to_rusthtmltoken(ident, prefix_token_option, output, it, is_raw_tokenstream)?;
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
                        return self.panic_or_return_error(format!("unexpected directive char: {}", c))?;
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
                    return self.panic_or_return_error(format!("unexpected delimiter: {:?}", delimiter));
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
    fn convert_rust_directive_identifier_to_rusthtmltoken(self: &Self, identifier: Ident, prefix_token_option: Option<RustHtmlToken>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError> {
        if let Some(directive) = self.context.try_get_directive(identifier.to_string()) {
            let r = directive.execute(&identifier, Rc::new(self.clone()), output, it);
            match r {
                Ok(r) => {
                    match r {
                        RustHtmlDirectiveResult::OkContinue => { },
                        RustHtmlDirectiveResult::OkBreak => { },
                        RustHtmlDirectiveResult::OkBreakAppendHtml => output.push(RustHtmlToken::AppendToHtml(vec![])),
                    }
                },
                Err(RustHtmlError(e)) => {
                    self.panic_or_return_error(format!("error executing directive: {}", e))?;
                }
            }
        } else {
            let mut inner_tokens = vec![];
            if let Some(prefix_token) = prefix_token_option {
                inner_tokens.push(prefix_token);
            }
            self.parse_identifier_expression(identifier, &mut inner_tokens, it, is_raw_tokenstream)?;
            output.push(RustHtmlToken::AppendToHtml(inner_tokens));
        }
        Ok(())
    }

    // convert a Rust identifier expression to a path string relative to the current working directory.
    // identifier: the identifier to convert.
    // it: the iterator to use.
    // returns: the path string or an error.
    fn convert_path_str(self: &Self, identifier: Ident, it: Rc<dyn IPeekableTokenTree>, _is_raw_tokenstream: bool) -> Result<String, RustHtmlError> {
        let mut path = std::path::PathBuf::new();

        let cwd = std::env::current_dir().unwrap();
        path.push(cwd);
        let relative_path = self.parse_string_with_quotes(false, identifier.clone(), it)?;
        path.push(relative_path.clone());

        Ok(path.to_str().unwrap().to_string())
    }

    // convert a Rust identifier expression to a path string relative to the views directory.
    // identifier: the identifier to convert.
    // it: the iterator to use.
    // returns: the path string or an error.
    fn convert_views_path_str(self: &Self, identifier: Ident, it: Rc<dyn IPeekableTokenTree>, _is_raw_tokenstream: bool) -> Result<String, RustHtmlError> {
        let path = self.parse_string_with_quotes(true, identifier.clone(), it)?;
        self.resolve_views_path_str(&path)
    }

    fn resolve_views_path_str(self: &Self, path: &str) -> Result<String, RustHtmlError> {
        if path == "_" {
            return Ok(path.to_string());
        }

        let cwd = std::env::current_dir().unwrap();

        // list of different prefixes to try
        let prefixes = vec![
            "src/views/",
            // folder,
            "src/views/shared/",
            ""
        ];

        // try each prefix
        let mut folders_tried = vec![];
        for prefix in prefixes {
            let mut path_buf = std::path::PathBuf::new();
            path_buf.push(cwd.clone());
            path_buf.push("example_web_app");
            path_buf.push(prefix);

            folders_tried.push(path_buf.to_str().unwrap().to_string());

            path_buf.push(path.clone());
    
            if path_buf.exists() {
                return Ok(path_buf.to_str().unwrap().to_string());
            } else {
            }
        }
        Err(RustHtmlError::from_string(format!("Could not find view {} in {:?}", path, folders_tried)))
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
            Err(e) => {
                self.panic_or_return_error(format!("{}", e))
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
                self.panic_or_return_error(format!("{}", e))
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
                RustHtmlToken::ReservedChar(c, _punct) => {
                    match c {
                        ';' => {
                            true
                        },
                        _ => {
                            false
                        }
                    }
                },
                RustHtmlToken::Group(..) => {
                    true
                },
                _ => {
                    false
                },
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
                _ => self.panic_or_return_error(format!("unexpected token after {} directive: {:?}", identifier, expect_string_token))?
            }
        } else {
            self.panic_or_return_error(format!("unexpected end of token stream after {} directive", identifier))?
        }
    }

    // parse Rust identifier expression and convert it to RustHtml tokens.
    // identifier: the identifier to convert.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn parse_identifier_expression(self: &Self, identifier: Ident, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError> {
        output.push(RustHtmlToken::Identifier(identifier.clone()));
        let mut last_token_was_ident = true;
        loop {
            let token_option = it.peek();
            if let Some(token) = token_option {
                match token {
                    TokenTree::Literal(literal) => {
                        output.push(RustHtmlToken::Literal(Some(literal.clone()), None));
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
                        let delimiter = group.delimiter();
                        let mut inner_tokens = vec![];
                        self.loop_next_and_convert(false, &mut inner_tokens, Rc::new(PeekableTokenTree::new(group.stream())), is_raw_tokenstream)?;
                        output.push(RustHtmlToken::GroupParsed(delimiter, inner_tokens));
                       
                        it.next();

                        // // not a function call or index
                        if delimiter == Delimiter::Brace {
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
        Ok(())
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
                TokenTree::Ident(ident2) => {
                    let mut inner_tokens = vec![];
                    self.parse_identifier_expression(ident2.clone(), &mut inner_tokens, it, is_raw_tokenstream)?;
                    Ok(RustHtmlIdentAndPunctAndGroupOrLiteral::IdentAndPunctAndGroup(self.convert_rusthtmltokens_to_ident_or_punct_or_group(inner_tokens)?))
                },
                _ => {
                    self.panic_or_return_error(format!("convert_string_or_ident did not find string or ident"))?
                }
            }
        } else {
            self.panic_or_return_error(format!("convert_string_or_ident did not find string or ident"))?
        }
    }

    // convert RustHtml tokens to a RustHtml identifier or punct or group.
    // tokens: the tokens to convert.
    // returns: the converted tokens or an error.
    fn convert_rusthtmltokens_to_ident_or_punct_or_group(self: &Self, tokens: Vec<RustHtmlToken>) -> Result<Vec<RustHtmlIdentOrPunctOrGroup>, RustHtmlError> {
        if tokens.len() == 0 {
            return self.panic_or_return_error(format!("tokens was empty"))?;
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
        token_option: Option<TokenTree>,
        parse_ctx: &mut HtmlTagParseContext,
        output: &mut Vec<RustHtmlToken>,
        it: Rc<dyn IPeekableTokenTree>,
        is_raw_tokenstream: bool,
    ) -> Result<bool, RustHtmlError> {
        match token_option {
            Some(token) => {
                match token {
                    TokenTree::Ident(ident) => {
                        self.convert_html_ident_to_rusthtmltoken(&ident, parse_ctx, output, it, is_raw_tokenstream)?;
                    },
                    TokenTree::Literal(literal) => {
                        self.convert_html_literal_to_rusthtmltoken(&literal, parse_ctx, output, is_raw_tokenstream)?;
                    },
                    TokenTree::Punct(punct) => {
                        if self.convert_html_punct_to_rusthtmltoken(&punct, parse_ctx, output, it, is_raw_tokenstream)? {
                            return Ok(true); // break
                        }
                    },
                    _ => {
                        return self.panic_or_return_error(format!("next_and_parse_html_tag Unexpected token {:?}", token));
                    },
                }
            },
            _ => {
                return Ok(true);// self.panic_or_return_error(format!("Could not read next token in next_and_parse_html_tag for {}", parse_ctx.tag_name));
            },
        }

        Ok(false) // continue
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
        parse_ctx: &mut HtmlTagParseContext,
        output: &mut Vec<RustHtmlToken>, 
        it: Rc<dyn IPeekableTokenTree>,
        _is_raw_tokenstream: bool,
    ) -> Result<(), RustHtmlError> {
        if parse_ctx.parse_attrs {
            if parse_ctx.parse_attr_val {
                parse_ctx.html_attr_val_ident.push(RustHtmlIdentOrPunct::Ident(ident.clone()));
                parse_ctx.parse_attr_val = false;
            } else {
                parse_ctx.html_attr_key_ident.push(RustHtmlIdentOrPunct::Ident(ident.clone()));
                parse_ctx.html_attr_key.push_str(&ident.to_string());
            }
        } else {
            parse_ctx.tag_name.push(RustHtmlIdentOrPunct::Ident(ident.clone()));
            let mut last_token_was_ident = true;
            loop {
                if let Some(next_token) = it.peek() {
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
        parse_ctx: &mut HtmlTagParseContext,
        output: &mut Vec<RustHtmlToken>, 
        _is_raw_tokenstream: bool,
    ) -> Result<(), RustHtmlError> {
        if parse_ctx.parse_attrs {
            if parse_ctx.parse_attr_val {
                parse_ctx.html_attr_val_literal = Some(literal.clone());
                self.on_kvp_defined(parse_ctx, output)?;
            } else {
                parse_ctx.html_attr_key_literal = Some(literal.clone());
                let s = snailquote::unescape(&literal.to_string()).unwrap();
                parse_ctx.html_attr_key.push_str(&s);
                parse_ctx.parse_attr_val = true;
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
        parse_ctx: &mut HtmlTagParseContext,
        output: &mut Vec<RustHtmlToken>, 
        it: Rc<dyn IPeekableTokenTree>,
        is_raw_tokenstream: bool,
    ) -> Result<bool, RustHtmlError> {
        let c = punct.as_char();
        if parse_ctx.parse_attrs {
            match c {
                '>' => {
                    return self.on_html_tag_parsed(None, parse_ctx, output);
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
                                return self.on_html_tag_parsed(Some(&punct), parse_ctx, output);
                            } else {
                                return self.panic_or_return_error(format!("convert_html_punct_to_rusthtmltoken Unexpected character '{}' (expected '>', prev: '{}')", closing_punct, c));
                            }
                        },
                        _ => {
                            return self.panic_or_return_error(format!("convert_html_punct_to_rusthtmltoken Unexpected token after /: {}", c));
                        },
                    }
                },
                '"' => {
                    if parse_ctx.html_attr_key.len() > 0 {
                        parse_ctx.parse_attr_val = true;
                    } else if parse_ctx.html_attr_val_ident.len() > 0 || parse_ctx.html_attr_val_literal.is_some() {
                        self.on_kvp_defined(parse_ctx, output)?;
                    }
                },
                '-' => {
                    if parse_ctx.parse_attr_val {
                        parse_ctx.html_attr_val_ident.push(RustHtmlIdentOrPunct::Punct(punct.clone()));
                    } else {
                        parse_ctx.html_attr_key_ident.push(RustHtmlIdentOrPunct::Punct(punct.clone()));
                        parse_ctx.html_attr_key.push_str(format!("{}", c).as_str());
                    }
                },
                '@' => {
                    // escaping the html to insert value
                    let directive_token = it.next().unwrap();

                    // fixme: this needs to be fixed
                    match directive_token {
                        TokenTree::Ident(ident) => {
                            let mut rust_ident_exp = vec![];
                            self.parse_identifier_expression(ident, &mut rust_ident_exp, it, is_raw_tokenstream)?;
                            parse_ctx.html_attr_val_rust = rust_ident_exp;
                        },
                        TokenTree::Literal(literal) => {
                            parse_ctx.html_attr_val_literal = Some(literal.clone());
                        },
                        _ => {
                            return self.panic_or_return_error(format!("Unexpected directive token after '@' in html attribute val parse: {:?}", directive_token))?;
                        }
                    }
                    self.on_kvp_defined(parse_ctx, output)?;
                },
                _ => {
                    let current_val = if parse_ctx.html_attr_val_ident.len() > 0 {
                        format!("{:?}", parse_ctx.html_attr_val_ident)
                    } else {
                        parse_ctx.html_attr_val_literal.as_ref().unwrap().to_string()
                    };
                    return self.panic_or_return_error(format!(
                        "Unexpected punct '{}' while parsing HTML tag '{}' attributes \
                        (read {:?}, current key: {}, current val: {:?})", c, parse_ctx.tag_name_as_str(),
                        parse_ctx.html_attrs, parse_ctx.html_attr_key, current_val));
                }
            }
        } else {
            match c {
                '>' => {
                    return self.on_html_tag_parsed(None, parse_ctx, output);
                },
                '/' => {
                    if parse_ctx.tag_name.len() > 0 {
                        let expect_closing_punct = it.next().unwrap();
                        return match expect_closing_punct {
                            TokenTree::Punct(closing_punct) => {
                                if closing_punct.as_char() == '>' {
                                    parse_ctx.is_self_contained_tag = true;
                                    return self.on_html_tag_parsed(Some(&punct), parse_ctx, output);
                                } else {
                                    self.panic_or_return_error(format!("Unexpected character '{}' (expected '>', prev: '{}')", closing_punct, c))
                                }
                            },
                            _ => {
                                self.panic_or_return_error(format!("convert_html_punct_to_rusthtmltoken Unexpected token after / (tag_name = {}): {:?}", parse_ctx.tag_name_as_str(), expect_closing_punct))
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

    // called when a HTML tag attribute key/value pair is defined.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // returns: nothing.
    fn on_kvp_defined(
        self: &Self,
        parse_ctx: &mut HtmlTagParseContext,
        output: &mut Vec<RustHtmlToken>,
    ) -> Result<(), RustHtmlError> {
        let mut attr_name = String::new();
        output.push(if let Some(is_literal) = &parse_ctx.html_attr_key_literal {
            let s = snailquote::unescape(&is_literal.to_string()).unwrap();
            attr_name.push_str(&s);
            RustHtmlToken::HtmlTagAttributeName(is_literal.to_string(), Some(RustHtmlIdentAndPunctOrLiteral::Literal(is_literal.clone())))
        } else if parse_ctx.html_attr_key_ident.len() > 0 {
            for ident_or_punct in &parse_ctx.html_attr_key_ident {
                match ident_or_punct {
                    RustHtmlIdentOrPunct::Ident(ident) => {
                        attr_name.push_str(&ident.to_string());
                    },
                    RustHtmlIdentOrPunct::Punct(punct) => {
                        attr_name.push(punct.as_char());
                    },
                }
            }
            RustHtmlToken::HtmlTagAttributeName(attr_name.clone(), Some(RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(parse_ctx.html_attr_key_ident.clone())))
        } else {
            return Err(RustHtmlError::from_string(format!("on_kvp_defined: html_attr_key_literal and html_attr_key_ident are both None")));
        });

        if let Some(is_literal) = &parse_ctx.html_attr_val_literal {
            output.push(RustHtmlToken::HtmlTagAttributeEquals(parse_ctx.equals_punct.as_ref().unwrap().as_char(), Some(parse_ctx.equals_punct.as_ref().unwrap().clone())));
            // println!("why am I missing quotes? {}", is_literal.to_string());
            // let s = snailquote::unescape(&is_literal.to_string()).unwrap();
            // println!("literal: {}", s); // this is missing quotes?
            output.push(RustHtmlToken::HtmlTagAttributeValue(Some(is_literal.to_string()), Some(is_literal.clone()), None, None));
            parse_ctx.html_attrs.insert(attr_name, Some(RustHtmlToken::Literal(Some(is_literal.clone()), Some(is_literal.to_string()))));
        } else if parse_ctx.html_attr_val_ident.len() > 0 {
            output.push(RustHtmlToken::HtmlTagAttributeEquals(parse_ctx.equals_punct.as_ref().unwrap().as_char(), Some(parse_ctx.equals_punct.as_ref().unwrap().clone())));

            let html_attr_val = RustHtmlToken::HtmlTagAttributeValue(None, None, Some(parse_ctx.html_attr_val_ident.clone()), None);
            output.push(html_attr_val.clone());
            parse_ctx.html_attrs.insert(attr_name, Some(html_attr_val));
        } else if parse_ctx.html_attr_val_rust.len() > 0 {
            output.push(RustHtmlToken::HtmlTagAttributeEquals(parse_ctx.equals_punct.as_ref().unwrap().as_char(), Some(parse_ctx.equals_punct.as_ref().unwrap().clone())));

            let html_attr_val = RustHtmlToken::HtmlTagAttributeValue(None, None, None, Some(parse_ctx.html_attr_val_rust.clone()));
            output.push(html_attr_val.clone());
            parse_ctx.html_attrs.insert(attr_name, Some(html_attr_val));
        } else {
            parse_ctx.html_attrs.insert(attr_name, None);
        }
        
        parse_ctx.clear_attr_kvp();

        Ok(())
    }

    // parse a Rust type identifier from a stream of tokens.
    // it: the iterator to use.
    // returns: the type identifier or an error.
    fn parse_type_identifier(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let mut type_parts: Vec<TokenTree> = vec![];
        loop
        {
            let ident_token = it.peek().unwrap();
            match ident_token.clone() {
                TokenTree::Ident(ident) => {
                    if ident.to_string() == "as" {
                        break;
                    }

                    type_parts.push(it.next().unwrap().clone());

                    // might have generics
                    if let Some(generic_start_token) = it.peek() {
                        match generic_start_token {
                            TokenTree::Punct(punct) => {
                                if punct.as_char() == '<' {
                                    type_parts.push(it.next().unwrap().clone());

                                    let mut punct_stack = vec![];
                                    loop {
                                        let peek_token = it.peek();
                                        if let Some(token) = peek_token {
                                            match token {
                                                TokenTree::Punct(punct) => {
                                                    let c = punct.as_char();
                                                    match c {
                                                        '<' => {
                                                            punct_stack.push(punct.clone());
                                                            type_parts.push(it.next().unwrap().clone());
                                                        },
                                                        '>' => {
                                                            type_parts.push(it.next().unwrap().clone());
                                                            if punct_stack.len() > 0 {
                                                                punct_stack.pop();
                                                            } else {
                                                                break;
                                                            }
                                                        },
                                                        _ => {
                                                            type_parts.push(it.next().unwrap().clone());
                                                        }
                                                    }
                                                },
                                                _ => {
                                                    type_parts.push(it.next().unwrap().clone());
                                                }
                                            }
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            },
                            _ => {
                            }
                        }
                    }

                    // peek for next 3 punct tokens
                    // if it is a colon, then push it
                    let mut colons = vec![];
                    for i in 0..3 {
                        if let Some(peek_colon) = it.peek_nth(i) {
                            match &peek_colon {
                                TokenTree::Punct(punct) => {
                                    match punct.as_char() {
                                        ':' => {
                                            colons.push(peek_colon);
                                        },
                                        _ => break,
                                    }
                                },
                                _ => break,
                            }
                        } else {
                            break;
                        }
                    }

                    // if only one is colon, then break
                    // if none are colon, then break
                    // if two then push them to type_parts
                    // if more than two then error
                    match colons.len() {
                        0 => break,
                        1 => break,
                        2 => {
                            it.next();
                            it.next();
                            type_parts.extend_from_slice(&colons);
                        },
                        _ => {
                            return self.panic_or_return_error(format!("unexpected colon count: {}", colons.len()));
                        }
                    }

                    // check that this is not a generic type
                    // if it is, then add to output.
                    let peek_token = it.peek();
                    if let Some(token) = peek_token {
                        match token {
                            TokenTree::Punct(punct) => {
                                if punct.as_char() == '<' {
                                    type_parts.push(it.next().unwrap().clone());
                                    loop {
                                        let peek_token = it.peek();
                                        if let Some(token) = peek_token {
                                            match token {
                                                TokenTree::Punct(punct) => {
                                                    if punct.as_char() == '>' {
                                                        type_parts.push(it.next().unwrap().clone());
                                                        break;
                                                    } else {
                                                        type_parts.push(it.next().unwrap().clone());
                                                    }
                                                },
                                                _ => {
                                                    type_parts.push(it.next().unwrap().clone());
                                                }
                                            }
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            },
                            _ => {
                            }
                        }
                    }
                },
                TokenTree::Punct(punct) => {
                    match punct.as_char() {
                        '_' | '<' | '>' => type_parts.push(it.next().unwrap().clone()),
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

    // called when a HTML tag is parsed.
    // punct: the punct token.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // returns: whether we should break the outer loop or not, or an error.
    fn on_html_tag_parsed(
        self: &Self,
        punct: Option<&Punct>,
        parse_ctx: &mut HtmlTagParseContext,
        output: &mut Vec<RustHtmlToken>
    ) -> Result<bool, RustHtmlError> {
        for tag_helper in self.context.get_tag_parsed_handler() {
            if tag_helper.matches(parse_ctx.tag_name_as_str().as_str(), parse_ctx.is_opening_tag) {
                match tag_helper.on_tag_parsed(parse_ctx, self.context.clone(), output) {
                    Ok(should_break) => {
                        if should_break {
                            break;
                        }
                    },
                    Err(e) => {
                        return self.panic_or_return_error(format!("error while processing tag helper: {}", e));
                    }
                }
                break;
            }
        }

        if parse_ctx.is_opening_tag {
            if parse_ctx.html_attr_key.len() > 0 {
                self.on_kvp_defined(parse_ctx, output)?;
            }

            output.push(
                if parse_ctx.is_void_tag() {
                    // println!("void tag: {}, punct: {:?}", parse_ctx.tag_name_as_str(), punct);
                    RustHtmlToken::HtmlTagCloseVoidPunct(punct.map(|punct| (punct.as_char(), punct.clone())))
                } else if parse_ctx.is_self_contained_tag {
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
        parse_ctx: &HtmlTagParseContext,
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
                        return self.panic_or_return_error(format!("error while processing tag helper: {}", e));
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
                return self.panic_or_return_error(format!("unexpected token: {:?}", token));
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

    fn get_context(self: &Self) -> Rc<dyn IRustHtmlParserContext> {
        self.context.clone()
    }
}
