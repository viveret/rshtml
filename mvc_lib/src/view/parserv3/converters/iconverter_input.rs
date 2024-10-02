use std::rc::Rc;

use proc_macro2::{Group, TokenTree};

use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::ipeekable_tokentree::IPeekableTokenTree;
use crate::view::parserv3::core::peekable::stream_peekable_tokentree::StreamPeekableTokenTree;
use crate::view::parserv3::core::peekable::vec_peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

pub trait IConverterInput {
    fn convert(&self, input: Rc<dyn IPeekableTokenTree>) -> Rc<dyn IPeekableRustHtmlToken>;
    fn convert_group(&self, group: Group) -> RustHtmlToken;
}

pub struct ConverterInput {
}

impl ConverterInput {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl IConverterInput for ConverterInput {
    fn convert(&self, input: Rc<dyn IPeekableTokenTree>) -> Rc<dyn IPeekableRustHtmlToken> {
        let mut output = vec![];
        loop {
            let token = input.peek();
            if token.is_none() {
                break;
            }
            match token.expect("peeked token") {
                TokenTree::Group(group) => {
                    let token = self.convert_group(group);
                    output.push(token);
                    input.next();
                },
                TokenTree::Ident(ident) => {
                    output.push(RustHtmlToken::Identifier(ident));
                    input.next();
                },
                TokenTree::Punct(punct) => {
                    output.push(RustHtmlToken::ReservedChar(punct.as_char(), punct));
                    input.next();
                },
                TokenTree::Literal(literal) => {
                    output.push(RustHtmlToken::Literal(Some(literal.clone()), Some(literal.to_string())));
                    input.next();
                },
            }
        }
        Rc::new(VecPeekableRustHtmlToken::new(output))
    }
    
    fn convert_group(&self, group: Group) -> RustHtmlToken {
        let group_stream_in = Rc::new(StreamPeekableTokenTree::new(group.stream()));
        let group_stream_out = self.convert(group_stream_in);
        RustHtmlToken::Group(group.delimiter(), group_stream_out, None)
    }
}