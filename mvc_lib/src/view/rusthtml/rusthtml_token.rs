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
    // GroupParsed(Vec<RustHtmlToken>),
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
    HtmlTagVoid(String, Option<Vec<RustHtmlIdentOrPunct>>),
    HtmlTagStart(String, Option<Vec<RustHtmlIdentOrPunct>>),
    HtmlTagEnd(String, Option<Vec<RustHtmlIdentOrPunct>>),
    HtmlTagAttributeName(String, Option<RustHtmlIdentAndPunctOrLiteral>),
    HtmlTagAttributeEquals(char, Option<Punct>),
    HtmlTagAttributeValue(Option<String>, Option<Vec<RustHtmlToken>>),
    HtmlTagCloseVoidPunct(char, Option<Punct>),
    HtmlTagCloseSelfContainedPunct(char, Option<Punct>),
    HtmlTagCloseStartChildrenPunct(char, Option<Punct>),

    // rust / html
    // External RustHtml file that is copied into the output
    ExternalRustHtml(String, Span),
    // External HTML file that is copied into the output
    ExternalHtml(String, Span),
    // Instruction to append to the HTML output
    AppendToHtml(Vec<RustHtmlToken>),

    Literal(Option<Literal>, Option<String>),
    Identifier(Ident),
    ReservedChar(char, Punct),
    ReservedIndent(String, Ident),
    Group(Delimiter, Group),
    GroupParsed(Delimiter, Vec<RustHtmlToken>),
    GroupOpen(Delimiter, Span),
    GroupClose(Delimiter, Span),

    // CompileError(String), // this will output a compile error using quote!
}
