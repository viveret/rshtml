use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Delimiter;
use proc_macro2::Group;
use proc_macro2::Literal;
use proc_macro2::Punct;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::ToTokens;

use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::ipeekable_tokentree::IPeekableTokenTree;
use crate::view::parserv3::core::peekable::vec_peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::vec_peekable_tokentree::VecPeekableTokenTree;
use crate::view::rusthtml::rusthtml_token::RustHtmlIdentAndPunctOrLiteral;
use crate::view::rusthtml::rusthtml_token::RustHtmlIdentOrPunct;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

pub trait IConverterOutput {
    fn convert(&self, input: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Rc<dyn IPeekableTokenTree>, RustHtmlError>;


    fn parse_rusthtmltokens_to_plain_rust(&self, rusthtml_tokens: &Vec<RustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_rusthtmltoken_to_tokentree(&self, token: &RustHtmlToken, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltokens_to_plain_rust(&self, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmlgroupparsed_to_tokentree(&self, delimiter: &Delimiter, inner_tokens: &Vec<RustHtmlToken>, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmlappendhtml_to_tokentree(&self, inner_as_string: Option<&String>, inner_as_literal: Option<&Literal>, inner_as_ident: Option<&Vec<RustHtmlIdentOrPunct>>, inner: Option<&Vec<RustHtmlToken>>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltextnode_to_tokentree(&self, first_text: &String, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltagstart_to_tokentree(&self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltagvoid_to_tokentree(&self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltagend_to_tokentree(&self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltagclosestartchildren_to_tokentree(&self, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltagclosesselfcontained_to_tokentree(&self, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltagclosevoid_to_tokentree(&self, tag_string: String, c: Option<(char, Punct)>, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltagattributeequals_to_tokentree(&self, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_ident_and_punct_or_literal_to_tokenstream(&self, tag: &RustHtmlIdentAndPunctOrLiteral, ct: Rc<dyn ICancellationToken>) -> Result<TokenStream, RustHtmlError>;
    fn convert_rusthtmltagattributename_to_tokentree(&self, tag: &String, tag_tokens: &Option<RustHtmlIdentAndPunctOrLiteral>, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltagattributevalue_to_tokentree(&self, 
        value: Option<&String>,
        value_literal: Option<&Literal>,
        value_tokens: Option<&Vec<RustHtmlToken>>,
        value_tokens_special: Option<&RustHtmlIdentAndPunctOrLiteral>,
        it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_appendhtmlstring_to_tokentree(&self, html_string: String, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;
    fn convert_ident_or_punct_to_tokenstream(&self, name: &Vec<RustHtmlIdentOrPunct>, ct: Rc<dyn ICancellationToken>) -> Result<TokenStream, RustHtmlError>;

    fn format_tag_open(&self, tag_string: &String, ct: Rc<dyn ICancellationToken>) -> String;
    fn format_tag_close(&self, tag_string: &String, ct: Rc<dyn ICancellationToken>) -> String;
    fn format_tag_name_as_string(&self, tag_tokens: &Vec<RustHtmlIdentOrPunct>, ct: Rc<dyn ICancellationToken>) -> String;
}

pub struct ConverterOutput {
}

impl ConverterOutput {
    pub fn new() -> Self {
        Self {
        }
    }
    
    // format a tag as HTML. this is used to convert RustHtml tags to HTML tags.
    // if tag_tokens is None, then tag is used as the tag name.
    // opening: whether or not this is an opening tag.
    // tag: the tag to format.
    // tag_tokens: the tokens of the tag.
    // returns: the tag as HTML.
    fn format_tag(opening: bool, tag: &String, is_void: bool, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>) -> String {
        let tag_string = 
            if let Some(tag_tokens) = tag_tokens {
                if tag_tokens.is_empty() {
                    tag.clone()
                } else {
                    RustHtmlIdentOrPunct::to_string_join(tag_tokens)
                }
            } else {
                tag.clone()
            };
        if tag_string.is_empty() {
            panic!("tag is empty");
        }
        
        if opening {
            // if is_void {
            //     let mut has_slash = true;
            //     match tag_string.as_str() {
            //         "!DOCTYPE" => has_slash = false,
            //         _ => (),
            //     }
            //     format!("<{}{}>", tag_string, if has_slash { "/" } else { "" })
            // } else {
                format!("<{}", tag_string)
            // }
        } else {
            format!("</{}>", tag_string)
        }
    }

    pub fn write_html_stream(&self, ident_tokenstream: TokenStream) -> TokenStream {
        TokenStream::from(quote::quote! { html_output.write_html(HtmlString::from(#ident_tokenstream)); })
    }
    
    fn convert_token(&self, token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        if ct.is_cancelled() {
            return Err(RustHtmlError::from_cancellationtoken(ct));
        }
        match token {
            // help: message: unsupported character `' '`
            // RustHtmlToken::Space(space) => {
            //     output.push(TokenTree::Punct(Punct::new(space.clone(), Spacing::Alone)));
            // }
            RustHtmlToken::Identifier(ident) => Ok(TokenTree::Ident(ident.clone())),
            RustHtmlToken::Literal(literal, string) =>
                if let Some(literal) = literal {
                    Ok(TokenTree::Literal(literal.clone()))
                } else if let Some(string) = string {
                    Ok(TokenTree::Literal(Literal::string(string)))
                } else {
                    Err(RustHtmlError::from_string(format!("Token is neither literal or string: {:?}", token)))
                }
            ,
            RustHtmlToken::ReservedChar(_, punct) => Ok(TokenTree::Punct(punct.clone())),
            RustHtmlToken::Group(delimiter, stream, group) => {
                if let Some(g) = group {
                    Ok(TokenTree::Group(g.clone()))
                } else {
                    let tokenstream = self.convert(stream.clone(), ct)?;
                    Ok(TokenTree::Group(Group::new(*delimiter, tokenstream.to_stream())))
                }
            },
            RustHtmlToken::GroupParsed(delimiter, inner_tokens) => 
                self.convert_rusthtmlgroupparsed_to_tokentree(delimiter, inner_tokens, it, ct),
            RustHtmlToken::HtmlTagStart(tag, tag_tokens) =>
                self.convert_rusthtmltagstart_to_tokentree(tag, tag_tokens.as_ref(), it, ct),
            RustHtmlToken::HtmlTagVoid(tag, tag_tokens) =>
                self.convert_rusthtmltagvoid_to_tokentree(tag, tag_tokens.as_ref(), it, ct),
            RustHtmlToken::HtmlTagEnd(tag, tag_tokens) =>
                self.convert_rusthtmltagend_to_tokentree(tag, tag_tokens.as_ref(), it, ct),
            RustHtmlToken::HtmlTagCloseStartChildrenPunct =>
                self.convert_rusthtmltagclosestartchildren_to_tokentree(it, ct),
            RustHtmlToken::HtmlTagCloseSelfContainedPunct =>
                self.convert_rusthtmltagclosesselfcontained_to_tokentree(it, ct),
            RustHtmlToken::HtmlTagCloseVoidPunct(tag, c) =>
                self.convert_rusthtmltagclosevoid_to_tokentree(tag.clone(), c.clone(), it, ct),
            RustHtmlToken::HtmlTagAttributeEquals(_c, _punct) =>
                self.convert_rusthtmltagattributeequals_to_tokentree(it, ct),
            RustHtmlToken::HtmlTagAttributeName(tag, tag_tokens) =>
                self.convert_rusthtmltagattributename_to_tokentree(tag, tag_tokens, it, ct),
            RustHtmlToken::HtmlTagAttributeValue(value_string, value_literal, value_rust_tokens, value_tokens) =>
                self.convert_rusthtmltagattributevalue_to_tokentree(value_string.as_ref(), value_literal.as_ref(), value_tokens.as_ref(), value_rust_tokens.as_ref(), it, ct),
            RustHtmlToken::HtmlTextNode(text) => 
                self.convert_rusthtmltextnode_to_tokentree(text, it, ct),
            RustHtmlToken::AppendToHtml(inner) =>
                self.convert_rusthtmlappendhtml_to_tokentree(None, None, None, Some(inner), ct),
            _ => { Err(RustHtmlError::from_string(format!("Could not handle token {:?}", token))) }
        }
    }

    // convert a RustHtml group to Rust tokens.
    // delimiter: the delimiter of the group.
    // inner_tokens: the inner tokens of the group.
    // output: the destination for the Rust tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_rusthtmlgroupparsed_to_tokentree(&self, delimiter: &Delimiter, inner_tokens: &Vec<RustHtmlToken>, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        let inner_it = Rc::new(VecPeekableRustHtmlToken::new(inner_tokens.clone()));
        let group = self.convert_rusthtmltokens_to_plain_rust(inner_it, ct)?;
        if let TokenTree::Group(group) = group {
            let group_inner = group.stream();
            Ok(TokenTree::Group(Group::new(delimiter.clone(), group_inner)))
        } else {
            Err(RustHtmlError::from_string(format!("Expected group, got {:?}", group)))
        }
    }
    
    fn convert_rusthtmlgroup_to_tokentree(&self, d: &Delimiter, s: Rc<dyn IPeekableRustHtmlToken>, g: &Option<Group>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        if let Some(group) = g {
            Ok(TokenTree::Group(group.clone()))
        } else {
            Ok(TokenTree::Group(Group::new(d.clone(), self.convert(s, ct)?.to_stream())))
        }
    }
}

impl IConverterOutput for ConverterOutput {
    fn convert(&self, input: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Rc<dyn IPeekableTokenTree>, RustHtmlError> {
        let mut output: Vec<TokenTree> = vec![];
        loop {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_cancellationtoken(ct))
            }

            let token = input.next();
            if let Some(token) = token {
                // todo: check for empty group
                output.push(self.convert_token(&token, input.clone(), ct.clone())?);
            } else {
                break;
            }
        }
        Ok(Rc::new(VecPeekableTokenTree::new(output)))
    }
    
    fn parse_rusthtmltokens_to_plain_rust(&self, rusthtml_tokens: &Vec<RustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let it = Rc::new(VecPeekableRustHtmlToken::new(rusthtml_tokens.clone()));
        let result = self.convert(it, ct)?;
        Ok(result.to_vec())
    }
    
    fn convert_rusthtmltoken_to_tokentree(&self, token: &RustHtmlToken, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        match token {
            RustHtmlToken::AppendToHtml(tokens) => todo!("convert_rusthtmltoken_to_tokentree for AppendToHtml"),
            RustHtmlToken::Group(d, s, g) => self.convert_rusthtmlgroup_to_tokentree(d, s.clone(), g, ct),
            RustHtmlToken::GroupClose(d, s) => todo!("convert_rusthtmltoken_to_tokentree for GroupClose"),
            RustHtmlToken::GroupOpen(d, s) => todo!("convert_rusthtmltoken_to_tokentree for GroupOpen"),
            RustHtmlToken::GroupParsed(d, tokens) => todo!("convert_rusthtmltoken_to_tokentree for GroupParsed"),
            RustHtmlToken::HtmlTagAttributeEquals(a, x) => todo!("convert_rusthtmltoken_to_tokentree for HtmlTagAttributeEquals"),
            RustHtmlToken::HtmlTagAttributeName(a,x ) => todo!("convert_rusthtmltoken_to_tokentree for HtmlTagAttributeName"),
            RustHtmlToken::HtmlTagAttributeValue(a, x, v, c) => todo!("convert_rusthtmltoken_to_tokentree for HtmlTagAttributeValue"),
            RustHtmlToken::HtmlTagCloseSelfContainedPunct => todo!("convert_rusthtmltoken_to_tokentree for HtmlTagCloseSelfContainedPunct"),
            RustHtmlToken::HtmlTagCloseStartChildrenPunct => todo!("convert_rusthtmltoken_to_tokentree for HtmlTagCloseStartChildrenPunct"),
            RustHtmlToken::HtmlTagCloseVoidPunct(_tag, x) => todo!("convert_rusthtmltoken_to_tokentree for HtmlTagCloseVoidPunct"),
            RustHtmlToken::HtmlTagEnd(x, e) => todo!("convert_rusthtmltoken_to_tokentree for HtmlTagEnd"),
            RustHtmlToken::HtmlTagStart(x, e) => todo!("convert_rusthtmltoken_to_tokentree for HtmlTagStart"),
            RustHtmlToken::HtmlTagVoid(x, e) => todo!("convert_rusthtmltoken_to_tokentree for HtmlTagVoid"),
            RustHtmlToken::HtmlTextNode(s) => todo!("convert_rusthtmltoken_to_tokentree for HtmlTextNode"),
            RustHtmlToken::Identifier(i) => Ok(TokenTree::Ident(i.clone())),
            RustHtmlToken::Literal(l, s) => Ok(TokenTree::Literal(l.clone().unwrap())),
            RustHtmlToken::ReservedChar(_, p) => Ok(TokenTree::Punct(p.clone())),
            RustHtmlToken::ReservedIndent(_, i) => Ok(TokenTree::Ident(i.clone())),
            RustHtmlToken::Space(c) => todo!("convert_rusthtmltoken_to_tokentree for Space")
        }
    }
    
    fn convert_rusthtmltokens_to_plain_rust(&self, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        let mut output = vec![];
        while let Some(token) = it.next() {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_cancellationtoken(ct))
            }
            output.push(self.convert_rusthtmltoken_to_tokentree(&token, ct.clone())?);
        }

        if output.len() == 1 {
            Ok(output.first().unwrap().clone())
        } else {
            Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from_iter(output))))
        }
    }
    
    fn convert_rusthtmlgroupparsed_to_tokentree(&self, delimiter: &Delimiter, inner_tokens: &Vec<RustHtmlToken>, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        todo!("convert_rusthtmlgroupparsed_to_tokentree")
    }
    
    fn convert_rusthtmlappendhtml_to_tokentree(&self, inner_as_string: Option<&String>, inner_as_literal: Option<&Literal>, inner_as_ident: Option<&Vec<RustHtmlIdentOrPunct>>, inner: Option<&Vec<RustHtmlToken>>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        if let Some(s) = inner_as_string {
            Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#s); }))))
        } else if let Some(l) = inner_as_literal {
            Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#l); }))))
        } else if let Some(i) = inner_as_ident {
            let ident = self.convert_ident_or_punct_to_tokenstream(i, ct)?;
            // Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#ident); }))))
            Ok(TokenTree::Group(Group::new(Delimiter::None, self.write_html_stream(ident))))
        } else if let Some(inner_tokens) = inner {
            let inner_it = Rc::new(VecPeekableRustHtmlToken::new(inner_tokens.clone()));
            let x = self.convert_rusthtmltokens_to_plain_rust(inner_it, ct)?;
            let inner_tokenstream = x.to_token_stream();
            Ok(TokenTree::Group(Group::new(Delimiter::None, self.write_html_stream(inner_tokenstream))))
        } else {
            Err(RustHtmlError::from_str("None of the append to html options were set (string, literal, ident, or tokens)"))
        }


        /*
        
        let mut inner_tokens = vec![];
        if let Some(inner) = inner {
            if inner.len() == 1 {
                match inner.first().unwrap() {
                    RustHtmlToken::Literal(literal, literal_string) => {
                        if let Some(literal_string) = literal_string {
                            output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { html_output.write_html_str(#literal_string); }))));
                        } else {
                            output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { html_output.write_html_str(#literal); }))));
                        }
                        return Ok(());
                    },
                    _ => {
                    }
                }
            }
            
            let inner_it = PeekableRustHtmlToken::new(inner);
            self.convert_rusthtmltokens_to_plain_rust(&mut inner_tokens, &inner_it)?;
            let inner_tokenstream1 = TokenStream::from_iter(inner_tokens);
            let inner_tokenstream = proc_macro2::TokenStream::from(inner_tokenstream1);
            output.push(TokenTree::Group(Group::new(Delimiter::None, self.write_html_stream(inner_tokenstream))));
        } else if let Some(inner_as_ident) = inner_as_ident {
            let ident_tokenstream = TokenStream::from_iter(
                inner_as_ident.iter()
                .map(|x| {
                    match x {
                        RustHtmlIdentOrPunct::Ident(ident) => TokenTree::Ident(ident.clone()),
                        RustHtmlIdentOrPunct::Punct(punct) => TokenTree::Punct(punct.clone()),
                    }
                })
                .collect::<Vec<TokenTree>>()
            );
            output.push(TokenTree::Group(Group::new(Delimiter::None, self.write_html_stream(ident_tokenstream))));
        } else if let Some(inner_as_literal) = inner_as_literal {
            output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { html_output.write_html_str(#inner_as_literal); }))));
        } else if let Some(inner_as_string) = inner_as_string {
            output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote! { html_output.write_html_str(#inner_as_string); }))));
        } else {
            return Err(RustHtmlError::from_str("Both inner_as_string and inner are None"));
        }
        Ok(())
        
        
         */
    }
    
    fn convert_rusthtmltextnode_to_tokentree(&self, first_text: &String, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        let mut text_node_content = Vec::new();
        text_node_content.push(first_text.clone());

        loop {
            let peek_token_option = it.peek();
            if let Some(peek_token) = peek_token_option {
                if let RustHtmlToken::HtmlTextNode(text) = peek_token {
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
        Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#text); }))))
    }
    
    fn convert_rusthtmltagstart_to_tokentree(&self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        let tag_as_html = Self::format_tag(true, tag, false, tag_tokens);
        Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#tag_as_html); }))))
    }
    
    fn convert_rusthtmltagvoid_to_tokentree(&self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        let tag_as_html = Self::format_tag(true, tag, true, tag_tokens);
        Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#tag_as_html); }))))
    }
    
    fn convert_rusthtmltagend_to_tokentree(&self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        let tag_as_html = Self::format_tag(false, tag, false, tag_tokens);
        Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#tag_as_html); }))))
    }
    
    fn convert_rusthtmltagclosestartchildren_to_tokentree(&self, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(">"); }))))
    }
    
    fn convert_rusthtmltagclosesselfcontained_to_tokentree(&self, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(">"); }))))
    }
    
    fn convert_rusthtmltagclosevoid_to_tokentree(&self, tag_string: String, c: Option<(char, Punct)>, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        let mut has_slash = true;
        match tag_string.as_str() {
            "!DOCTYPE" => has_slash = false,
            _ => (),
        }
        let s = if has_slash { "/>" } else { ">" };
        Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#s); }))))
    }
    
    fn convert_rusthtmltagattributeequals_to_tokentree(&self, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str("="); }))))
    }
    
    fn convert_ident_and_punct_or_literal_to_tokenstream(&self, _tag: &RustHtmlIdentAndPunctOrLiteral, ct: Rc<dyn ICancellationToken>) -> Result<TokenStream, RustHtmlError> {
        todo!("convert_ident_and_punct_or_literal_to_tokenstream")
    }
    
    fn convert_ident_or_punct_to_tokenstream(&self, name: &Vec<RustHtmlIdentOrPunct>, ct: Rc<dyn ICancellationToken>) -> Result<TokenStream, RustHtmlError> {
        todo!("convert_ident_and_punct_or_literal_to_tokenstream")
    }
    
    fn convert_rusthtmltagattributename_to_tokentree(&self, tag: &String, _tag_tokens: &Option<RustHtmlIdentAndPunctOrLiteral>, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { 
            html_output.write_html_str(" ");
            html_output.write_html_str(#tag);
        }))))
    }
    
    fn convert_rusthtmltagattributevalue_to_tokentree(&self, 
        value: Option<&String>,
        value_literal: Option<&Literal>,
        value_tokens: Option<&Vec<RustHtmlToken>>,
        value_tokens_special: Option<&RustHtmlIdentAndPunctOrLiteral>,
        it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<TokenTree, RustHtmlError> {
        if let Some(value) = value {
            Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#value); }))))
        } else if let Some(value_literal) = value_literal {
            let s = value_literal.to_string();
            let s = format!("\"{}\"", html_escape::encode_double_quoted_attribute(&s));
            // let s = value_literal.to_token_stream();
            Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#s); }))))
        } else if let Some(value_tokens) = value_tokens {
            if value_tokens.len() == 1 {
                match value_tokens.first().unwrap() {
                    RustHtmlToken::Space(_) => todo!("convert_rusthtmltagattributevalue_to_tokentree space"),
                    RustHtmlToken::HtmlTextNode(_) => todo!("convert_rusthtmltagattributevalue_to_tokentree "),
                    RustHtmlToken::HtmlTagVoid(_, rust_html_ident_or_puncts) => todo!("convert_rusthtmltagattributevalue_to_tokentree HtmlTagVoid"),
                    RustHtmlToken::HtmlTagStart(_, rust_html_ident_or_puncts) => todo!("convert_rusthtmltagattributevalue_to_tokentree HtmlTagStart"),
                    RustHtmlToken::HtmlTagEnd(_, rust_html_ident_or_puncts) => todo!("convert_rusthtmltagattributevalue_to_tokentree HtmlTagEnd"),
                    RustHtmlToken::HtmlTagAttributeName(_, rust_html_ident_and_punct_or_literal) => todo!("convert_rusthtmltagattributevalue_to_tokentree HtmlTagAttributeName"),
                    RustHtmlToken::HtmlTagAttributeEquals(_, punct) => todo!("convert_rusthtmltagattributevalue_to_tokentree HtmlTagAttributeEquals"),
                    RustHtmlToken::HtmlTagAttributeValue(_, literal, rust_html_ident_and_punct_or_literal, rust_html_tokens) => todo!("convert_rusthtmltagattributevalue_to_tokentree HtmlTagAttributeValue"),
                    RustHtmlToken::HtmlTagCloseVoidPunct(_, _) => todo!("convert_rusthtmltagattributevalue_to_tokentree HtmlTagCloseVoidPunct"),
                    RustHtmlToken::HtmlTagCloseSelfContainedPunct => todo!("convert_rusthtmltagattributevalue_to_tokentree HtmlTagCloseSelfContainedPunct"),
                    RustHtmlToken::HtmlTagCloseStartChildrenPunct => todo!("convert_rusthtmltagattributevalue_to_tokentree HtmlTagCloseStartChildrenPunct"),
                    RustHtmlToken::AppendToHtml(rust_html_tokens) => todo!("convert_rusthtmltagattributevalue_to_tokentree AppendToHtml"),
                    RustHtmlToken::Literal(literal, s) => {
                        let s_out = literal.clone().map(|x| x.to_string());
                        let s_out = s_out.or(s.clone());
                        let s = s_out.unwrap();
                        // let s = format!("{}", &s);
                        // let s = value_literal.to_token_stream();
                        Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#s); }))))
                    },
                    RustHtmlToken::Identifier(ident) => {
                        Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#ident); }))))
                    },
                    RustHtmlToken::ReservedChar(_, punct) => todo!("convert_rusthtmltagattributevalue_to_tokentree ReservedChar"),
                    RustHtmlToken::ReservedIndent(_, ident) => todo!("convert_rusthtmltagattributevalue_to_tokentree ReservedIndent"),
                    RustHtmlToken::Group(delimiter, ipeekable_rust_html_token, group) => todo!("convert_rusthtmltagattributevalue_to_tokentree Group"),
                    RustHtmlToken::GroupParsed(delimiter, rust_html_tokens) => todo!("convert_rusthtmltagattributevalue_to_tokentree GroupParsed"),
                    RustHtmlToken::GroupOpen(delimiter, span) => todo!("convert_rusthtmltagattributevalue_to_tokentree GroupOpen"),
                    RustHtmlToken::GroupClose(delimiter, span) => todo!("convert_rusthtmltagattributevalue_to_tokentree GroupClose"),
                }
            } else {
                let as_stream = Rc::new(VecPeekableRustHtmlToken::new(value_tokens.clone()));
                let converted = self.convert_rusthtmltokens_to_plain_rust(as_stream, ct)?;
                Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#converted); }))))
            }
        } else if let Some(token) = value_tokens_special {
            let converted = self.convert_ident_and_punct_or_literal_to_tokenstream(token, ct)?;
            Ok(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#converted); }))))
        } else {
            todo!("convert_rusthtmltagattributevalue_to_tokentree failed")
        }
    }
    
    fn convert_appendhtmlstring_to_tokentree(&self, _html_string: String, _it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        todo!("convert_appendhtmlstring_to_tokentree")
    }
    
    fn format_tag_open(&self, tag_string: &String, ct: Rc<dyn ICancellationToken>) -> String {
        todo!("format_tag_open")
    }
    
    fn format_tag_close(&self, tag_string: &String, ct: Rc<dyn ICancellationToken>) -> String {
        todo!("format_tag_close")
    }
    
    fn format_tag_name_as_string(&self, tag_tokens: &Vec<RustHtmlIdentOrPunct>, ct: Rc<dyn ICancellationToken>) -> String {
        todo!("format_tag_name_as_string")
    }
}