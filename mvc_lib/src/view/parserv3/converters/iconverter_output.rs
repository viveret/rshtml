use std::rc::Rc;

use proc_macro2::Delimiter;
use proc_macro2::Group;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::parser_parts::peekable_tokentree::VecPeekableTokenTree;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;

pub trait IConverterOutput {
    fn convert(&self, input: Rc<dyn IPeekableRustHtmlToken>) -> Result<Rc<dyn IPeekableTokenTree>, RustHtmlError>;
}

pub struct ConverterOutput {
}

impl ConverterOutput {
    pub fn new() -> Self {
        Self {
        }
    }
    
    fn convert_token(&self, token: &RustHtmlToken) -> Result<TokenTree, RustHtmlError> {
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
                self.convert_rusthtmlgroupparsed_to_tokentree(delimiter, inner_tokens, it)?,
            RustHtmlToken::HtmlTagStart(tag, tag_tokens) =>
                self.convert_rusthtmltagstart_to_tokentree(tag, tag_tokens.as_ref(), it)?,
            RustHtmlToken::HtmlTagVoid(tag, tag_tokens) =>
                self.convert_rusthtmltagvoid_to_tokentree(tag, tag_tokens.as_ref(), it)?,
            RustHtmlToken::HtmlTagEnd(tag, tag_tokens) =>
                self.convert_rusthtmltagend_to_tokentree(tag, tag_tokens.as_ref(), it)?,
            RustHtmlToken::HtmlTagCloseStartChildrenPunct =>
                self.convert_rusthtmltagclosestartchildren_to_tokentree(output, it)?,
            RustHtmlToken::HtmlTagCloseSelfContainedPunct =>
                self.convert_rusthtmltagclosesselfcontained_to_tokentree(output, it)?,
            RustHtmlToken::HtmlTagCloseVoidPunct(c) =>
                self.convert_rusthtmltagclosevoid_to_tokentree(c.clone(), output, it)?,
            RustHtmlToken::HtmlTagAttributeEquals(_c, _punct) =>
                self.convert_rusthtmltagattributeequals_to_tokentree(output, it)?,
            RustHtmlToken::HtmlTagAttributeName(tag, tag_tokens) =>
                self.convert_rusthtmltagattributename_to_tokentree(tag, tag_tokens, output, it)?,
            RustHtmlToken::HtmlTagAttributeValue(value_string, value_literal, value_tokens, value_rust_tokens) =>
                self.convert_rusthtmltagattributevalue_to_tokentree(value_string.as_ref(), value_literal.as_ref(), value_tokens.as_ref(), value_rust_tokens.as_ref(), output, it)?,
            RustHtmlToken::HtmlTextNode(text) => 
                self.convert_rusthtmltextnode_to_tokentree(text, output, it)?,
            RustHtmlToken::AppendToHtml(inner) =>
                self.convert_rusthtmlappendhtml_to_tokentree(None, None, None, Some(inner), output)?,
            _ => { Err(RustHtmlError::from_string(format!("Could not handle token {:?}", token))) }
        }
    }

    // convert a RustHtml group to Rust tokens.
    // delimiter: the delimiter of the group.
    // inner_tokens: the inner tokens of the group.
    // output: the destination for the Rust tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn convert_rusthtmlgroupparsed_to_tokentree(self: &Self, delimiter: &Delimiter, inner_tokens: &Vec<RustHtmlToken>, output: &mut Vec<TokenTree>, _it: Rc<dyn IPeekableRustHtmlToken>) -> Result<(), RustHtmlError> {
        let mut group = vec![];
        let inner_it = Rc::new(VecPeekableRustHtmlToken::new(inner_tokens.clone()));
        self.convert_rusthtmltokens_to_plain_rust(&mut group, inner_it)?;
        output.push(TokenTree::Group(Group::new(delimiter.clone(), TokenStream::from_iter(group.iter().cloned()))));
        Ok(())
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
            output.extend(self.convert_token(token)?);
            input.next();
        }
        Ok(Rc::new(VecPeekableTokenTree::new(output)))
    }
}