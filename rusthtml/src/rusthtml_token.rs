// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
use std::collections::HashMap;

use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Span, TokenStream, TokenTree};

#[derive(Clone, Debug)]
pub enum RustHtmlToken {
    // any / both
    Space(char),

    // html
    HtmlTextNode(String),
    HtmlTagStart {
        tag: String,
        attributes: HashMap<String, Option<RustHtmlToken>>,
        is_self_contained_tag: bool,
    },
    HtmlTagEnd(String),

    // rust
    Literal(Literal),
    Identifier(Ident),
    ReservedChar(char, Punct),
    ReservedIndent(String, Ident),
    Group(Delimiter, Group),
    GroupOpen(Delimiter, Span),
    GroupClose(Delimiter, Span),

    // CompileError(String), // this will output a compile error using quote!
}
