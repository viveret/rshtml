// use std::rc::Rc;

// use proc_macro2::{Delimiter, Span, TokenStream, TokenTree, Punct, Literal};

// use crate::view::rusthtml::rusthtml_token::{RustHtmlToken, RustHtmlIdentAndPunctOrLiteral, RustHtmlIdentOrPunct };
// use crate::view::rusthtml::rusthtml_error::RustHtmlError;

// use super::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;



// // this is used to convert a RustHtmlToken stream into a Rust token stream.
// // this is called after parsing the RustHtml language.
// pub trait IRustHtmlToRustConverter {
//     fn preprocess_rusthtmltokens(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
//     fn postprocess_rusthtmltokens(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
//     fn preprocess_tokenstream(self: &Self, tokens: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError>;
//     fn postprocess_tokenstream(self: &Self, tokens: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError>;
// }
