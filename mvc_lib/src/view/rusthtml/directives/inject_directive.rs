use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenStream, TokenTree};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parsers::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::parsers::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;

use super::irusthtml_directive::IRustHtmlDirective;


// The "use" directive is used to import a namespace or type into the view. it is similar to the "use" keyword in Rust.
pub struct InjectDirective {}

impl InjectDirective {
    pub fn new() -> Self {
        Self {}
    }

    fn parse_identifier_for_variable_name(self: &Self, context: Rc<dyn IRustHtmlParserContext>, type_ident_tokens: Rc<dyn IPeekableTokenTree>, parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, it: &Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError<'static>> {
        if let Some(inject_name_token) = it.next() {
            match &inject_name_token {
                TokenTree::Ident(_) => {
                    let mut inject_name_vec: Vec<TokenTree> = Vec::new();
                    inject_name_vec.push(inject_name_token.clone());
        
                    let inject_name_tokenstream = proc_macro2::TokenStream::from(TokenStream::from_iter(inject_name_vec));
                    let type_ident_tokenstream = type_ident_tokens.to_stream();
                    context.mut_inject_statements().push(quote::quote! { let #inject_name_tokenstream = #type_ident_tokenstream ::new(view_context, services); }.into());
                    Ok(RustHtmlDirectiveResult::OkContinue)
                },
                _ => {
                    Err(RustHtmlError::from_string(format!("Unexpected token for variable name after inject directive: {:?}", inject_name_token)))
                }
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

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, _: &Ident, _ident_token: &TokenTree, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // expecting type identifier
        if let Ok(type_ident_tokens) = parser.get_rust_parser().parse_type_identifier(it.clone(), ct) {
            // next token should be "as"
            if let Some(ref as_token) = it.peek() {
                match as_token {
                    TokenTree::Ident(ident) => {
                        if ident.to_string() == "as" {
                            it.next();
                            // next token should be identifier for the injected variable
                            self.parse_identifier_for_variable_name(context, type_ident_tokens, parser, output, &it)
                        } else {
                            Err(RustHtmlError::from_string(format!("Unexpected ident after inject directive: {:?}", ident)))
                        }
                    },
                    TokenTree::Punct(punct) => {
                        match punct.as_char() {
                            ':' => {
                                it.next();
                                // next token should be identifier for the injected variable
                                self.parse_identifier_for_variable_name(context, type_ident_tokens, parser, output, &it)
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
