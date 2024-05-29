use std::{cell::RefCell, rc::Rc};

use core_lib::asyncly::icancellation_token::ICancellationToken;

use crate::view::rusthtml::rusthtml_directive_result::{RustHtmlDirectiveResult, RustHtmlDirectiveResultV3};
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::directives::irusthtml_directive::IRustHtmlDirective;
use crate::view::parserv3::parserv3::IParserV3;

use super::iconverter_middle::IConverterMiddle;

pub struct ConverterDirectives {
    directives: Vec<Rc<dyn IRustHtmlDirective>>,
    parser: RefCell<Option<Rc<dyn IParserV3>>>
}

impl ConverterDirectives {
    pub fn new(
        directives: Vec<Rc<dyn IRustHtmlDirective>>,
    ) -> Self {
        Self {
            directives,
            parser: RefCell::new(None)
        }
    }

    pub fn get_parser(&self) -> Rc<dyn IParserV3> {
        self.parser.borrow().as_ref().unwrap().clone()
    }
}

impl IConverterMiddle for ConverterDirectives {
    fn convert(&self,
        input: Rc<dyn IPeekableRustHtmlToken>,
        context: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<Rc<dyn IPeekableRustHtmlToken>, RustHtmlError> {
        // need to peek for name which is an ident
        match input.peek() {
            Some(token) => {
                match token {
                    RustHtmlToken::Identifier(ident) => {
                        // move forward one token
                        input.next();
                        let name = ident.to_string();
                        let directive = self.directives.iter().find(|d| d.matches(&name));
                        match directive {
                            Some(d) => {
                                // execute the directive
                                d.execute_new_v3(context, ident, token, self.get_parser(), input.clone(), ct)
                            }
                            None => {
                                // panic
                                panic!("No directive found for name: {}", name)
                            }
                        }
                    }
                    _ => {
                        // panic
                        panic!("No identifier found after @")
                    }
                }
            }
            None => {
                // panic
                panic!("No token found after @")
            }
        }
    }
}