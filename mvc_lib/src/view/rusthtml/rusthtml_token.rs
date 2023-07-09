// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs

use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Span};

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
    HtmlTagVoid(String, Option<Vec<RustHtmlIdentOrPunct>>),
    HtmlTagStart(String, Option<Vec<RustHtmlIdentOrPunct>>),
    HtmlTagEnd(String, Option<Vec<RustHtmlIdentOrPunct>>),
    HtmlTagAttributeName(String, Option<RustHtmlIdentAndPunctOrLiteral>),
    HtmlTagAttributeEquals(char, Option<Punct>),
    HtmlTagAttributeValue(Option<String>, Option<Vec<RustHtmlIdentOrPunct>>, Option<Vec<RustHtmlToken>>),
    HtmlTagCloseVoidPunct(char, Option<Punct>),
    HtmlTagCloseSelfContainedPunct(char, Option<Punct>),
    HtmlTagCloseStartChildrenPunct(char, Option<Punct>),

    // rust / html
    // External RustHtml file that is copied into the output
    // ExternalRustHtml(String, Span),
    // External HTML file that is copied into the output
    // ExternalHtml(String, Span),
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

impl RustHtmlToken {
    // todo/fixme: this is not matching the spacing of original tokenstream.
    // look into preserving spacing field from tokentree / tokenstream.
    pub fn to_string(&self) -> String {
        // panic!("fix m e fix me fix me");
        match self {
            RustHtmlToken::Space(c) => c.to_string(),
            RustHtmlToken::HtmlTextNode(s, _) => s.to_string(),
            RustHtmlToken::HtmlTagVoid(s, _) => format!("<{} />", s.to_string()),
            RustHtmlToken::HtmlTagStart(s, _) => format!("<{}", s.to_string()),
            RustHtmlToken::HtmlTagEnd(s, _) => format!("</{}>", s.to_string()),
            RustHtmlToken::HtmlTagAttributeName(s, _) => s.to_string(),
            RustHtmlToken::HtmlTagAttributeEquals(c, _) => c.to_string(),
            RustHtmlToken::HtmlTagAttributeValue(s, _, _) => {
                match s {
                    Some(s) => s.to_string(),
                    None => "".to_string()
                }
            },
            RustHtmlToken::HtmlTagCloseVoidPunct(c, _) => c.to_string(),
            RustHtmlToken::HtmlTagCloseSelfContainedPunct(c, _) => c.to_string(),
            RustHtmlToken::HtmlTagCloseStartChildrenPunct(c, _) => c.to_string(),
            // RustHtmlToken::ExternalRustHtml(s, _) => s.to_string(),
            // RustHtmlToken::ExternalHtml(s, _) => s.to_string(),
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