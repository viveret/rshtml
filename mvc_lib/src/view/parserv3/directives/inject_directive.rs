use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, Span, TokenStream, TokenTree};

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::vec_peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::{RustHtmlDirectiveResult, RustHtmlDirectiveResultV3};
use crate::view::parserv3::parserv3::IParserV3;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};

use super::irusthtml_directive::IRustHtmlDirective;


// The "use" directive is used to import a namespace or type into the view. it is similar to the "use" keyword in Rust.
pub struct InjectDirective {}

impl InjectDirective {
    pub fn new() -> Self {
        Self {}
    }

    pub fn insert_assignment_code(self: &Self, context: Rc<dyn IRustHtmlParserContext>, inject_type: TokenStream, parser: Rc<dyn IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        context.push_inject_statements(quote::quote! {
            ServiceCollectionExtensions::get_required_single::<#inject_type>(services)
        });
        Ok(())
    }
}

impl IRustHtmlDirective for InjectDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "inject"
    }

    fn execute_new_v3(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        // expect name of injected service
        let injected_ident_token = it.next();
        let ident_tokentree = match &injected_ident_token {
            Some(RustHtmlToken::Identifier(i)) => {
                parser.get_converter_out().convert_rusthtmltoken_to_tokentree(&injected_ident_token.unwrap(), ct.clone())?
            },
            _ => return Err(RustHtmlError::from_string(format!("Unexpected token for inject directive identifier: {:?}", injected_ident_token))),
        };

        // then expect "as" keyword
        let separator_tokenrusthtml = match it.peek() {
            Some(as_token) => {
                match as_token {
                    RustHtmlToken::Identifier(as_ident) => {
                        if as_ident.to_string() == "as" {
                            it.next().unwrap()
                        } else {
                            return Err(RustHtmlError::from_string(format!("Unexpected ident after inject directive: {:?}", as_ident)))
                        }
                    },
                    RustHtmlToken::ReservedChar(':', _) => {
                        it.next().unwrap()
                    },
                    _ => {
                        return Err(RustHtmlError::from_string(format!("Unexpected token after inject directive: {:?}", as_token)))
                    },
                }
            },
            None => return Err(RustHtmlError::from_string(format!("Unexpected end of input after inject directive"))),
        };
        let separator_token = parser.get_converter_out().convert_rusthtmltoken_to_tokentree(&separator_tokenrusthtml, ct.clone())?;

        let type_ident_tokens_rusthtml = parser.get_rust_parser().parse_type_identifier(it.clone(), ct.clone())?;
        let type_ident_tokens_rusthtml_stream = Rc::new(VecPeekableRustHtmlToken::new(type_ident_tokens_rusthtml));
        let type_ident_tokens = parser.get_converter_out().convert(type_ident_tokens_rusthtml_stream, ct.clone())?;
        let type_ident_stream = type_ident_tokens.to_stream();

        let inject_stream = quote::quote! {
            let #ident_tokentree #separator_token Rc<#type_ident_stream>
        };
        context.push_inject_statements(inject_stream.clone());

        context.push_inject_statements(quote::quote! { = });
        // insert the assignment code that's automatically generated
        self.insert_assignment_code(context.clone(), type_ident_stream, parser, it, ct.clone())?;
        // append ';' to separate statements?
        context.push_inject_statements(quote::quote! { ; });
        Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, None))
    }
}
