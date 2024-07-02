use std::rc::Rc;

use proc_macro2::TokenTree;

use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::parser_parts::peekable_tokentree::StreamPeekableTokenTree;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;

pub trait IConverterInput {
    fn convert(&self, input: Rc<dyn IPeekableTokenTree>) -> Rc<dyn IPeekableRustHtmlToken>;
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
                    let group_stream_in = Rc::new(StreamPeekableTokenTree::new(group.stream()));
                    let group_stream_out = self.convert(group_stream_in);
                    output.push(RustHtmlToken::Group(group.delimiter(), group_stream_out, Some(group.clone())));
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
}