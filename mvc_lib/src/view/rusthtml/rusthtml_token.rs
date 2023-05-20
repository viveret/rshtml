// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Span};

// a RustHtml token for a Rust identifier or punctuation.
#[derive(Clone, Debug)]
pub enum RustHtmlIdentOrPunct {
    Ident(Ident),
    Punct(Punct),
}

// a RustHtml token for a Rust identifier or punctuation or literal.
#[derive(Clone, Debug)]
pub enum RustHtmlIdentAndPunctOrLiteral {
    Literal(Literal),
    IdentAndPunct(Vec<RustHtmlIdentOrPunct>)
}

// a RustHtml token for a Rust identifier or punctuation or group.
#[derive(Clone, Debug)]
pub enum RustHtmlIdentOrPunctOrGroup {
    Ident(Ident),
    Punct(Punct),
    Group(Group),
}

// a RustHtml token for a Rust identifier or punctuation or group or literal.
#[derive(Clone, Debug)]
pub enum RustHtmlIdentAndPunctAndGroupOrLiteral {
    Literal(Literal),
    IdentAndPunctAndGroup(Vec<RustHtmlIdentOrPunctOrGroup>)
}

// The token types for the RustHtml language.
// Each enum variant represents a different part of the RustHtml language.
#[derive(Clone, Debug)]
pub enum RustHtmlToken {
    // any / both
    Space(char),

    // html
    HtmlTextNode(String, Span),
    HtmlTagVoid(String, Vec<RustHtmlIdentOrPunct>),
    HtmlTagStart(String, Vec<RustHtmlIdentOrPunct>),
    HtmlTagEnd(String, Vec<RustHtmlIdentOrPunct>),
    HtmlTagAttributeName(String, RustHtmlIdentAndPunctOrLiteral),
    HtmlTagAttributeEquals(Punct),
    HtmlTagAttributeValue(Vec<RustHtmlToken>),
    HtmlTagCloseVoidPunct(Punct),
    HtmlTagCloseSelfContainedPunct(Punct),
    HtmlTagCloseStartChildrenPunct(Punct),

    // rust / html
    // External RustHtml file that is copied into the output
    ExternalRustHtml(String, Span),
    // External HTML file that is copied into the output
    ExternalHtml(String, Span),
    // Instruction to append to the HTML output
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
