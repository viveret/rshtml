use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenTree, Delimiter};

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::RustHtmlDirectiveResultV3;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};

use super::irusthtml_directive::IRustHtmlDirective;



// the "while" directive is used to create a while loop in the view.
// It will loop over the contents of the directive until the condition is false.
pub struct WhileDirective {}

impl WhileDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for WhileDirective {
    fn matches(&self, name: &String) -> bool {
        name == "while"
    }

    // fn execute(&self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, _ident_token: &TokenTree, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
    //     output.push(RustHtmlToken::Identifier(identifier.clone()));
        
    //     // read until we reach the loop body {}
    //     loop {
    //         if let Some(token) = it.peek() {
    //             match token {
    //                 TokenTree::Ident(ident) => {
    //                     output.push(RustHtmlToken::Identifier(ident.clone()));
    //                     it.next();
    //                 },
    //                 TokenTree::Literal(literal) => {
    //                     output.push(RustHtmlToken::Literal(Some(literal.clone()), None));
    //                     it.next();
    //                 },
    //                 TokenTree::Punct(punct) => {
    //                     output.push(RustHtmlToken::ReservedChar(punct.as_char(), punct.clone()));
    //                     it.next();
    //                 },
    //                 TokenTree::Group(group) => {
    //                     let delimiter = group.delimiter();
    //                     match delimiter {
    //                         Delimiter::Brace => {
    //                             match parser.get_converter().convert_group(&group, false, context, ct) {
    //                                 Ok(tokens) => {
    //                                     output.push(tokens);
    //                                     break;
    //                                 },
    //                                 Err(RustHtmlError(err)) => {
    //                                     return Err(RustHtmlError::from_string(err.to_string()));
    //                                 }
    //                             }
    //                         },
    //                         _ => {
    //                             panic!("unexpected group delimiter: {:?}", delimiter);
    //                         }
    //                     }
    //                 }
    //             }
    //         } else {
    //             break;
    //         }
    //     }

    //     Ok(RustHtmlDirectiveResult::OkContinue)
    // }
    
    fn execute_new_v3(&self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        todo!("execute_new_v3 while directive")
    }
}