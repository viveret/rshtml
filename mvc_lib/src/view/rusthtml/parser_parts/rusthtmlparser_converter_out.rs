use std::cell::RefCell;
use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use core_lib::sys::call_tracker::CallstackTrackerScope;
use core_macro_lib::nameof_member_fn;
use proc_macro2::{TokenTree, Punct, Delimiter, Group, Ident, TokenStream, Literal, Span};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use crate::view::rusthtml::rusthtml_token::{RustHtmlToken, RustHtmlIdentAndPunctOrLiteral, RustHtmlIdentOrPunct};

use super::peekable_rusthtmltoken::{IPeekableRustHtmlToken, VecPeekableRustHtmlToken};
use super::rusthtmlparser_all::{IRustHtmlParserAssignSharedParts, IRustHtmlParserAll};


pub trait IRustHtmlParserConverterOut: IRustHtmlParserAssignSharedParts {
    fn convert_token(self: &Self, token: RustHtmlToken) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_peek_stream(self: &Self, tokens: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_vec(self: &Self, tokens: Vec<RustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<TokenTree>, RustHtmlError>;

    fn convert_rusthtmltoken_to_tokentree(self: &Self, token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_rusthtmltokens_to_plain_rust(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_rusthtmlgroupparsed_to_tokentree(self: &Self, delimiter: &Delimiter, inner_tokens: Vec<RustHtmlToken>, _it: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_rusthtmlappendhtml_to_tokentree(self: &Self, inner_as_string: Option<&String>, inner_as_string: Option<&Literal>, inner_as_ident: Option<&Vec<RustHtmlIdentOrPunct>>, inner: Option<Vec<RustHtmlToken>>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_rusthtmltextnode_to_tokentree(self: &Self, first_text: &String, _first_span: &Span, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_rusthtmltagstart_to_tokentree(self: &Self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_rusthtmltagvoid_to_tokentree(self: &Self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_rusthtmltagend_to_tokentree(self: &Self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_rusthtmltagclosestartchildren_to_tokentree(self: &Self, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_rusthtmltagclosesselfcontained_to_tokentree(self: &Self, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_rusthtmltagclosevoid_to_tokentree(self: &Self, c: Option<(char, Punct)>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_rusthtmltagattributeequals_to_tokentree(self: &Self, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_ident_and_punct_or_literal_to_tokenstream(self: &Self, tag: &RustHtmlIdentAndPunctOrLiteral) -> Result<TokenStream, RustHtmlError>;
    fn convert_rusthtmltagattributename_to_tokentree(self: &Self, tag: &String, tag_tokens: &Option<RustHtmlIdentAndPunctOrLiteral>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_rusthtmltagattributevalue_to_tokentree(self: &Self, value_string: Option<&String>, value_literal: Option<&Literal>, value_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, value_rust: Option<&Vec<RustHtmlToken>>, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_appendhtmlstring_to_tokentree(self: &Self, html_string: String, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError>;

    fn convert_rusthtmlappendhtml_vec_rshtml_to_tokentree(self: &Self, inner: Vec<RustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_rusthtmlappendhtml_vec_ident_or_punct_to_tokentree(self: &Self, inner_as_ident: &Vec<RustHtmlIdentOrPunct>) -> Result<Vec<TokenTree>, RustHtmlError>;


    fn format_tag_open(self: &Self, tag_string: &String) -> String;
    fn format_tag_close(self: &Self, tag_string: &String) -> String;
    fn format_tag_name_as_string(self: &Self, tag_tokens: &Vec<RustHtmlIdentOrPunct>) -> String;

}


pub struct RustHtmlParserConverterOut {
    parser: RefCell<Option<Rc<dyn IRustHtmlParserAll>>>,
}

impl RustHtmlParserConverterOut {
    pub fn new() -> Self {
        Self {
            parser: RefCell::new(None),
        }
    }

    pub fn write_html_stream(self: &Self, ident_tokenstream: TokenStream) -> TokenStream {
        TokenStream::from(quote::quote! { html_output.write_html(HtmlString::from(#ident_tokenstream)); })
    }
}

impl IRustHtmlParserAssignSharedParts for RustHtmlParserConverterOut {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        *self.parser.borrow_mut() = Some(parser);
    }
}

impl IRustHtmlParserConverterOut for RustHtmlParserConverterOut {
    fn convert_token(self: &Self, token: RustHtmlToken) -> Result<Vec<TokenTree>, RustHtmlError> {
        todo!("convert_token");
    }

    fn convert_vec(self: &Self, tokens: Vec<RustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let _scope = CallstackTrackerScope::enter(context.get_call_stack(), nameof::name_of_type!(RustHtmlParserConverterOut), nameof_member_fn!(RustHtmlParserConverterOut::convert_vec));
        let it = Rc::new(VecPeekableRustHtmlToken::new(tokens));
        self.convert_peek_stream(it, context.clone(), ct)
    }

    fn convert_peek_stream(self: &Self, tokens: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        // let mut output = vec![];
        // loop {
        //     match tokens.peek() {
        //         Some(token) => {
        //             let token = token.clone();
        //             let converted = self.convert_token(token)?;
        //             output.extend(converted);
        //         },
        //         None => {
        //             break;
        //         }
        //     }
        // }
        // Ok(output)
        let _scope = CallstackTrackerScope::enter(context.get_call_stack(), nameof::name_of_type!(RustHtmlParserConverterOut), nameof_member_fn!(RustHtmlParserConverterOut::convert_peek_stream));
        self.convert_rusthtmltokens_to_plain_rust(tokens, context.clone(), ct)
    }

    fn convert_rusthtmltoken_to_tokentree(self: &Self, token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let _scope = CallstackTrackerScope::enter(context.get_call_stack(), nameof::name_of_type!(RustHtmlParserConverterOut), nameof_member_fn!(RustHtmlParserConverterOut::convert_rusthtmltoken_to_tokentree));
        match token {
            RustHtmlToken::GroupParsed(delimiter, inner_tokens) => {
                self.convert_rusthtmlgroupparsed_to_tokentree(delimiter, inner_tokens.clone(), it, context.clone(), ct)
            },
            RustHtmlToken::Identifier(ident) => Ok(vec![TokenTree::Ident(ident.clone())]),
            RustHtmlToken::Literal(literal, string) => {
                if let Some(literal) = literal {
                    Ok(vec![TokenTree::Literal(literal.clone())])
                } else if let Some(string) = string {
                    Ok(vec![TokenTree::Literal(Literal::string(string))])
                } else {
                    Err(RustHtmlError::from_string(format!("Token is neither literal or string: {:?}", token)))
                }
            },
            RustHtmlToken::ReservedChar(_, punct) => Ok(vec![TokenTree::Punct(punct.clone())]),
            RustHtmlToken::Group(_delimiter, group) => Ok(vec![TokenTree::Group(group.clone().unwrap().clone())]),
            RustHtmlToken::GroupParsed(delimiter, inner_tokens) => 
                self.convert_rusthtmlgroupparsed_to_tokentree(delimiter, inner_tokens.clone(), it, context.clone(), ct),
            RustHtmlToken::HtmlTagStart(tag, tag_tokens) =>
                self.convert_rusthtmltagstart_to_tokentree(tag, tag_tokens.as_ref(), it),
            RustHtmlToken::HtmlTagVoid(tag, tag_tokens) =>
                self.convert_rusthtmltagvoid_to_tokentree(tag, tag_tokens.as_ref(), it),
            RustHtmlToken::HtmlTagEnd(tag, tag_tokens) =>
                self.convert_rusthtmltagend_to_tokentree(tag, tag_tokens.as_ref(), it),
            RustHtmlToken::HtmlTagCloseStartChildrenPunct =>
                self.convert_rusthtmltagclosestartchildren_to_tokentree(it),
            RustHtmlToken::HtmlTagCloseSelfContainedPunct =>
                self.convert_rusthtmltagclosesselfcontained_to_tokentree(it),
            RustHtmlToken::HtmlTagCloseVoidPunct(c) =>
                self.convert_rusthtmltagclosevoid_to_tokentree(c.clone(), it),
            RustHtmlToken::HtmlTagAttributeEquals(_c, _punct) =>
                self.convert_rusthtmltagattributeequals_to_tokentree(it),
            RustHtmlToken::HtmlTagAttributeName(tag, tag_tokens) =>
                self.convert_rusthtmltagattributename_to_tokentree(tag, tag_tokens, it),
            RustHtmlToken::HtmlTagAttributeValue(value_string, value_literal, value_tokens, value_rust_tokens) =>
                self.convert_rusthtmltagattributevalue_to_tokentree(value_string.as_ref(), value_literal.as_ref(), value_tokens.as_ref(), value_rust_tokens.as_ref(), it),
            RustHtmlToken::HtmlTextNode(text, span) => 
                self.convert_rusthtmltextnode_to_tokentree(text, span, it),
            RustHtmlToken::AppendToHtml(inner) =>
                self.convert_rusthtmlappendhtml_to_tokentree(None, None, None, Some(inner.clone())),
            _ => { return Err(RustHtmlError::from_string(format!("Could not handle token {:?}", token))); }
        }
    }

    fn convert_rusthtmltokens_to_plain_rust(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let mut output = vec![];
        loop
        {
            if ct.is_cancelled() {
                // probably an infinite loop
                return Err(RustHtmlError::from_string(format!("Cancellation token was cancelled")));
            }

            let token_option = it.next();
            if let Some(token) = token_option {
                let tokens = self.convert_rusthtmltoken_to_tokentree(&token, it.clone(), context.clone(), ct.clone())?;
                output.extend(tokens);
            } else {
                break;
            }
        }
        Ok(output)
    }

    fn convert_rusthtmlgroupparsed_to_tokentree(self: &Self, delimiter: &Delimiter, inner_tokens: Vec<RustHtmlToken>, _it: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let inner_it = Rc::new(VecPeekableRustHtmlToken::new(inner_tokens));
        let group = self.convert_rusthtmltokens_to_plain_rust(inner_it, context, ct)?;
        Ok(vec![TokenTree::Group(Group::new(delimiter.clone(), TokenStream::from_iter(group.iter().cloned())))])
    }

    fn convert_rusthtmlappendhtml_to_tokentree(self: &Self, inner_as_string: Option<&String>, inner_as_literal: Option<&Literal>, inner_as_ident: Option<&Vec<RustHtmlIdentOrPunct>>, inner: Option<Vec<RustHtmlToken>>) -> Result<Vec<TokenTree>, RustHtmlError> {
        if let Some(inner) = inner {
            self.convert_rusthtmlappendhtml_vec_rshtml_to_tokentree(inner)
        } else if let Some(inner_as_ident) = inner_as_ident {
            self.convert_rusthtmlappendhtml_vec_ident_or_punct_to_tokentree(inner_as_ident)
        } else if let Some(inner_as_literal) = inner_as_literal {
            Ok(vec![TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#inner_as_literal); })))])
        } else if let Some(inner_as_string) = inner_as_string {
            Ok(vec![TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#inner_as_string); })))])
        } else {
            Err(RustHtmlError::from_str("Both inner_as_string and inner are None"))
        }
    }

    fn convert_rusthtmltextnode_to_tokentree(self: &Self, first_text: &String, _first_span: &Span, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
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
        Ok(vec![TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#text); })))])
    }

    fn convert_rusthtmltagstart_to_tokentree(self: &Self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let tag_as_html = self.format_tag_open(tag); // true, tag, false, tag_tokens
        Ok(vec![TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#tag_as_html); })))])
    }

    fn convert_rusthtmltagvoid_to_tokentree(self: &Self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let tag_as_html = self.format_tag_open(tag); // true, tag, true, tag_tokens
        Ok(vec![TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#tag_as_html); })))])
    }

    fn convert_rusthtmltagend_to_tokentree(self: &Self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let tag_as_html = self.format_tag_close(tag); // false, tag, false, tag_tokens
        Ok(vec![TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#tag_as_html); })))])
    }

    fn convert_rusthtmltagclosestartchildren_to_tokentree(self: &Self, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        Ok(vec![TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(">"); })))])
    }

    fn convert_rusthtmltagclosesselfcontained_to_tokentree(self: &Self, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        Ok(vec![TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str("/>"); })))])
    }

    fn convert_rusthtmltagclosevoid_to_tokentree(self: &Self, c: Option<(char, Punct)>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let s = if c.is_some() { "/>" } else { ">" };
        Ok(vec![TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#s); })))])
    }

    fn convert_rusthtmltagattributeequals_to_tokentree(self: &Self, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        Ok(vec![TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str("="); })))])
    }

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

    fn convert_rusthtmltagattributename_to_tokentree(self: &Self, tag: &String, tag_tokens: &Option<RustHtmlIdentAndPunctOrLiteral>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let mut output = vec![];
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(" "); }))));
        
        let tag_as_html = if let Some(tag_tokens) = tag_tokens {
            match tag_tokens {
                RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(ident_and_punct) => self.format_tag_name_as_string(ident_and_punct),
                RustHtmlIdentAndPunctOrLiteral::Literal(literal) => literal.to_string(),
            }
        } else {
            tag.to_string()
        };
        output.push(TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#tag_as_html); }))));
        Ok(output)
    }

    fn convert_rusthtmltagattributevalue_to_tokentree(self: &Self, value_string: Option<&String>, value_literal: Option<&Literal>, value_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, value_rust: Option<&Vec<RustHtmlToken>>, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let mut output = vec![];
        let quote_before_tokens = self.convert_appendhtmlstring_to_tokentree("\"".to_string(), it.clone())?;
        output.extend(quote_before_tokens);

        let inner_tokens = self.convert_rusthtmlappendhtml_to_tokentree(value_string, value_literal, value_tokens, value_rust.cloned())?;
        output.extend(inner_tokens);

        let tokens_after = self.convert_appendhtmlstring_to_tokentree("\"".to_string(), it.clone())?;
        output.extend(tokens_after);

        Ok(output)
    }

    fn convert_appendhtmlstring_to_tokentree(self: &Self, html_string: String, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        Ok(vec![TokenTree::Group(Group::new(Delimiter::None, TokenStream::from(quote::quote! { html_output.write_html_str(#html_string); })))])
    }

    fn convert_rusthtmlappendhtml_vec_rshtml_to_tokentree(self: &Self, inner: Vec<RustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        todo!("convert_rusthtmlappendhtml_vec_rshtml_to_tokentree")
    }

    fn convert_rusthtmlappendhtml_vec_ident_or_punct_to_tokentree(self: &Self, inner_as_ident: &Vec<RustHtmlIdentOrPunct>) -> Result<Vec<TokenTree>, RustHtmlError> {
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
        Ok(vec![TokenTree::Group(Group::new(Delimiter::None, self.write_html_stream(ident_tokenstream)))])
    }

    fn format_tag_open(self: &Self, tag_string: &String) -> String {
        format!("<{}", tag_string)
    }

    fn format_tag_close(self: &Self, tag_string: &String) -> String {
        format!("</{}>", tag_string)
    }

    fn format_tag_name_as_string(self: &Self, tag_tokens: &Vec<RustHtmlIdentOrPunct>) -> String {
        tag_tokens.iter()
            .map(|x| match x {
                RustHtmlIdentOrPunct::Ident(ident) => ident.to_string(),
                RustHtmlIdentOrPunct::Punct(punct) => punct.to_string(),
            })
            .collect::<Vec<String>>()
            .join("")
    }

    // fn convert_out(self: &Self, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<TokenStream, RustHtmlError> {
    //     let output = context.pop_output_buffer().unwrap();
    //     let output_it = Rc::new(VecPeekableRustHtmlToken::new(output.borrow().clone()));
    //     match self.convert_peek_stream(output_it, context, ct) {
    //         Ok(output) => Ok(TokenStream::from_iter(output.into_iter())),
    //         Err(RustHtmlError(err)) => Err(RustHtmlError::from_string(err.into_owned()))
    //     }
    // }
}