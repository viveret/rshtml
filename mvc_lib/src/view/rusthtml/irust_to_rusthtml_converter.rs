// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
use std::rc::Rc;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, TokenStream, TokenTree};

use crate::view::rusthtml::rusthtml_token::RustHtmlIdentOrPunctOrGroup;
use crate::view::rusthtml::rusthtml_token::RustHtmlIdentAndPunctAndGroupOrLiteral;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use super::html_tag_parse_context::HtmlTagParseContext;
use super::peekable_tokentree::IPeekableTokenTree;
use super::rusthtml_parser_context::IRustHtmlParserContext;


// this is used to parse the RustHtml language that is in Rust TokenTree tokens into a RustHtmlToken stream of RustHtml tokens.
// this is called before converting the RustHtml tokens back to Rust tokens.
pub trait IRustToRustHtmlConverter {
    fn expand_external_tokenstream(self: &Self, path_str: &String, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>) -> Result<(), RustHtmlError>;
    fn parse_tokenstream_to_rusthtmltokens(self: &Self, is_in_html_mode: bool, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken<Ident, Punct, Literal>>, RustHtmlError>;
    fn parse_string_with_quotes(self: &Self, identifier: Ident, it: Rc<dyn IPeekableTokenTree>) -> Result<String, RustHtmlError>;
    fn parse_identifier_expression(self: &Self, identifier: Ident, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError>;
    fn parse_type_identifier(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn loop_next_and_convert(self: &Self, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError>;
    fn next_and_convert(self: &Self, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<bool, RustHtmlError>;

    fn convert_copy(self: &Self, token: TokenTree, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>) -> Result<(), RustHtmlError>;
    fn convert_path_str(self: &Self, identifier: Ident, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<String, RustHtmlError>;
    fn convert_views_path_str(self: &Self, identifier: Ident, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<String, RustHtmlError>;
    fn resolve_views_path_str(self: &Self, path: &str) -> Result<String, RustHtmlError>;
    fn convert_string_or_ident(self: &Self, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<RustHtmlIdentAndPunctAndGroupOrLiteral, RustHtmlError>;
    fn convert_tokentree_to_rusthtmltoken(self: &Self, token: TokenTree, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<bool, RustHtmlError>;
    fn convert_punct_to_rusthtmltoken(self: &Self, punct: Punct, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<bool, RustHtmlError>;
    fn convert_html_entry_to_rusthtmltoken(self: &Self, c: char, punct: Punct, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError>;
    fn convert_html_ident_to_rusthtmltoken(self: &Self, ident: &Ident, ctx: &mut HtmlTagParseContext, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError>;
    fn convert_html_punct_to_rusthtmltoken(self: &Self, punct: &Punct, ctx: &mut HtmlTagParseContext, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<bool, RustHtmlError>;
    fn convert_html_literal_to_rusthtmltoken(self: &Self, literal: &Literal, parse_ctx: &mut HtmlTagParseContext, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError>;
    fn convert_ident_and_punct_and_group_or_literal_to_tokenstream(self: &Self, tag: &RustHtmlIdentAndPunctAndGroupOrLiteral) -> Result<TokenStream, RustHtmlError>;
    fn convert_group_to_rusthtmltoken(self: &Self, group: Group, expect_return_html: bool, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError>;
    fn convert_rust_entry_to_rusthtmltoken(self: &Self, c: char, punct: Punct, is_in_html_mode: bool, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError>;
    fn convert_rust_directive_to_rusthtmltoken(self: &Self, token: TokenTree, prefix_token_option: Option<RustHtmlToken<Ident, Punct, Literal>>, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<bool, RustHtmlError>;
    fn convert_rust_directive_identifier_to_rusthtmltoken(self: &Self, identifier: Ident, prefix_token_option: Option<RustHtmlToken<Ident, Punct, Literal>>, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError>;
    fn convert_rust_directive_group_to_rusthtmltoken(self: &Self, group: Group, prefix_token_option: Option<RustHtmlToken<Ident, Punct, Literal>>, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, is_raw_tokenstream: bool) -> Result<(), RustHtmlError>;
    fn convert_rusthtmltokens_to_ident_or_punct_or_group(self: &Self, rusthtml_tokens: Vec<RustHtmlToken<Ident, Punct, Literal>>) -> Result<Vec<RustHtmlIdentOrPunctOrGroup>, RustHtmlError>;

    fn is_start_of_current_expression(self: &Self, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>) -> bool;
    fn next_and_parse_html_tag(self: &Self, token_option: Option<TokenTree>, ctx: &mut HtmlTagParseContext, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<bool, RustHtmlError>;
    fn on_html_tag_parsed(self: &Self, punct: &Punct, parse_ctx: &mut HtmlTagParseContext, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>) -> Result<bool, RustHtmlError>;
    fn on_html_node_parsed(self: &Self, ctx: &HtmlTagParseContext, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>) -> Result<bool, RustHtmlError>;
    fn on_kvp_defined(self: &Self, ctx: &mut HtmlTagParseContext, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>);
    fn get_opening_delim(self: &Self, delim: Delimiter) -> &'static str;
    fn get_closing_delim(self: &Self, delim: Delimiter) -> &'static str;

    fn get_context(self: &Self) -> Rc<dyn IRustHtmlParserContext>;
}
