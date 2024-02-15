use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenStream, TokenTree};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::rusthtml::parser_parts::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;

use super::irusthtml_directive::IRustHtmlDirective;


// The "use" directive is used to import a namespace or type into the view. it is similar to the "use" keyword in Rust.
pub struct InjectDirective {}

impl InjectDirective {
    pub fn new() -> Self {
        Self {}
    }

    fn parse_identifier_for_variable_name(
        self: &Self,
        context: Rc<dyn IRustHtmlParserContext>,
        type_ident_tokens: Rc<dyn IPeekableRustHtmlToken>,
        parser: Rc<dyn IRustHtmlParserAll>,
        _output: &mut Vec<RustHtmlToken>,
        it: &Rc<dyn IPeekableRustHtmlToken>,
        ct: Rc<dyn ICancellationToken>,
    ) -> Result<RustHtmlDirectiveResult, RustHtmlError<'static>> {
        if let Some(RustHtmlToken::Identifier(inject_name)) = it.next() {
            if let Some(RustHtmlToken::Identifier(type_ident)) = it.next() {
                let rust = quote::quote! { let #inject_name = #type_ident ::new(view_context, services); }.into();
                // let converted = parser.get_converter().convert_stream(rust, ct.clone());
                // use if let instead
                if let Ok(converted) = parser.get_converter().convert_stream(rust, context.clone(), ct.clone()) {
                    context.push_inject_statements_rshtml(converted, parser, context.clone(), ct);
                    Ok(RustHtmlDirectiveResult::OkContinue)
                } else {
                    Err(RustHtmlError::from_string(format!("Error converting inject directive to Rust")))
                    
                }
            } else {
                Err(RustHtmlError::from_string(format!("Unexpected token after inject directive: {:?}", it.peek())))
            }
        } else {
            Err(RustHtmlError::from_string(format!("Unexpected end of input after inject directive")))
        }
    }
}

impl IRustHtmlDirective for InjectDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "inject"
    }

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, _: &Ident, _ident_token: &RustHtmlToken, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // expecting type identifier
        if let Ok(type_ident_tokens) = parser.get_rust_parser().parse_type_identifier(it.clone(), ct.clone()) {
            // next token should be "as"
            if let Some(ref as_token) = it.peek() {
                match as_token {
                    RustHtmlToken::Identifier(ident) => {
                        if ident.to_string() == "as" {
                            it.next();
                            // next token should be identifier for the injected variable
                            self.parse_identifier_for_variable_name(context, type_ident_tokens, parser, output, &it, ct.clone())
                        } else {
                            Err(RustHtmlError::from_string(format!("Unexpected ident after inject directive: {:?}", ident)))
                        }
                    },
                    RustHtmlToken::ReservedChar(c, punct) => {
                        match punct.as_char() {
                            ':' => {
                                it.next();
                                // next token should be identifier for the injected variable
                                self.parse_identifier_for_variable_name(context, type_ident_tokens, parser, output, &it, ct.clone())
                            },
                            _ => {
                                Err(RustHtmlError::from_string(format!("Unexpected punct after inject directive: {:?}", punct)))
                            }
                        }
                    },
                    _ => Err(RustHtmlError::from_string(format!("Unexpected token after inject directive: {:?}", as_token))),
                }
            } else {
                Err(RustHtmlError::from_string(format!("Unexpected end of input after inject directive")))
            }
        } else {
            Err(RustHtmlError::from_str("Error parsing use directive"))
        }
    }
}
