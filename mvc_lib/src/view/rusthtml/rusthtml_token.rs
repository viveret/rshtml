// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Span};

// a RustHtml token for a Rust identifier or punctuation.
#[derive(Clone, Debug)]
pub enum RustHtmlIdentOrPunct<TIdent, TPunct> {
    Ident(TIdent),
    Punct(TPunct),
}

// a RustHtml token for a Rust identifier or punctuation or literal.
#[derive(Clone, Debug)]
pub enum RustHtmlIdentAndPunctOrLiteral<TIdent, TPunct, TLiteral> {
    Literal(TLiteral),
    IdentAndPunct(Vec<RustHtmlIdentOrPunct<TIdent, TPunct>>)
}

// a RustHtml token for a Rust identifier or punctuation or group.
#[derive(Clone, Debug)]
pub enum RustHtmlIdentOrPunctOrGroup<TIdent, TPunct, TGroup> {
    Ident(TIdent),
    Punct(TPunct),
    Group(TGroup),
    // GroupParsed(Vec<RustHtmlToken<Ident, Punct, Literal>>),
}

// a RustHtml token for a Rust identifier or punctuation or group or literal.
#[derive(Clone, Debug)]
pub enum RustHtmlIdentAndPunctAndGroupOrLiteral<TLiteral, TIdent, TPunct, TGroup> {
    Literal(TLiteral),
    IdentAndPunctAndGroup(Vec<RustHtmlIdentOrPunctOrGroup<TIdent, TPunct, TGroup>>)
}

// The token types for the RustHtml language.
// Each enum variant represents a different part of the RustHtml language.
#[derive(Clone, Debug)]
pub enum RustHtmlToken<TIdent, TPunct, TLiteral> {
    // any / both
    Space(char),

    // html
    HtmlTextNode(String, Span),
    HtmlTagVoid(String, Option<Vec<RustHtmlIdentOrPunct<TIdent, TPunct>>>),
    HtmlTagStart(String, Option<Vec<RustHtmlIdentOrPunct<TIdent, TPunct>>>),
    HtmlTagEnd(String, Option<Vec<RustHtmlIdentOrPunct<TIdent, TPunct>>>),
    HtmlTagAttributeName(String, Option<RustHtmlIdentAndPunctOrLiteral<TLiteral, TIdent, TPunct>>),
    HtmlTagAttributeEquals(char, Option<Punct>),
    HtmlTagAttributeValue(Option<String>, Option<Vec<RustHtmlToken<TIdent, TPunct, TLiteral>>>),
    HtmlTagCloseVoidPunct(char, Option<Punct>),
    HtmlTagCloseSelfContainedPunct(char, Option<Punct>),
    HtmlTagCloseStartChildrenPunct(char, Option<Punct>),

    // rust / html
    // External RustHtml file that is copied into the output
    ExternalRustHtml(String, Span),
    // External HTML file that is copied into the output
    ExternalHtml(String, Span),
    // Instruction to append to the HTML output
    AppendToHtml(Vec<RustHtmlToken<TIdent, TPunct, TLiteral>>),

    Literal(Option<Literal>, Option<String>),
    Identifier(Ident),
    ReservedChar(char, Punct),
    ReservedIndent(String, Ident),
    Group(Delimiter, Group),
    GroupParsed(Delimiter, Vec<RustHtmlToken<TIdent, TPunct, TLiteral>>),
    GroupOpen(Delimiter, Span),
    GroupClose(Delimiter, Span),

    // CompileError(String), // this will output a compile error using quote!
}

impl<TIdent, TPunct, TLiteral> RustHtmlToken<TIdent, TPunct, TLiteral> {
    pub fn to_string(&self) -> String {
        match self {
            RustHtmlToken::Space(c) => c.to_string(),
            RustHtmlToken::HtmlTextNode(s, _) => s.to_string(),
            RustHtmlToken::HtmlTagVoid(s, _) => format!("<{} />", s.to_string()),
            RustHtmlToken::HtmlTagStart(s, _) => format!("<{}", s.to_string()),
            RustHtmlToken::HtmlTagEnd(s, _) => format!("</{}>", s.to_string()),
            RustHtmlToken::HtmlTagAttributeName(s, _) => s.to_string(),
            RustHtmlToken::HtmlTagAttributeEquals(c, _) => c.to_string(),
            RustHtmlToken::HtmlTagAttributeValue(s, _) => {
                match s {
                    Some(s) => s.to_string(),
                    None => "".to_string()
                }
            },
            RustHtmlToken::HtmlTagCloseVoidPunct(c, _) => c.to_string(),
            RustHtmlToken::HtmlTagCloseSelfContainedPunct(c, _) => c.to_string(),
            RustHtmlToken::HtmlTagCloseStartChildrenPunct(c, _) => c.to_string(),
            RustHtmlToken::ExternalRustHtml(s, _) => s.to_string(),
            RustHtmlToken::ExternalHtml(s, _) => s.to_string(),
            RustHtmlToken::AppendToHtml(tokens) => tokens.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(" "),
            RustHtmlToken::Literal(l, _) => {
                match l {
                    Some(l) => l.to_string(),
                    None => "".to_string()
                }
            },
            RustHtmlToken::Identifier(ident) => ident.to_string(),
            RustHtmlToken::ReservedChar(c, _) => c.to_string(),
            RustHtmlToken::ReservedIndent(s, _) => s.to_string(),
            RustHtmlToken::Group(_, group) => group.to_string(),
            RustHtmlToken::GroupParsed(delimiter, tokens) => {
                let inner = tokens.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(" ");
                match delimiter {
                    Delimiter::Brace => format!("{{ {} }}", inner),
                    Delimiter::Bracket => format!("[ {} ]", inner),
                    Delimiter::Parenthesis => format!("( {} )", inner),
                    Delimiter::None => inner,
                }
            },
            RustHtmlToken::GroupOpen(delimiter, _) => {
                match delimiter {
                    Delimiter::Brace => "{".to_string(),
                    Delimiter::Bracket => "[".to_string(),
                    Delimiter::Parenthesis => "(".to_string(),
                    Delimiter::None => "".to_string(),
                }
            },
            RustHtmlToken::GroupClose(delimiter, _) => {
                match delimiter {
                    Delimiter::Brace => "}".to_string(),
                    Delimiter::Bracket => "]".to_string(),
                    Delimiter::Parenthesis => ")".to_string(),
                    Delimiter::None => "".to_string(),
                }
            },
        }
    }
}