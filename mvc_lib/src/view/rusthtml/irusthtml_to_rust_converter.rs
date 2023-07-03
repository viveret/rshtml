use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};

use crate::view::rusthtml::rusthtml_token::{RustHtmlToken, RustHtmlIdentAndPunctOrLiteral, RustHtmlIdentOrPunct };
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use super::peekable_rusthtmltoken::IPeekableRustHtmlToken;


// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs



// this is used to convert a RustHtmlToken stream into a Rust token stream.
// this is called after parsing the RustHtml language.
pub trait IRustHtmlToRustConverter {
    fn preprocess_rusthtmltokens(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn postprocess_rusthtmltokens(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn preprocess_tokenstream(self: &Self, tokens: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn postprocess_tokenstream(self: &Self, tokens: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError>;

    fn parse_rusthtmltokens_to_plain_rust(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_rusthtmltoken_to_tokentree(self: &Self, token: &RustHtmlToken, output: &mut Vec<TokenTree>, it: &dyn IPeekableRustHtmlToken) -> Result<bool, RustHtmlError>;
    fn convert_rusthtmltokens_to_plain_rust(self: &Self, output: &mut Vec<TokenTree>, it: &dyn IPeekableRustHtmlToken) -> Result<bool, RustHtmlError>;
    fn convert_rusthtmlgroupparsed_to_tokentree(self: &Self, delimiter: &Delimiter, inner_tokens: &Vec<RustHtmlToken>, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError>;
    fn convert_rusthtmlappendhtml_to_tokentree(self: &Self, inner_as_string: Option<&String>, inner: Option<&Vec<RustHtmlToken>>, output: &mut Vec<TokenTree>) -> Result<(), RustHtmlError>;
    fn convert_rusthtmltextnode_to_tokentree(self: &Self, first_text: &String, _first_span: &Span, output: &mut Vec<TokenTree>, it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError>;
    fn convert_rusthtmltagstart_to_tokentree(self: &Self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError>;
    fn convert_rusthtmltagend_to_tokentree(self: &Self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError>;
    fn convert_rusthtmltagclosestartchildren_to_tokentree(self: &Self, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError>;
    fn convert_rusthtmltagclosesselfcontained_to_tokentree(self: &Self, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError>;
    fn convert_rusthtmltagclosevoid_to_tokentree(self: &Self, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError>;
    fn convert_rusthtmltagattributeequals_to_tokentree(self: &Self, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError>;
    fn convert_ident_and_punct_or_literal_to_tokenstream(self: &Self, tag: &RustHtmlIdentAndPunctOrLiteral) -> Result<TokenStream, RustHtmlError>;
    fn convert_rusthtmltagattributename_to_tokentree(self: &Self, tag: &String, tag_tokens: &Option<RustHtmlIdentAndPunctOrLiteral>, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError>;
    fn convert_rusthtmltagattributevalue_to_tokentree(self: &Self, value: Option<&String>, value_tokens: Option<&Vec<RustHtmlToken>>, output: &mut Vec<TokenTree>, it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError>;
    fn convert_appendhtmlstring_to_tokentree(self: &Self, html_string: String, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError>;
}
