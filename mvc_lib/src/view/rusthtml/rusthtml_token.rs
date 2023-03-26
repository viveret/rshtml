// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
use std::collections::HashMap;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Span, Spacing, TokenStream, TokenTree};

#[derive(Clone, Debug)]
pub enum RustHtmlToken {
    // any / both
    Space(char),

    // html
    HtmlTextNode(String, Span),
    HtmlTagStart {
        tag: String,
        attributes: HashMap<String, Option<RustHtmlToken>>,
        is_self_contained_tag: bool,
    },
    HtmlTagEnd(String),

    // rust
    ExternalRustHtml(String, Span),
    ExternalHtml(String, Span),
    AppendToHtml(Vec<RustHtmlToken>),

    Literal(Literal),
    Identifier(Ident),
    ReservedChar(char, Punct),
    ReservedIndent(String, Ident),
    Group(Delimiter, Group),
    GroupParsed(Delimiter, Vec<RustHtmlToken>),
    GroupOpen(Delimiter, Span),
    GroupClose(Delimiter, Span),

    // CompileError(String), // this will output a compile error using quote!
}
