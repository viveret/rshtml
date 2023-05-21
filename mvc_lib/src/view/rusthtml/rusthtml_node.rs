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