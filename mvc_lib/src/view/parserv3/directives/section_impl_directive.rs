use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident};

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::RustHtmlDirectiveResultV3;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::irusthtml_directive::IRustHtmlDirective;



// The "impl" directive is used to implement a trait required by the view definition.
// it is similar to the "impl" keyword in Rust, except that the trait must be referenced by an "@implements" directive.
pub struct ImplSectionDirective {}

impl ImplSectionDirective {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlDirective for ImplSectionDirective {
    fn matches(&self, name: &String) -> bool {
        name == "impl"
    }

    // fn execute(&self, context: Rc<dyn IRustHtmlParserContext>, _identifier: &Ident, _ident_token: &TokenTree, _parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, _ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
    //     // expecting group
    //     match it.next() {
    //         Some(group_token) => {
    //             match group_token {
    //                 TokenTree::Group(group) => {
    //                     context.set_impl_section(Some(group.stream()));
    //                     Ok(RustHtmlDirectiveResult::OkContinue)
    //                 },
    //                 _ => {
    //                     Err(RustHtmlError::from_string(format!("unexpected token after impl directive: {:?}", group_token)))
    //                 }
    //             }
    //         },
    //         None => {
    //             Err(RustHtmlError::from_string(format!("unexpected end of input after impl directive")))
    //         }
    //     }
    // }
    
    // fn execute_new(&self, _context: Rc<dyn IRustHtmlParserContext>, _identifier: &Ident, _ident_token: &RustHtmlToken, _parser: Rc<dyn IRustHtmlParserAll>, _output: &mut Vec<RustHtmlToken>, _it: Rc<dyn IPeekableRustHtmlToken>, _ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
    //     todo!("execute_new impl directive")
    // }
    
    // fn execute_old(&self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &TokenTree, parser: Rc<crate::view::rusthtml::rusthtml_parser::RustHtmlParser>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
    //     todo!("execute_old impl directive")
    // }
    
    fn execute_new_v3(&self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, parser: Rc<dyn crate::view::parserv3::parserv3::IParserV3>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        todo!("execute_new_v3 impl directive")
    }
}