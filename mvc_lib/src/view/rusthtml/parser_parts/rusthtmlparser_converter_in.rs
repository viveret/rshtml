use std::cell::RefCell;
use std::rc::Rc;

use core_lib::{asyncly::icancellation_token::ICancellationToken, sys::call_tracker::CallstackTrackerScope};
use core_macro_lib::nameof_member_fn;
use proc_macro2::{TokenTree, Punct, Delimiter, Group, Ident, TokenStream, Literal};

use crate::view::rusthtml::{irusthtml_parser_context::IRustHtmlParserContext, rusthtml_error::RustHtmlError};
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;

use super::peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use super::rusthtmlparser_all::{IRustHtmlParserAssignSharedParts, IRustHtmlParserAll};


pub trait IRustHtmlParserConverterIn: IRustHtmlParserAssignSharedParts {
    fn convert(self: &Self, token: TokenTree, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlToken, RustHtmlError>;
    fn convert_rust(self: &Self, tokens: Rc<dyn IPeekableTokenTree>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn convert_stream(self: &Self, tokens: TokenStream, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn convert_vec(self: &Self, tokens: Vec<TokenTree>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn convert_group(self: &Self, group: &Group, expect_return_html: bool, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlToken, RustHtmlError>;
    fn convert_literal(self: &Self, literal: &Literal, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlToken, RustHtmlError>;
    fn convert_punct(self: &Self, punct: &Punct) -> Result<RustHtmlToken, RustHtmlError>;
    fn convert_ident(self: &Self, ident: &Ident) -> Result<RustHtmlToken, RustHtmlError>;
}


pub struct RustHtmlParserConverterIn {
    parser: RefCell<Option<Rc<dyn IRustHtmlParserAll>>>,
}

impl RustHtmlParserConverterIn {
    pub fn new() -> Self {
        Self {
            parser: RefCell::new(None),
        }
    }
}

impl IRustHtmlParserAssignSharedParts for RustHtmlParserConverterIn {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        *self.parser.borrow_mut() = Some(parser);
    }
}

impl IRustHtmlParserConverterIn for RustHtmlParserConverterIn {
    fn convert(self: &Self, token: TokenTree, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlToken, RustHtmlError> {
        let _scope = CallstackTrackerScope::enter(ctx.get_call_stack(), nameof::name_of_type!(RustHtmlParserConverterIn), nameof_member_fn!(RustHtmlParserConverterIn::convert));
        match token {
            TokenTree::Group(group) => {
                self.convert_group(&group, false, ctx.clone(), ct)
            },
            TokenTree::Ident(ident) => {
                self.convert_ident(&ident)
            },
            TokenTree::Punct(punct) => {
                self.convert_punct(&punct)
            },
            TokenTree::Literal(literal) => {
                self.convert_literal(&literal, ct)
            }
        }
    }

    fn convert_vec(self: &Self, tokens: Vec<TokenTree>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let _scope = CallstackTrackerScope::enter(ctx.get_call_stack(), nameof::name_of_type!(RustHtmlParserConverterIn), nameof_member_fn!(RustHtmlParserConverterIn::convert_vec));
        let mut output = vec![];
        for token in tokens {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_str("convert_vec cancelled"));
            }
            output.push(self.convert(token, ctx.clone(), ct.clone())?);
        }
        Ok(output)
    }

    fn convert_group(self: &Self, group: &Group, expect_return_html: bool, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlToken, RustHtmlError> {
        let _scope = CallstackTrackerScope::enter(ctx.get_call_stack(), nameof::name_of_type!(RustHtmlParserConverterIn), nameof_member_fn!(RustHtmlParserConverterIn::convert_group));
        if ct.is_cancelled() {
            return Err(RustHtmlError::from_str("convert_group cancelled"));
        }

        let delimiter = group.delimiter();

        if expect_return_html && delimiter == Delimiter::Bracket {
            return Err(RustHtmlError::from_str("convert_group: expect_return_html is true and delimiter is Delimiter::Bracket"));
        }
        
        // if delimiter == Delimiter::Brace {
            let mut inner_tokens = vec![];
            
            // prefix and postfix with html_output decorators
            if expect_return_html {
                let tokens = self.convert_stream(quote::quote! { let html_output = HtmlBuffer::new(); }, ctx.clone(), ct.clone())?;
                inner_tokens.extend_from_slice(&tokens);
            }
            
            let inner2 = self.convert_stream(group.stream(), ctx.clone(), ct.clone())?;
            inner_tokens.extend_from_slice(&inner2);
            
            if expect_return_html {
                let tokens = self.convert_stream(quote::quote! { html_output.collect_html() }, ctx.clone(), ct)?;
                inner_tokens.extend_from_slice(&tokens);
            }

            Ok(RustHtmlToken::Group(delimiter, Rc::new(VecPeekableRustHtmlToken::new(inner_tokens)), Some(group.clone())))
        // } else {
        //     Ok(RustHtmlToken::Group(delimiter, group.clone()))
        // }
    }

    fn convert_stream(self: &Self, tokens: TokenStream, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let _scope = CallstackTrackerScope::enter(ctx.get_call_stack(), nameof::name_of_type!(RustHtmlParserConverterIn), nameof_member_fn!(RustHtmlParserConverterIn::convert_stream));
        let mut output = vec![];
        for token in tokens {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_str("convert_stream cancelled"));
            }
            output.push(self.convert(token, ctx.clone(), ct.clone())?);
        }
        Ok(output)
    }

    fn convert_literal(self: &Self, literal: &Literal, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlToken, RustHtmlError> {
        Ok(RustHtmlToken::Literal(Some(literal.clone()), None))
    }

    fn convert_punct(self: &Self, punct: &Punct) -> Result<RustHtmlToken, RustHtmlError> {
        Ok(RustHtmlToken::ReservedChar(punct.as_char(), punct.clone()))
    }

    fn convert_ident(self: &Self, ident: &Ident) -> Result<RustHtmlToken, RustHtmlError> {
        Ok(RustHtmlToken::Identifier(ident.clone()))
    }

    fn convert_rust(self: &Self, tokens: Rc<dyn IPeekableTokenTree>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let _scope = CallstackTrackerScope::enter(ctx.get_call_stack(), nameof::name_of_type!(RustHtmlParserConverterIn), nameof_member_fn!(RustHtmlParserConverterIn::convert_rust));
        let mut output = vec![];
        loop {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_str("convert_rust cancelled"));
            }

            let next_token = tokens.next();
            match next_token {
                Some(token) => {
                    output.push(self.convert(token, ctx.clone(), ct.clone())?);
                },
                None => {
                    break;
                }
            }
        }
        Ok(output)
    }
}