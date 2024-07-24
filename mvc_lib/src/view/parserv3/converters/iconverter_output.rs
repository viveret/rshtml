use std::rc::Rc;

use proc_macro2::Delimiter;
use proc_macro2::Group;
use proc_macro2::Literal;
use proc_macro2::Punct;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::ipeekable_tokentree::IPeekableTokenTree;
use crate::view::parserv3::core::peekable::vec_peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::vec_peekable_tokentree::VecPeekableTokenTree;
use crate::view::rusthtml::rusthtml_token::RustHtmlIdentAndPunctOrLiteral;
use crate::view::rusthtml::rusthtml_token::RustHtmlIdentOrPunct;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

pub trait IConverterOutput {
    fn convert(&self, input: Rc<dyn IPeekableRustHtmlToken>) -> Result<Rc<dyn IPeekableTokenTree>, RustHtmlError>;


    fn parse_rusthtmltokens_to_plain_rust(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn convert_rusthtmltoken_to_tokentree(self: &Self, token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltokens_to_plain_rust(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmlgroupparsed_to_tokentree(self: &Self, delimiter: &Delimiter, inner_tokens: &Vec<RustHtmlToken>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmlappendhtml_to_tokentree(self: &Self, inner_as_string: Option<&String>, inner_as_literal: Option<&Literal>, inner_as_ident: Option<&Vec<RustHtmlIdentOrPunct>>, inner: Option<&Vec<RustHtmlToken>>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltextnode_to_tokentree(self: &Self, first_text: &String, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltagstart_to_tokentree(self: &Self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltagvoid_to_tokentree(self: &Self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltagend_to_tokentree(self: &Self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltagclosestartchildren_to_tokentree(self: &Self, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltagclosesselfcontained_to_tokentree(self: &Self, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltagclosevoid_to_tokentree(self: &Self, c: Option<(char, Punct)>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltagattributeequals_to_tokentree(self: &Self, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_ident_and_punct_or_literal_to_tokenstream(self: &Self, tag: &RustHtmlIdentAndPunctOrLiteral) -> Result<TokenStream, RustHtmlError>;
    fn convert_rusthtmltagattributename_to_tokentree(self: &Self, tag: &String, tag_tokens: &Option<RustHtmlIdentAndPunctOrLiteral>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_rusthtmltagattributevalue_to_tokentree(self: &Self, value_string: Option<&String>, value_literal: Option<&Literal>, value_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, value_rust: Option<&Vec<RustHtmlToken>>, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError>;
    fn convert_appendhtmlstring_to_tokentree(self: &Self, html_string: String, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<(), RustHtmlError>;

    fn format_tag_open(self: &Self, tag_string: &String) -> String;
    fn format_tag_close(self: &Self, tag_string: &String) -> String;
    fn format_tag_name_as_string(self: &Self, tag_tokens: &Vec<RustHtmlIdentOrPunct>) -> String;
}

pub struct ConverterOutput {
}

impl ConverterOutput {
    pub fn new() -> Self {
        Self {
        }
    }
    
    fn convert_token(&self, token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError> {
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
            RustHtmlToken::Group(_delimiter, _stream, group) => Ok(TokenTree::Group(group.clone().expect("group is None"))),
            RustHtmlToken::GroupParsed(delimiter, inner_tokens) => 
                self.convert_rusthtmlgroupparsed_to_tokentree(delimiter, inner_tokens, it),
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
            RustHtmlToken::HtmlTextNode(text) => 
                self.convert_rusthtmltextnode_to_tokentree(text, it),
            RustHtmlToken::AppendToHtml(inner) =>
                self.convert_rusthtmlappendhtml_to_tokentree(None, None, None, Some(inner)),
            _ => { Err(RustHtmlError::from_string(format!("Could not handle token {:?}", token))) }
        }
    }

    // convert a RustHtml group to Rust tokens.
    // delimiter: the delimiter of the group.
    // inner_tokens: the inner tokens of the group.
    // output: the destination for the Rust tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_rusthtmlgroupparsed_to_tokentree(self: &Self, delimiter: &Delimiter, inner_tokens: &Vec<RustHtmlToken>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError> {
        let inner_it = Rc::new(VecPeekableRustHtmlToken::new(inner_tokens.clone()));
        let group = self.convert_rusthtmltokens_to_plain_rust(inner_it)?;
        if let TokenTree::Group(group) = group {
            let group_inner = group.stream();
            Ok(TokenTree::Group(Group::new(delimiter.clone(), group_inner)))
        } else {
            Err(RustHtmlError::from_string(format!("Expected group, got {:?}", group)))
        }
    }
}

impl IConverterOutput for ConverterOutput {
    fn convert(&self, input: Rc<dyn IPeekableRustHtmlToken>) -> Result<Rc<dyn IPeekableTokenTree>, RustHtmlError> {
        let mut output: Vec<TokenTree> = vec![];
        loop {
            let token = input.peek();
            if token.is_none() {
                break;
            }
            let token = token.unwrap();
            input.next();
            // todo: check for empty group
            output.push(self.convert_token(&token, input.clone())?);
        }
        Ok(Rc::new(VecPeekableTokenTree::new(output)))
    }
    
    fn parse_rusthtmltokens_to_plain_rust(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let it = Rc::new(VecPeekableRustHtmlToken::new(rusthtml_tokens.clone()));
        let result = self.convert(it)?;
        Ok(result.to_vec())
    }
    
    fn convert_rusthtmltoken_to_tokentree(self: &Self, token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError> {
        todo!()
    }
    
    fn convert_rusthtmltokens_to_plain_rust(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError> {
        todo!()
    }
    
    fn convert_rusthtmlgroupparsed_to_tokentree(self: &Self, delimiter: &Delimiter, inner_tokens: &Vec<RustHtmlToken>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError> {
        todo!()
    }
    
    fn convert_rusthtmlappendhtml_to_tokentree(self: &Self, inner_as_string: Option<&String>, inner_as_literal: Option<&Literal>, inner_as_ident: Option<&Vec<RustHtmlIdentOrPunct>>, inner: Option<&Vec<RustHtmlToken>>) -> Result<TokenTree, RustHtmlError> {
        todo!()
    }
    
    fn convert_rusthtmltextnode_to_tokentree(self: &Self, first_text: &String, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError> {
        todo!()
    }
    
    fn convert_rusthtmltagstart_to_tokentree(self: &Self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError> {
        todo!()
    }
    
    fn convert_rusthtmltagvoid_to_tokentree(self: &Self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError> {
        todo!()
    }
    
    fn convert_rusthtmltagend_to_tokentree(self: &Self, tag: &String, tag_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError> {
        todo!()
    }
    
    fn convert_rusthtmltagclosestartchildren_to_tokentree(self: &Self, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError> {
        todo!()
    }
    
    fn convert_rusthtmltagclosesselfcontained_to_tokentree(self: &Self, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError> {
        todo!()
    }
    
    fn convert_rusthtmltagclosevoid_to_tokentree(self: &Self, c: Option<(char, Punct)>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError> {
        todo!()
    }
    
    fn convert_rusthtmltagattributeequals_to_tokentree(self: &Self, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError> {
        todo!()
    }
    
    fn convert_ident_and_punct_or_literal_to_tokenstream(self: &Self, _tag: &RustHtmlIdentAndPunctOrLiteral) -> Result<TokenStream, RustHtmlError> {
        todo!()
    }
    
    fn convert_rusthtmltagattributename_to_tokentree(self: &Self, _tag: &String, _tag_tokens: &Option<RustHtmlIdentAndPunctOrLiteral>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError> {
        todo!()
    }
    
    fn convert_rusthtmltagattributevalue_to_tokentree(self: &Self, _value_string: Option<&String>, value_literal: Option<&Literal>, _value_tokens: Option<&Vec<RustHtmlIdentOrPunct>>, _value_rust: Option<&Vec<RustHtmlToken>>, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<TokenTree, RustHtmlError> {
        todo!()
    }
    
    fn convert_appendhtmlstring_to_tokentree(self: &Self, _html_string: String, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<(), RustHtmlError> {
        todo!()
    }
    
    fn format_tag_open(self: &Self, tag_string: &String) -> String {
        todo!()
    }
    
    fn format_tag_close(self: &Self, tag_string: &String) -> String {
        todo!()
    }
    
    fn format_tag_name_as_string(self: &Self, tag_tokens: &Vec<RustHtmlIdentOrPunct>) -> String {
        todo!()
    }
}