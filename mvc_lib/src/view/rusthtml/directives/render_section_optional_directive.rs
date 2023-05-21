use std::{borrow::Cow, rc::Rc};

use proc_macro::{Ident, TokenTree, Group};

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;

use super::irusthtml_directive::IRustHtmlDirective;


// The "render_section_optional" directive is used to render a section of the view if it exists.
pub struct RenderSectionOptionalDirective {}

impl RenderSectionOptionalDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for RenderSectionOptionalDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "render_section_optional"
    }

    fn execute(self: &Self, ident: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // get the section name
        let section_name = match it.next() {
            Some(token) => {
                match &token {
                    TokenTree::Ident(ident) => ident.to_string(),
                    TokenTree::Literal(literal) => literal.to_string(),
                    TokenTree::Group(group) => {
                        match group.delimiter() {
                            proc_macro::Delimiter::Parenthesis => {
                                let tokens = group.stream().into_iter().take(1).collect::<Vec<TokenTree>>();
                                let token = tokens.get(0).unwrap();
                                match token {
                                    TokenTree::Ident(ident) => ident.to_string(),
                                    TokenTree::Literal(literal) => literal.to_string(),
                                    _ => return Err(RustHtmlError::from_string(format!("The \"{}\" directive requires a section name in the (), not {}.", ident.to_string(), token.to_string())))
                                }
                            },
                            _ => return Err(RustHtmlError::from_string(format!("The \"{}\" directive requires a section name, not {}.", ident.to_string(), token.to_string())))
                        }
                    },
                    _ => return Err(RustHtmlError::from_string(format!("The \"{}\" directive requires a section name.", ident.to_string())))
                }
            }
            _ => return Err(RustHtmlError::from_string(format!("The \"{}\" directive requires a section name.", ident.to_string())))
        };
        
        // get section tokens
        let try_get_section_tokens = parser.get_context().get_section(&section_name);
        if let Some(section_tokens) = try_get_section_tokens {
            // add section tokens to output
            output.push(RustHtmlToken::Group(proc_macro::Delimiter::None, Group::new(proc_macro::Delimiter::None, section_tokens)));
        }

        Ok(RustHtmlDirectiveResult::OkContinue)
    }
}