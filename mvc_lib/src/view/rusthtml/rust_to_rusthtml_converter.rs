use core::panic;
// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use core_macro_lib::nameof_member_fn;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, TokenStream, TokenTree};

use crate::view::rusthtml::rusthtml_token::{RustHtmlToken, RustHtmlIdentAndPunctAndGroupOrLiteral, RustHtmlIdentOrPunctOrGroup };
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use super::html_tag_parse_context::HtmlTagParseContext;
use super::ihtml_tag_parse_context::IHtmlTagParseContext;
use super::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
use super::parser_parts::peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use super::parser_parts::peekable_tokentree::{IPeekableTokenTree, StreamPeekableTokenTree};
use super::parser_parts::rusthtmlparser_all::{IRustHtmlParserAll, IRustHtmlParserAssignSharedParts};
use super::rusthtml_directive_result::RustHtmlDirectiveResult;
use super::irusthtml_parser_context::IRustHtmlParserContext;
use super::rusthtml_parser::RustHtmlParser;


// this implements the IRustToRustHtml trait.
#[derive(Clone)]
pub struct RustToRustHtmlConverter {
    // the context for the RustHtml parser.
    pub context: RefCell<Option<Rc<dyn IRustHtmlParserContext>>>,
    // pub parser: RefCell<Option<Rc<dyn IRustHtmlParserAll>>>,
    pub parser: RefCell<Option<Rc<RustHtmlParser>>>,
}

// impl IRustHtmlParserAssignSharedParts for RustToRustHtmlConverter {
//     fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
//         self.parser.replace(Some(parser));
//     }
// }

impl RustToRustHtmlConverter {
    // create a new instance of the RustToRustHtml parser.
    // context: the context for the RustHtml parser.
    pub fn new(ctx: Option<Rc<dyn IRustHtmlParserContext>>) -> Self {
        Self {
            context: RefCell::new(ctx),
            parser: RefCell::new(None),
        }
    }

    fn peek_reserved_chars_in_str(self: &Self, arg: &'static str, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool, ct: Rc<dyn ICancellationToken>) -> Result<bool, RustHtmlError> {
        for c in arg.chars() {
            if !self.peek_reserved_char(c, output, it.clone(), is_raw_tokenstream, ct.clone())? {
                return Ok(false);
            }
        }
    
        Ok(true)
    }

    pub fn peek_reserved_char(self: &Self, expected_char: char, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, _is_raw_tokenstream: bool, _ct: Rc<dyn ICancellationToken>) -> Result<bool, RustHtmlError> {
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

    fn expect_punct(&self, expected_char: char, it: Rc<dyn IPeekableTokenTree>) -> Result<(TokenTree, char), RustHtmlError> {
        if let Some(next_token) = it.next() {
            match &next_token {
                TokenTree::Punct(next_punct) => {
                    if next_punct.as_char() == expected_char {
                        Ok((next_token.clone(), expected_char))
                    } else {
                        Err(RustHtmlError::from_string(format!("Expected '{}'", expected_char)))
                    }
                },
                _ => {
                    Err(RustHtmlError::from_string(format!("Expected '{}'", expected_char)))
                }
            }
        } else {
            Err(RustHtmlError::from_string(format!("Expected '{}'", expected_char)))
        }
    }

    fn get_context(self: &Self) -> Rc<dyn IRustHtmlParserContext> {
        self.context.borrow().as_ref().expect("context not set").clone()
    }

    // fn get_parser(self: &Self) -> Rc<dyn IRustHtmlParserAll> {
    //     self.parser.borrow().as_ref().expect("parser not set").clone()
    // }

    // fn try_get_parser(self: &Self) -> Result<Rc<dyn IRustHtmlParserAll>, RustHtmlError> {
    //     match self.parser.borrow().as_ref() {
    //         Some(parser) => Ok(parser.clone()),
    //         None => Err(RustHtmlError::from_str("parser not set")),
    //     }
    // }

    fn get_parser(self: &Self) -> Rc<RustHtmlParser> {
        self.parser.borrow().as_ref().expect("parser not set").clone()
    }

    fn try_get_parser(self: &Self) -> Result<Rc<RustHtmlParser>, RustHtmlError> {
        match self.parser.borrow().as_ref() {
            Some(parser) => Ok(parser.clone()),
            None => Err(RustHtmlError::from_str("parser not set")),
        }
    }
}

impl IRustToRustHtmlConverter for RustToRustHtmlConverter {
    fn assign_shared_parser(self: &Self, parser: Rc<RustHtmlParser>) {
        self.parser.replace(Some(parser));
    }

    // parse a token stream to RustHtml tokens.
    // is_in_html_mode: whether we are in HTML mode or not.
    // it: the token stream to parse.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: the RustHtml tokens.
     fn parse_tokenstream_to_rusthtmltokens(self: &Self, is_in_html_mode: bool, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut rusthtml_tokens = Vec::new();
        self.loop_next_and_convert(is_in_html_mode, &mut rusthtml_tokens, it, is_raw_tokenstream, ct.clone())?;
        Ok(rusthtml_tokens)
    }

    // loop through the token stream and convert it to RustHtml tokens.
    // is_in_html_mode: whether we are in HTML mode or not.
    // output: the destination for the RustHtml tokens.
    // it: the token stream to parse.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: nothing or error.
    fn loop_next_and_convert(self: &Self, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        loop {
            if self.next_and_convert(is_in_html_mode, output, it.clone(), is_raw_tokenstream, ct.clone())? {
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
    fn next_and_convert(self: &Self, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool, ct: Rc<dyn ICancellationToken>) -> Result<bool, RustHtmlError> {
        let token_option = it.next();

        if let Some(token) = token_option {
            if self.convert_tokentree_to_rusthtmltoken(token, is_in_html_mode, output, it, is_raw_tokenstream, ct.clone())? {
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
    fn convert_tokentree_to_rusthtmltoken(self: &Self, token: TokenTree, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool, ct: Rc<dyn ICancellationToken>) -> Result<bool, RustHtmlError> {
        match token.clone() {
            TokenTree::Ident(ident) => {
                if is_in_html_mode {
                    self.get_context().add_operation_to_ooo_log(format!("convert_tokentree_to_rusthtmltoken: {:?}", ident));
                    output.push(RustHtmlToken::HtmlTextNode(ident.to_string()));
                } else {
                    output.push(RustHtmlToken::Identifier(ident));
                }
            },
            TokenTree::Literal(literal) => {
                self.get_context().add_operation_to_ooo_log(format!("convert_tokentree_to_rusthtmltoken({:?})", literal));
                if is_in_html_mode {
                    output.push(RustHtmlToken::HtmlTextNode(literal.to_string()));
                } else {
                    output.push(RustHtmlToken::Literal(Some(literal), None));
                }
            },
            TokenTree::Punct(punct) => {
                if self.convert_punct_to_rusthtmltoken(punct, is_in_html_mode, output, it, is_raw_tokenstream, ct.clone())? {
                    return Ok(true);
                }
            },
            TokenTree::Group(group) => {
                self.convert_group_to_rusthtmltoken(group, false, is_in_html_mode, output, is_raw_tokenstream, ct.clone())?;
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
    fn convert_punct_to_rusthtmltoken(self: &Self, punct: Punct, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool, ct: Rc<dyn ICancellationToken>) -> Result<bool, RustHtmlError> {
        let c = punct.as_char();
        self.get_context().add_operation_to_ooo_log(format!("convert_punct_to_rusthtmltoken: {}", c));
        match c {
            '@' => {
                self.convert_rust_entry_to_rusthtmltoken(c, punct, is_in_html_mode, output, it, is_raw_tokenstream, ct.clone())?;
            },
            '<' => {
                self.convert_html_entry_to_rusthtmltoken(c, punct, true, output, it, is_raw_tokenstream, ct.clone())?;
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
                if self.peek_reserved_chars_in_str("|->", output, it.clone(), is_raw_tokenstream, ct.clone())? {
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
                                                self.convert_group_to_rusthtmltoken(group, true, is_in_html_mode, output, is_raw_tokenstream, ct.clone())?;
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
                    output.push(RustHtmlToken::HtmlTextNode(punct.as_char().to_string()));
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
    fn convert_rust_entry_to_rusthtmltoken(self: &Self, _c: char, _punct: Punct, _is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        if let Some(directive_token) = it.next() {
            self.convert_rust_directive_to_rusthtmltoken(directive_token, None, output, it, is_raw_tokenstream, ct)?;
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
    fn convert_html_entry_to_rusthtmltoken(self: &Self, c: char, punct: Punct, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        if is_in_html_mode || self.is_start_of_current_expression(output) {
            // the below context is orphaned by not passing the parent html tag parse context.
            // this is usually fine. but we need to pass the main context to call add_operation_to_ooo_log
            let ctx = Rc::new(HtmlTagParseContext::new(Some(self.get_context())));
            let mut output_inner = vec![];
            // it.enable_log_next("convert_html_entry_to_rusthtmltoken");
            loop {
                let token_option = it.next();
                if let Some(token) = token_option {
                    if self.next_and_parse_html_tag(&token, ctx.clone(), &mut output_inner, it.clone(), is_raw_tokenstream, ct.clone())? {
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
                self.get_context().htmltag_scope_stack_push(ctx.tag_name_as_str());
                loop {
                    if self.next_and_convert(true, &mut output_inner, it.clone(), is_raw_tokenstream, ct.clone())? {
                        break;
                    }
                    match output_inner.last().expect("output_inner.last() failed") {
                        RustHtmlToken::HtmlTagEnd(tag_end, _tag_end_tokens) => {
                            if tag_end == &ctx.tag_name_as_str() {
                                break;
                            }
                        },
                        _ => {
                        }
                    }
                }
                let last_scope_from_stack = self.get_context().htmltag_scope_stack_pop().expect("htmltag_scope_stack_pop failed");
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
    fn convert_group_to_rusthtmltoken(self: &Self, group: Group, expect_return_html: bool, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken>, is_raw_tokenstream: bool, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        let delimiter = group.delimiter();
        let it = Rc::new(StreamPeekableTokenTree::new(group.stream()));
        if is_in_html_mode {
            self.get_context().add_operation_to_ooo_log(format!("convert_group_to_rusthtmltoken: {:?}", group));
            let c_start = self.get_opening_delim(delimiter);
            let c_end = self.get_closing_delim(delimiter);

            output.push(RustHtmlToken::HtmlTextNode(c_start.to_string()));
            self.loop_next_and_convert(true, output, it, is_raw_tokenstream, ct.clone())?;
            output.push(RustHtmlToken::HtmlTextNode(c_end.to_string()));
        } else {
            if delimiter == Delimiter::Brace {
                let mut inner_tokens = vec![];
                
                // prefix and postfix with html_output decorators
                if expect_return_html {
                    self.loop_next_and_convert(is_in_html_mode, &mut inner_tokens, Rc::new(StreamPeekableTokenTree::new(quote::quote! { let html_output = HtmlBuffer::new(); }.into())), false, ct.clone())?;
                }
                
                self.loop_next_and_convert(false, &mut inner_tokens, it, is_raw_tokenstream, ct.clone())?;
                
                if expect_return_html {
                    self.loop_next_and_convert(is_in_html_mode, &mut inner_tokens, Rc::new(StreamPeekableTokenTree::new(quote::quote! { html_output.collect_html() }.into())), false, ct.clone())?;
                }

                output.push(RustHtmlToken::GroupParsed(delimiter, inner_tokens));
            } else {
                match self.convert_copy(TokenTree::Group(group), output, ct) {
                    Ok(_) => {},
                    Err(err) => {
                        return Err(err);
                    }
                }
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
    fn convert_rust_directive_to_rusthtmltoken(self: &Self, token: TokenTree, prefix_token_option: Option<RustHtmlToken>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool, ct: Rc<dyn ICancellationToken>) -> Result<bool, RustHtmlError>  {
        match token {
            TokenTree::Ident(ref ident) => {
                self.convert_rust_directive_identifier_to_rusthtmltoken(ident, &token, prefix_token_option, output, it, is_raw_tokenstream, self.get_context(), ct)?;
            },
            TokenTree::Group(group) => {
                self.convert_rust_directive_group_to_rusthtmltoken(group, prefix_token_option, output, is_raw_tokenstream, ct)?;
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
                            return self.convert_rust_directive_to_rusthtmltoken(token, Some(prefix_token), output, it, is_raw_tokenstream, ct);
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
    fn convert_rust_directive_group_to_rusthtmltoken(self: &Self, group: Group, _prefix_token_option: Option<RustHtmlToken>, output: &mut Vec<RustHtmlToken>, is_raw_tokenstream: bool, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        let mut inner_tokens = vec![];
        let it = Rc::new(StreamPeekableTokenTree::new(group.stream()));
        self.loop_next_and_convert(false, &mut inner_tokens, it, is_raw_tokenstream, ct)?;
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
    fn convert_rust_directive_identifier_to_rusthtmltoken(self: &Self, identifier: &Ident, ident_token: &TokenTree, prefix_token_option: Option<RustHtmlToken>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        if let Some(directive) = self.get_context().try_get_directive(identifier.to_string()) {
            match self.try_get_parser() {
                Ok(parser) => {
                    let r = directive.execute_old(context, &identifier, ident_token, parser, output, it, ct);
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
                },
                Err(RustHtmlError(e)) => {
                    return Err(RustHtmlError::from_string(format!("error getting parser in convert_rust_directive_identifier_to_rusthtmltoken: {}", e)));
                }
            }
            // let r = directive.execute(context, &identifier, ident_token, self.get_parser(), output, it, ct);
            // match r {
            //     Ok(r) => {
            //         match r {
            //             RustHtmlDirectiveResult::OkContinue => { },
            //             RustHtmlDirectiveResult::OkBreak => { },
            //             RustHtmlDirectiveResult::OkBreakAppendHtml => output.push(RustHtmlToken::AppendToHtml(vec![])),
            //         }
            //     },
            //     Err(RustHtmlError(e)) => {
            //         return Err(RustHtmlError::from_string(format!("error executing directive: {}", e)));
            //     }
            // }
        } else {
            let mut inner_tokens = vec![];
            if let Some(prefix_token) = prefix_token_option {
                inner_tokens.push(prefix_token);
            }
            self.parse_identifier_expression(true, identifier, ident_token, true, &mut inner_tokens, it, is_raw_tokenstream, ct)?;
            output.push(RustHtmlToken::AppendToHtml(inner_tokens));
        }
        Ok(())
    }

    // convert a Rust identifier expression to a path string relative to the current working directory.
    // identifier: the identifier to convert.
    // it: the iterator to use.
    // returns: the path string or an error.
    fn next_path_str(self: &Self, _context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, _identifier_token: &TokenTree, it: Rc<dyn IPeekableTokenTree>, _is_raw_tokenstream: bool, _ct: Rc<dyn ICancellationToken>) -> Result<String, RustHtmlError> {
        let mut path = std::path::PathBuf::new();
        let cwd = std::env::current_dir().expect("current_dir failed");
        path.push(cwd);
        let relative_path = self.parse_string_with_quotes(false, identifier.clone(), it)?;
        path.push(relative_path.clone());

        Ok(path.to_str().expect("could not convert path to string").to_string())
    }

    fn peek_path_str(self: &Self, _context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, _identifier_token: &TokenTree,  it: Rc<dyn IPeekableTokenTree>, _is_raw_tokenstream: bool, _ct: Rc<dyn ICancellationToken>) -> Result<String, RustHtmlError> {
        let mut path = std::path::PathBuf::new();
        let cwd = std::env::current_dir().expect("current_dir failed");
        path.push(cwd);
        let relative_path = self.parse_string_with_quotes(true, identifier.clone(), it)?;
        path.push(relative_path.clone());

        Ok(path.to_str().expect("could not convert path to string").to_string())
    }

    // expand an external token stream into RustHtml tokens.
    // path: the path to the external token stream.
    // output: the destination for the RustHtml tokens.
    // returns: nothing or an error.
    fn expand_external_tokenstream(self: &Self, path: &String, output: &mut Vec<RustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        match std::fs::read_to_string(path) {
            Ok(input_str) => {
                self.expand_external_rshtml_string(&input_str, output, ct)
            },
            Err(_e) => {
                let parent_path = std::path::Path::new(path).parent().expect("path.parent failed");
                match std::fs::read_to_string(parent_path) {
                    Ok(input_str) => {
                        self.expand_external_rshtml_string(&input_str, output, ct)
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
    fn expand_external_rshtml_string(self: &Self, input_str: &String, output: &mut Vec<RustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        let input_result = TokenStream::from_str(input_str.as_str());
        
        match input_result {
            Ok(input) => {
                let peekable = Rc::new(StreamPeekableTokenTree::new(input));
                let rusthtml_tokens = self.parse_tokenstream_to_rusthtmltokens(true, peekable, true, ct)?;
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
            let last = output.last().expect("output.last() failed");
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
                TokenTree::Literal(literal) => Ok(snailquote::unescape(&literal.to_string()).expect("snailquote::unescape failed")),
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
    fn parse_identifier_expression(self: &Self, add_first_ident: bool, _identifier: &Ident, identifier_token: &TokenTree, last_token_was_ident: bool, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        match self.extract_identifier_expression(add_first_ident, identifier_token, last_token_was_ident, it, is_raw_tokenstream, ct.clone()) {
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
                            self.loop_next_and_convert(false, &mut inner_tokens, Rc::new(StreamPeekableTokenTree::new(group.stream())), is_raw_tokenstream, ct.clone())?;
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
    fn convert_string_or_ident(self: &Self, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlIdentAndPunctAndGroupOrLiteral, RustHtmlError> {
        if let Some(expect_string_or_ident_token) = it.next() {
            match expect_string_or_ident_token {
                TokenTree::Literal(literal) => {
                    Ok(RustHtmlIdentAndPunctAndGroupOrLiteral::Literal(literal.clone()))
                },
                TokenTree::Ident(ref ident2) => {
                    let mut inner_tokens = vec![];
                    self.parse_identifier_expression(true, ident2, &expect_string_or_ident_token, true, &mut inner_tokens, it, is_raw_tokenstream, ct)?;
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
                RustHtmlToken::Group(_, _stream, group) => RustHtmlIdentOrPunctOrGroup::Group(group.clone().expect("group.clone failed")),
                RustHtmlToken::GroupParsed(delimiter, tokens) => RustHtmlIdentOrPunctOrGroup::Group(Group::new(delimiter.clone(), tokens.iter().map(|x| match x {
                    RustHtmlToken::Identifier(ident) => TokenTree::Ident(ident.clone()),
                    RustHtmlToken::ReservedChar(_, punct) => TokenTree::Punct(punct.clone()),
                    RustHtmlToken::Group(_, _stream, group) => TokenTree::Group(group.clone().expect("group.clone failed")),
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
        ct: Rc<dyn ICancellationToken>
    ) -> Result<bool, RustHtmlError> {
        match token {
            TokenTree::Ident(ident) => {
                // println!("next_and_parse_html_tag: {:?}", token);
                self.convert_html_ident_to_rusthtmltoken(&ident, parse_ctx, output, it, is_raw_tokenstream, ct)?;
            },
            TokenTree::Literal(literal) => {
                self.convert_html_literal_to_rusthtmltoken(&literal, parse_ctx, output, is_raw_tokenstream, ct)?;
            },
            TokenTree::Punct(punct) => {
                return self.convert_html_punct_to_rusthtmltoken(&punct, parse_ctx, output, it, is_raw_tokenstream, ct);
            },
            _ => {
                return Err(RustHtmlError::from_string(format!("RustToRustHtmlConverter::next_and_parse_html_tag Unexpected token {:?}", token)));
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
        _output: &mut Vec<RustHtmlToken>, 
        it: Rc<dyn IPeekableTokenTree>,
        _is_raw_tokenstream: bool,
        _ct: Rc<dyn ICancellationToken>
    ) -> Result<(), RustHtmlError> {
        self.get_context().add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::convert_html_ident_to_rusthtmltoken), ident.to_string()));
        if parse_ctx.is_parsing_attrs() {
            if parse_ctx.is_parsing_attr_val() {
                parse_ctx.html_attr_val_ident_push(ident);
                self.on_kvp_defined(parse_ctx)?;
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
                            match parse_ctx.on_html_tag_name_parsed() {
                                Ok(_) => {},
                                Err(e) => {
                                    return Err(RustHtmlError::from_string(format!("convert_html_ident_to_rusthtmltoken: {}", e)));
                                }
                            }
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
        _output: &mut Vec<RustHtmlToken>, 
        _is_raw_tokenstream: bool,
        _ct: Rc<dyn ICancellationToken>
    ) -> Result<(), RustHtmlError> {
        self.get_context().add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::convert_html_literal_to_rusthtmltoken), literal.to_string()));
        if parse_ctx.is_parsing_attrs() {
            if parse_ctx.is_parsing_attr_val() {
                if parse_ctx.is_key_defined() {
                    parse_ctx.set_html_attr_val_literal(literal);
                    self.on_kvp_defined(parse_ctx)?;
                } else {
                    panic!("was supposed to call on_kvp_defined but key was None (literal: {:?})", literal);
                }
            } else {
                parse_ctx.set_html_attr_key_literal(literal);
                let s = snailquote::unescape(&literal.to_string()).expect("snailquote::unescape failed");
                parse_ctx.html_attr_key_push_str(&s);
                parse_ctx.set_parse_attr_val(true);
            }
        } else {
            return Err(RustHtmlError::from_string(format!("Cannot use literal for tag name")))
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
        _output: &mut Vec<RustHtmlToken>, 
        it: Rc<dyn IPeekableTokenTree>,
        is_raw_tokenstream: bool,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<bool, RustHtmlError> {
        let c = punct.as_char();
        if parse_ctx.is_parsing_attrs() {
            parse_ctx.get_main_context().add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::convert_html_punct_to_rusthtmltoken), c));
            match c {
                '>' => {
                    return self.on_html_tag_parsed(None, parse_ctx, ct);
                },
                '=' => {
                    if parse_ctx.is_key_defined() {
                        parse_ctx.set_equals_punct(punct);
                    } else {
                        // need some context here
                        let next_token = it.peek().expect("it.peek failed");
                        return Err(RustHtmlError::from_string(format!("convert_html_punct_to_rusthtmltoken Unexpected '=' before {:?} (key was None)", next_token)));
                    }
                },
                '/' => {
                    let expect_closing_punct = it.next().expect("it.next failed");
                    match expect_closing_punct {
                        TokenTree::Punct(closing_punct) => {
                            if closing_punct.as_char() == '>' {
                                parse_ctx.set_is_self_contained_tag(true);
                                return self.on_html_tag_parsed(Some(&punct), parse_ctx, ct);
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
                        self.on_kvp_defined(parse_ctx)?;
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
                    let directive_token = it.next().expect("it.next failed");

                    // fixme: this needs to be fixed, it is not checking directive logic
                    match directive_token {
                        TokenTree::Ident(ref ident) => {
                            let mut rust_ident_exp = vec![];
                            self.parse_identifier_expression(true, ident, &directive_token, false, &mut rust_ident_exp, it, is_raw_tokenstream, ct)?;
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
                        self.on_kvp_defined(parse_ctx)?;
                    }
                },
                _ => {
                    let current_val = if parse_ctx.has_html_attr_val_ident() {
                        format!("ignoring {:?}", parse_ctx.get_html_attr_val_ident())
                    } else {
                        parse_ctx.get_html_attr_val_literal().as_ref().expect("get_html_attr_val_literal as_ref failed").to_string()
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
                    return self.on_html_tag_parsed(None, parse_ctx, ct);
                },
                '/' => {
                    if parse_ctx.has_tag_name() {
                        let expect_closing_punct = it.next().expect("it.next failed");
                        return match expect_closing_punct {
                            TokenTree::Punct(closing_punct) => {
                                if closing_punct.as_char() == '>' {
                                    parse_ctx.set_is_self_contained_tag(true);
                                    return self.on_html_tag_parsed(Some(&punct), parse_ctx, ct);
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
    ) -> Result<(), RustHtmlError> {
        let r = parse_ctx.on_kvp_defined();
        match r {
            Ok(x) => {
                match parse_ctx.get_main_context().push_output_tokens(&x) {
                    Ok(_) => {
                        Ok(())
                    },
                    Err(e) => {
                        return Err(RustHtmlError::from_string(format!("error on_kvp_defined: {}", e)));
                    }
                }
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
        let mut output = Vec::<TokenTree>::new();
        loop {
            let next_token = it.peek();
            if let Some(token) = &next_token {
                match token {
                    TokenTree::Ident(_ident) => {
                        output.push(it.next().expect("it.next()").clone());

                        // peek for next 3 punct tokens
                        // if it is a colon, then push it
                        let mut colons = Vec::<TokenTree>::new();
                        for i in 0..3 {
                            if let Some(peek_colon) = it.peek_nth(i) {
                                match &peek_colon {
                                    TokenTree::Punct(punct) => {
                                        let c = punct.as_char();
                                        match c {
                                            ':' => {
                                                colons.push(peek_colon.clone());
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
                                output.extend_from_slice(&colons);
                            },
                            _ => {
                                return Err(RustHtmlError::from_string(format!("unexpected colon count: {}", colons.len())));
                            }
                        }
                    },
                    TokenTree::Punct(punct) => {
                        let c = punct.as_char();
                        match c {
                            '<' => {
                                output.push(it.next().expect("it.next()").clone());
                                let inner = self.parse_type_identifier(it.clone())?;
                                output.extend_from_slice(&inner);
                                
                                // assert that next token is '>'
                                match self.expect_punct('>', it) {
                                    Ok((t, _c)) => {
                                        output.push(t);
                                    },
                                    Err(e) => {
                                        return Err(e);
                                    }
                                }
                                break;
                            },
                            ':' => {
                                output.push(it.next().expect("it.next()").clone());
                            },
                            _ => {
                                return Err(RustHtmlError::from_string(format!("parse_type_identifier unexpected punct: {:?}", token)));
                            }
                        }
                    },
                    _ => {
                        output.push(it.next().expect("it.next()").clone());
                    }
                }
            } else {
                break;
            }
        }
        Ok(output)
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
        ct: Rc<dyn ICancellationToken>,
    ) -> Result<bool, RustHtmlError> {
        if parse_ctx.is_opening_tag() {
            if parse_ctx.is_kvp_defined() {
                self.on_kvp_defined(parse_ctx.clone())?;
            }
        }

        for tag_helper in self.get_context().get_tag_parsed_handler() {
            if tag_helper.matches(parse_ctx.tag_name_as_str().as_str(), parse_ctx.is_opening_tag()) {
                match tag_helper.on_tag_parsed(parse_ctx.clone(), ct.clone()) {
                    Ok((_tokens, should_break)) => {
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
            match parse_ctx.get_main_context().push_output_token(
                if parse_ctx.is_void_tag() {
                    RustHtmlToken::HtmlTagCloseVoidPunct(punct.map(|punct| (punct.as_char(), punct.clone())))
                } else if parse_ctx.is_self_contained_tag() {
                    RustHtmlToken::HtmlTagCloseSelfContainedPunct
                } else {
                    RustHtmlToken::HtmlTagCloseStartChildrenPunct
                }
            ) {
                Ok(_) => {},
                Err(e) => {
                    return Err(RustHtmlError::from_string(format!("error on_html_tag_parsed: {}", e)));
                }
            }
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
        _output: &mut Vec<RustHtmlToken>
    ) -> Result<bool, RustHtmlError> {
        for node_helper in self.get_context().get_node_parsed_handler() {
            if node_helper.matches(parse_ctx.tag_name_as_str().as_str()) {
                match node_helper.on_node_parsed(parse_ctx, self.get_context()) {
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
    fn convert_copy(self: &Self, token: TokenTree, output: &mut Vec<RustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        output.push(match token.clone() {
            TokenTree::Literal(literal) => RustHtmlToken::Literal(Some(literal), None),
            TokenTree::Ident(ident) => RustHtmlToken::Identifier(ident),
            TokenTree::Group(group) => {
                let stream = group.stream();
                let peek_stream = Rc::new(StreamPeekableTokenTree::new(stream));
                let converted_stream = self.parse_tokenstream_to_rusthtmltokens(true, peek_stream, true, ct)?;
                RustHtmlToken::Group(group.delimiter(), Rc::new(VecPeekableRustHtmlToken::new(converted_stream)), Some(group))
            },
            _ => {
                return Err(RustHtmlError::from_string(format!("convert_copy unexpected token: {:?}", token)));
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
        self.context.borrow().clone().expect("context was None")
    }

    // need to differentiate between type and value identifier.
    // or at least allow '.' where we allow ':'
    // also need to allow optional add identifier token in case of *html*.link
    fn extract_identifier_expression(self: &Self, add_first_ident: bool, identifier_token: &TokenTree, last_token_was_ident: bool, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool, ct: Rc<dyn ICancellationToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let mut output = vec![];

        if add_first_ident {
            output.push(identifier_token.clone());
        }

        let mut last_token_was_ident = last_token_was_ident;
        loop {
            let next_token = it.peek();
            if let Some(token) = &next_token {
                match token {
                    TokenTree::Ident(_ident) => {
                        if last_token_was_ident {
                            return Err(RustHtmlError::from_string(format!("unexpected ident after ident: {:?}", token)));
                        }
                        output.push(it.next().expect("it.next()").clone());
                        last_token_was_ident = true;
                    },
                    TokenTree::Punct(punct) => {
                        let c = punct.as_char();
                        match c {
                            '.' => {
                                if last_token_was_ident {
                                    output.push(it.next().expect("it.next()").clone());
                                    last_token_was_ident = false;
                                } else {
                                    return Err(RustHtmlError::from_string(format!("unexpected '.' after punct: {:?}", token)));
                                }
                            },
                            ':' => {
                                if last_token_was_ident {
                                    output.push(it.next().expect("it.next()").clone());
                                    last_token_was_ident = false;
                                } else {
                                    return Err(RustHtmlError::from_string(format!("unexpected ':' after punct: {:?}", token)));
                                }
                            },
                            '!' => {
                                if last_token_was_ident {
                                    output.push(it.next().expect("it.next()").clone());
                                    last_token_was_ident = false;
                                    break;
                                } else {
                                    return Err(RustHtmlError::from_string(format!("unexpected '!' after punct: {:?}", token)));
                                }
                            },
                            '<' => {
                                if last_token_was_ident {
                                    output.push(it.next().expect("it.next()").clone());
                                    last_token_was_ident = false;

                                    // recurse into inner expression
                                    let inner = self.extract_identifier_expression(false, token, last_token_was_ident, it, is_raw_tokenstream, ct.clone())?;
                                    output.extend_from_slice(&inner);

                                    break;
                                } else {
                                    return Err(RustHtmlError::from_string(format!("unexpected '<' after punct: {:?}", token)));
                                }
                            },
                            '>' => {
                                break;
                            },
                            _ => {
                                return Err(RustHtmlError::from_string(format!("extract_identifier_expression unexpected punct: {:?}", token)));
                            }
                        }
                    },
                    TokenTree::Group(group) => {
                        let delimiter = group.delimiter();
                        if delimiter == Delimiter::Bracket {
                            if last_token_was_ident {
                                output.push(it.next().expect("it.next()").clone());
                                last_token_was_ident = false;
                            } else {
                                return Err(RustHtmlError::from_string(format!("unexpected group after punct: {:?}", token)));
                            }
                        } else {
                            break;
                        }
                    },
                    _ => {
                        return Err(RustHtmlError::from_string(format!("extract_identifier_expression unexpected token: {:?}", token)));
                    }
                }
            } else {
                break;
            }
        }

        Ok(output)
    }
}
