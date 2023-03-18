use std::collections::HashMap;

use proc_macro2::TokenTree;

pub enum RustHtmlNode {
    Text(String),
    Tag {
        tag: String,
        attributes: HashMap<String, String>,
    },
    Rust(TokenTree),
}

impl RustHtmlNode {
    // pub fn to_tokens(self: &Self) -> Vec<HtmlNodeToken> {
    //     match self {
    //         HtmlNode::Text(string) => vec![HtmlNodeToken::Text(string.to_string())],
    //         HtmlNode::Tag { tag, attributes } => vec![
    //             HtmlNodeToken::TagStart { tag: tag.to_string(), attributes: attributes.clone() },
    //             HtmlNodeToken::TagEnd(tag.to_string()),
    //         ],
    //     }
    // }

    pub fn into_token_stream(self: &Self) -> () {

    }
}