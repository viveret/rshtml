// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Span};

#[derive(Clone, Debug)]
pub enum RustHtmlIdentOrPunct {
    Ident(Ident),
    Punct(Punct),
}

#[derive(Clone, Debug)]
pub enum RustHtmlIdentAndPunctOrLiteral {
    Literal(Literal),
    IdentAndPunct(Vec<RustHtmlIdentOrPunct>)
}

#[derive(Clone, Debug)]
pub enum RustHtmlIdentOrPunctOrGroup {
    Ident(Ident),
    Punct(Punct),
    Group(Group),
}

#[derive(Clone, Debug)]
pub enum RustHtmlIdentAndPunctAndGroupOrLiteral {
    Literal(Literal),
    IdentAndPunctAndGroup(Vec<RustHtmlIdentOrPunctOrGroup>)
}

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
