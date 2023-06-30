use std::rc::Rc;

use proc_macro2::{Delimiter, Group, Literal, Span, TokenStream, TokenTree};
use quote::quote;

use crate::view::rusthtml::rusthtml_token::{RustHtmlToken, RustHtmlIdentAndPunctOrLiteral, RustHtmlIdentOrPunct };
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use super::html_tag_parse_context::HtmlTagParseContext;
use super::irusthtml_to_rust_converter::IRustHtmlToRustConverter;
use super::peekable_rusthtmltoken::PeekableRustHtmlToken;
use super::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use super::rusthtml_parser_context::{RustHtmlParserContext, IRustHtmlParserContext};


// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs



// this is used to convert a RustHtmlToken stream into a Rust token stream.
// this is called after parsing the RustHtml language.
pub struct RustHtmlToRustConverter {
    pub context: Rc<RustHtmlParserContext>,
}

impl RustHtmlToRustConverter {
    // create a new instance of RustHtmlToRust parser.
    // context: the context to use.
    pub fn new(context: Rc<RustHtmlParserContext>) -> Self {
        Self {
            context: context,
        }
    }
}

impl IRustHtmlToRustConverter for RustHtmlToRustConverter {
    // converts a stream of RustHtml tokens to a stream of Rust tokens.
    // rusthtml_tokens: the RustHtml tokens to convert.
    // returns: the Rust tokens or an error.
    fn parse_rusthtmltokens_to_plain_rust(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError> { // , Option<Vec<TokenTree>>)
        let rusthtml_tokens = self.preprocess_rusthtmltokens(rusthtml_tokens)?;

        let mut rust_output = Vec::new();
        let it = PeekableRustHtmlToken::new(&rusthtml_tokens);
        loop 
        {
            if self.convert_rusthtmltokens_to_plain_rust(&mut rust_output, &it)? {
                break;
            }
        }
        Ok(rust_output)
    }

    // converts a RustHtml token stream to a Rust token stream.
    // output: the destination for the Rust tokens.
    // it: the iterator to use.
    // returns: whether we should break the outer loop or not, or an error.
    fn convert_rusthtmltokens_to_plain_rust(self: &Self, output: &mut Vec<TokenTree>, it: &dyn IPeekableRustHtmlToken) -> Result<bool, RustHtmlError> {
        let mut should_break_outer_loop = false;
        loop
        {
            let token_option = it.next();
            if let Some(token) = token_option {
                let break_loop = self.convert_rusthtmltoken_to_tokentree(&token, output, it)?;
                if break_loop {
                    break;
                }
            } else {
                should_break_outer_loop = true;
                break;
            }
        }

        Ok(should_break_outer_loop)
    }

    // convert a RustHtml group to Rust tokens.
    // delimiter: the delimiter of the group.
    // inner_tokens: the inner tokens of the group.
    // output: the destination for the Rust tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_rusthtmlgroupparsed_to_tokentree(self: &Self, delimiter: &Delimiter, inner_tokens: &Vec<RustHtmlToken>, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError> {
        let mut group = vec![];
        let inner_it = PeekableRustHtmlToken::new(inner_tokens);
        self.convert_rusthtmltokens_to_plain_rust(&mut group, &inner_it)?;
        output.push(TokenTree::Group(Group::new(delimiter.clone(), TokenStream::from_iter(group.iter().cloned()))));
        Ok(())
    }

    // convert a RustHtml append HTML token to Rust tokens.
    // inner: the inner tokens of the group.
    // output: the destination for the Rust tokens.
    // returns: nothing or an error.
    fn convert_rusthtmlappendhtml_to_tokentree(self: &Self, inner_as_string: Option<&String>, inner: Option<&Vec<RustHtmlToken>>, output: &mut Vec<TokenTree>) -> Result<(), RustHtmlError> {
        let mut inner_tokens = vec![];
        if let Some(inner) = inner {
            let inner_it = PeekableRustHtmlToken::new(inner);
            self.convert_rusthtmltokens_to_plain_rust(&mut inner_tokens, &inner_it)?;
            let inner_tokenstream1 = TokenStream::from_iter(inner_tokens);
            let inner_tokenstream = proc_macro2::TokenStream::from(inner_tokenstream1);
            output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { html_output.write_html((#inner_tokenstream).into()); }))));
        } else if let Some(inner_as_string) = inner_as_string {
            output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { html_output.write_html(#inner_as_string.into()); }))));
        } else {
            return Err(RustHtmlError::from_str("Both inner_as_string and inner are None"));
        }
        Ok(())
    }

    // convert RustHtml tokens to a Rust token stream.
    // token: the token to convert.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: if we should break the outer loop or not, or an error.
    fn convert_rusthtmltoken_to_tokentree(self: &Self, token: &RustHtmlToken, output: &mut Vec<TokenTree>, it: &dyn IPeekableRustHtmlToken) -> Result<bool, RustHtmlError> {
        match token {
            RustHtmlToken::Identifier(ident) => output.push(TokenTree::Ident(ident.clone())),
            RustHtmlToken::Literal(literal, string) => {
                if let Some(literal) = literal {
                    output.push(TokenTree::Literal(literal.clone()));
                } else if let Some(string) = string {
                    output.push(TokenTree::Literal(Literal::string(string)));
                } else {
                    return Err(RustHtmlError::from_string(format!("Could not handle token {:?}", token)));
                }
            },
            RustHtmlToken::ReservedChar(_, punct) => output.push(TokenTree::Punct(punct.clone())),
            RustHtmlToken::Group(_delimiter, group) => output.push(TokenTree::Group(group.clone())),
            RustHtmlToken::GroupParsed(delimiter, inner_tokens) => 
                self.convert_rusthtmlgroupparsed_to_tokentree(delimiter, inner_tokens, output, it)?,
            RustHtmlToken::HtmlTagStart(tag, tag_tokens) =>
                self.convert_rusthtmltagstart_to_tokentree(tag, tag_tokens.as_ref(), output, it)?,
            RustHtmlToken::HtmlTagVoid(tag, tag_tokens) =>
                self.convert_rusthtmltagstart_to_tokentree(tag, tag_tokens.as_ref(), output, it)?,
            RustHtmlToken::HtmlTagEnd(tag, tag_tokens) =>
                self.convert_rusthtmltagend_to_tokentree(tag, tag_tokens.as_ref(), output, it)?,
            RustHtmlToken::HtmlTagCloseStartChildrenPunct(_c, _punct) =>
                self.convert_rusthtmltagclosestartchildren_to_tokentree(output, it)?,
            RustHtmlToken::HtmlTagCloseSelfContainedPunct(_c, _punct) =>
                self.convert_rusthtmltagclosesselfcontained_to_tokentree(output, it)?,
            RustHtmlToken::HtmlTagCloseVoidPunct(_c, _punct) =>
                self.convert_rusthtmltagclosevoid_to_tokentree(output, it)?,
            RustHtmlToken::HtmlTagAttributeEquals(_c, _punct) =>
                self.convert_rusthtmltagattributeequals_to_tokentree(output, it)?,
            RustHtmlToken::HtmlTagAttributeName(tag, tag_tokens) =>
                self.convert_rusthtmltagattributename_to_tokentree(tag, tag_tokens, output, it)?,
            RustHtmlToken::HtmlTagAttributeValue(value, value_tokens) =>
                self.convert_rusthtmltagattributevalue_to_tokentree(value.as_ref(), value_tokens.as_ref(), output, it)?,
            RustHtmlToken::HtmlTextNode(text, span) => 
                self.convert_rusthtmltextnode_to_tokentree(text, span, output, it)?,
            RustHtmlToken::AppendToHtml(inner) =>
                self.convert_rusthtmlappendhtml_to_tokentree(None, Some(inner), output)?,
            RustHtmlToken::ExternalHtml(path, span) =>
                self.convert_htmlexternal_to_tokentree(path, span.clone(), output, it)?,
            _ => { return Err(RustHtmlError::from_string(format!("Could not handle token {:?}", token))); }
        }
        Ok(false)
    }

    // read and convert an external HTML file to RustHtml tokens.
    // path: the path to the external HTML file.
    // span: the span of the directive.
    // output: the destination for the Rust tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_htmlexternal_to_tokentree(self: &Self, path: &String, span: Span, output: &mut Vec<TokenTree>, it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError> {
        let content = std::fs::read_to_string(path).unwrap();
        self.convert_rusthtmltextnode_to_tokentree(&content, &span, output, it)
    }




    // all the below could be post processors, but it's easier to do it here.
    // if we use post processors then we can chain them together correctly.



    // convert a RustHtml start tag to Rust tokens.
    // tag: the tag to convert.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_rusthtmltagstart_to_tokentree(self: &Self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError> {
        let tag_as_html = format_tag(true, tag, tag_tokens);
        // println!("tag_as_html: {}", tag_as_html);
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { html_output.write_html_str(#tag_as_html); }))));
        Ok(())
    }

    // convert a RustHtml end tag to Rust tokens.
    // tag: the tag to convert.
    // output: the destination for the Rust tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_rusthtmltagend_to_tokentree(self: &Self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError> {
        let tag_as_html = format_tag(false, tag, tag_tokens);
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { html_output.write_html_str(#tag_as_html); }))));
        Ok(())
    }

    // convert a RustHtml close tag to Rust tokens.
    // output: the destination for the Rust tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_rusthtmltagclosestartchildren_to_tokentree(self: &Self, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError> {
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { html_output.write_html_str(">"); }))));
        Ok(())
    }
    
    // convert a RustHtml close tag to Rust tokens.
    // output: the destination for the Rust tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_rusthtmltagclosesselfcontained_to_tokentree(self: &Self, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError> {
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { html_output.write_html_str("/>"); }))));
        Ok(())
    }

    // convert a RustHtml close tag to Rust tokens.
    // output: the destination for the Rust tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_rusthtmltagclosevoid_to_tokentree(self: &Self, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError> {
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { html_output.write_html_str(">"); }))));
        Ok(())
    }

    // convert a RustHtml tag attribute equals punct to Rust tokens.
    // output: the destination for the Rust tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_rusthtmltagattributeequals_to_tokentree(self: &Self, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError> {
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { html_output.write_html_str("="); }))));
        Ok(())
    }

    // convert a RustHtml identifier or punct or group to Rust tokens.
    // tag: the tag to convert.
    // returns: the converted tokens or an error.
    fn convert_ident_and_punct_or_literal_to_tokenstream(self: &Self, tag: &RustHtmlIdentAndPunctOrLiteral) -> Result<TokenStream, RustHtmlError> {
        Ok(TokenStream::from_iter(match tag {
            RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(ident_and_punct) => {
                if ident_and_punct.len() == 0 {
                    return Err(RustHtmlError::from_string(format!("ident_and_punct was empty")));
                }
        
                ident_and_punct.iter()
                    .map(|x| match x {
                        RustHtmlIdentOrPunct::Ident(ident) => TokenTree::Ident(ident.clone()),
                        RustHtmlIdentOrPunct::Punct(punct) => TokenTree::Punct(punct.clone()),
                    })
                    .collect()
            },
            RustHtmlIdentAndPunctOrLiteral::Literal(literal) => vec![TokenTree::Literal(literal.clone())],
        }.iter().cloned()))
    }

    // convert a RustHtml tag attribute name to Rust tokens.
    // tag: the tag to convert.
    // output: the destination for the Rust tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_rusthtmltagattributename_to_tokentree(self: &Self, tag: &String, tag_tokens: &Option<RustHtmlIdentAndPunctOrLiteral>, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError> {
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { html_output.write_html_str(" "); }))));
        
        let tag_as_html = if let Some(tag_tokens) = tag_tokens {
            match tag_tokens {
                RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(ident_and_punct) => HtmlTagParseContext::fmt_tag_name_as_str(&ident_and_punct),
                RustHtmlIdentAndPunctOrLiteral::Literal(literal) => literal.to_string(),
            }
        } else {
            tag.to_string()
        };
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { html_output.write_html_str(#tag_as_html); }))));
        Ok(())
    }

    // convert a RustHtml tag attribute value to Rust tokens.
    // v: the value to convert.
    // output: the destination for the Rust tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_rusthtmltagattributevalue_to_tokentree(self: &Self, value: Option<&String>, value_tokens: Option<&Vec<RustHtmlToken>>, output: &mut Vec<TokenTree>, it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError> {
        self.convert_appendhtmlstring_to_tokentree("\"".to_string(), output, it)?;
        // inner tokens
        self.convert_rusthtmlappendhtml_to_tokentree(value, value_tokens, output)?;
        self.convert_appendhtmlstring_to_tokentree("\"".to_string(), output, it)?;
        Ok(())
    }

    // convert a RustHtml text node to Rust tokens by concatenating all text nodes.
    // first_text: the first text node.
    // first_span: the span of the first text node.
    // output: the destination for the Rust tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_rusthtmltextnode_to_tokentree(self: &Self, first_text: &String, _first_span: &Span, output: &mut Vec<TokenTree>, it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError> {
        let mut text_node_content = Vec::new();
        text_node_content.push(first_text.clone());

        loop {
            let peek_token_option = it.peek();
            if let Some(peek_token) = peek_token_option {
                if let RustHtmlToken::HtmlTextNode(text, _span) = peek_token {
                    text_node_content.push(text.clone());
                    it.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        let text = text_node_content.join("");
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { html_output.write_html_str(#text); }))));
        Ok(())
    }

    // generate code that appends an HTML string to the HTML output.
    // html_string: the HTML string to append.
    // output: the destination for the Rust tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_appendhtmlstring_to_tokentree(self: &Self, html_string: String, output: &mut Vec<TokenTree>, _it: &dyn IPeekableRustHtmlToken) -> Result<(), RustHtmlError> {
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { html_output.write_html_str((#html_string).into()); }))));
        Ok(())
    }

    fn preprocess_rusthtmltokens(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut rusthtml_tokens = rusthtml_tokens.clone();
        for x in self.context.get_preprocessors() {
            match x.process_rusthtml(&rusthtml_tokens) {
                Ok(tokens) => rusthtml_tokens = tokens,
                Err(RustHtmlError(e)) => return Err(RustHtmlError::from_string(e.to_string())),
            }
        }
        Ok(rusthtml_tokens)
    }
}


// format a tag as HTML. this is used to convert RustHtml tags to HTML tags.
// if tag_tokens is None, then tag is used as the tag name.
// opening: whether or not this is an opening tag.
// tag: the tag to format.
// tag_tokens: the tokens of the tag.
// returns: the tag as HTML.
fn format_tag(opening: bool, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>) -> String {
    let tag_string = if let Some(tag_tokens) = tag_tokens { HtmlTagParseContext::fmt_tag_name_as_str(tag_tokens) } else { tag.clone() };
    let tag_as_html = if opening { format!("<{}", tag_string) } else { format!("</{}>", tag_string) };
    tag_as_html
}