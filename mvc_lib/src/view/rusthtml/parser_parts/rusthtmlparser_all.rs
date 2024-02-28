use std::cell::RefCell;
use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use core_lib::sys::call_tracker::CallstackTrackerScope;
use core_macro_lib::nameof_member_fn;
use proc_macro2::TokenStream;

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;

use super::peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use super::peekable_tokentree::StreamPeekableTokenTree;
use super::rusthtmlparser_converter_in::{IRustHtmlParserConverterIn, RustHtmlParserConverterIn};
use super::rusthtmlparser_converter_out::{IRustHtmlParserConverterOut, RustHtmlParserConverterOut};
use super::rusthtmlparser_expander::{IRustHtmlParserExpander, RustHtmlParserExpander};
use super::rusthtmlparser_rusthtml::IRustHtmlParserRustOrHtml;
use super::rusthtmlparser_rust::IRustHtmlParserRust;
use super::rusthtmlparser_html::IRustHtmlParserHtml;

pub trait IRustHtmlParserAll {
    // top level functions
    fn expand_rust(self: &Self, input: TokenStream, cancellation_token: Rc<dyn ICancellationToken>) -> Result<TokenStream, RustHtmlError>;
    fn expand_rust_with_context(self: &Self, context: Rc<RustHtmlParserContext>, input: TokenStream, cancellation_token: Rc<dyn ICancellationToken>) -> Result<TokenStream, RustHtmlError>;

    // individual parts
    fn get_html_parser(&self) -> Rc<dyn IRustHtmlParserHtml>;
    fn get_rust_parser(&self) -> Rc<dyn IRustHtmlParserRust>;
    fn get_rust_or_html_parser(&self) -> Rc<dyn IRustHtmlParserRustOrHtml>;
    fn get_converter(&self) -> Rc<dyn IRustHtmlParserConverterIn>;
    fn get_converter_out(&self) -> Rc<dyn IRustHtmlParserConverterOut>;
    fn get_expander(&self) -> Rc<dyn IRustHtmlParserExpander>;
}

pub trait IRustHtmlParserAssignSharedParts {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>);
}


pub struct RustHtmlParserAll {
    html_parser: Rc<dyn IRustHtmlParserHtml>,
    rust_parser: Rc<dyn IRustHtmlParserRust>,
    rust_or_html_parser: Rc<dyn IRustHtmlParserRustOrHtml>,
    rust_converter_in: Rc<dyn IRustHtmlParserConverterIn>,
    rust_converter_out: Rc<dyn IRustHtmlParserConverterOut>,
    rust_expander: Rc<dyn IRustHtmlParserExpander>,
}

impl RustHtmlParserAll {
    pub fn new(
        html_parser: Rc<dyn IRustHtmlParserHtml>,
        rust_parser: Rc<dyn IRustHtmlParserRust>,
        rust_or_html_parser: Rc<dyn IRustHtmlParserRustOrHtml>,
        rust_converter_in: Rc<dyn IRustHtmlParserConverterIn>,
        rust_converter_out: Rc<dyn IRustHtmlParserConverterOut>,
        rust_expander: Rc<dyn IRustHtmlParserExpander>,
    ) -> Rc<RustHtmlParserAll> {
        let s = Rc::new(Self {
            html_parser,
            rust_parser,
            rust_or_html_parser,
            rust_converter_in,
            rust_converter_out,
            rust_expander,
        });

        s.html_parser.assign_shared_parser(s.clone());
        s.rust_parser.assign_shared_parser(s.clone());
        s.rust_or_html_parser.assign_shared_parser(s.clone());
        s.rust_converter_in.assign_shared_parser(s.clone());
        s.rust_converter_out.assign_shared_parser(s.clone());
        s.rust_expander.assign_shared_parser(s.clone());
        s
    }

    pub fn new_default() -> Rc<RustHtmlParserAll> {
        Self::new(
            Rc::new(super::rusthtmlparser_html::RustHtmlParserHtml::new()),
            Rc::new(super::rusthtmlparser_rust::RustHtmlParserRust::new()),
            Rc::new(super::rusthtmlparser_rusthtml::RustHtmlParserRustOrHtml::new()),
            Rc::new(RustHtmlParserConverterIn::new()),
            Rc::new(RustHtmlParserConverterOut::new()),
            Rc::new(RustHtmlParserExpander::new()),
        )
    }
}

impl IRustHtmlParserAll for RustHtmlParserAll {
    fn get_html_parser(&self) -> Rc<dyn IRustHtmlParserHtml> {
        self.html_parser.clone()
    }

    fn get_rust_parser(&self) -> Rc<dyn IRustHtmlParserRust> {
        self.rust_parser.clone()
    }

    fn get_rust_or_html_parser(&self) -> Rc<dyn IRustHtmlParserRustOrHtml> {
        self.rust_or_html_parser.clone()
    }

    fn get_converter(&self) -> Rc<dyn IRustHtmlParserConverterIn> {
        self.rust_converter_in.clone()
    }

    fn get_converter_out(&self) -> Rc<dyn IRustHtmlParserConverterOut> {
        self.rust_converter_out.clone()
    }

    fn get_expander(&self) -> Rc<dyn IRustHtmlParserExpander> {
        self.rust_expander.clone()
    }

    fn expand_rust(self: &Self, input: TokenStream, cancellation_token: Rc<dyn ICancellationToken>) -> Result<TokenStream, RustHtmlError> {
        let context: Rc<RustHtmlParserContext> = Rc::new(RustHtmlParserContext::new(false, false, "".to_string()));
        self.expand_rust_with_context(context, input, cancellation_token)
    }

    fn expand_rust_with_context(self: &Self, context: Rc<RustHtmlParserContext>, input: TokenStream, cancellation_token: Rc<dyn ICancellationToken>) -> Result<TokenStream, RustHtmlError> {
        let _scope = CallstackTrackerScope::enter(context.get_call_stack(), nameof::name_of_type!(RustHtmlParserAll), nameof_member_fn!(RustHtmlParserAll::expand_rust_with_context));
        let it = Rc::new(StreamPeekableTokenTree::new(input));
        match self.get_converter().convert_rust(it, context.clone(), cancellation_token.clone()) {
            Ok(input_rshtml) => {
                match self.get_expander().expand_rshtml(context.clone(), Rc::new(VecPeekableRustHtmlToken::new(input_rshtml.clone())), cancellation_token.clone()) {
                    Ok(()) => {
                        match self.get_converter_out().convert_out(context.clone(), cancellation_token) {
                            Ok(output) => {
                                Ok(TokenStream::from_iter(output.into_iter()))
                            },
                            Err(RustHtmlError(err)) => {
                                Err(RustHtmlError::from_string(err.into_owned()))
                            }
                        }
                    },
                    Err(RustHtmlError(err)) => {
                        Err(RustHtmlError::from_string(err.into_owned()))
                    }
                }
            },
            Err(RustHtmlError(err)) => {
                Err(RustHtmlError::from_string(err.into_owned()))
            }
        }
    }
}
