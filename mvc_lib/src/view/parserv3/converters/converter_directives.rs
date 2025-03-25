use core::panic;
use std::{cell::RefCell, rc::Rc};

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Delimiter;

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::vec_peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::{RustHtmlDirectiveResult, RustHtmlDirectiveResultV3};
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::parserv3::parserv3::IParserV3;

use super::iconverter_middle::IConverterMiddle;

pub struct ConverterDirectives {
    parser: RefCell<Option<Rc<dyn IParserV3>>>
}

impl ConverterDirectives {
    pub fn new(
    ) -> Self {
        Self {
            parser: RefCell::new(None)
        }
    }

    pub fn get_parser(&self) -> Rc<dyn IParserV3> {
        self.parser.borrow().as_ref().expect("could not get parser from ConverterDirectives").clone()
    }
}

impl IConverterMiddle for ConverterDirectives {
    fn convert(&self,
        input: Rc<dyn IPeekableRustHtmlToken>,
        context: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        // context.log_info("ConverterDirectives::convert".to_string());
        // need to peek for name which is an ident
        match input.peek() {
            Some(ref token) => {
                // println!("directive convert token: {}", token.to_string());
                match token {
                    RustHtmlToken::Identifier(ident) => {
                        let name = ident.to_string();
                        // println!("{} is an actual directive, checking if exists", name.as_str());
                        let directive = context.try_get_directive(name.clone());
                        match directive {
                            Some(d) => {
                                // println!("directive {} exists", name.as_str());
                                // execute the directive
                                input.next();
                                let v3result = d.execute_new_v3(context, &ident, token, self.get_parser(), input.clone(), ct)?;
                                if let Some(v3result_stream) = &v3result.1 {
                                    // println!("output of {} directive:", name);
                                    // for t in v3result_stream.to_vec().into_iter() {
                                    //     print!("{} ", t.to_string());
                                    // }
                                    // println!();
                                }
                                return Ok(v3result);
                            }
                            None => {
                                let exp = self.get_parser().get_rust_parser().parse_expression(input, context.clone(), ct)?;
                                return Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, Some(Rc::new(VecPeekableRustHtmlToken::new(vec![RustHtmlToken::AppendToHtml(exp)])))));
                            }
                        }
                    },
                    RustHtmlToken::ReservedChar(c, p) => {
                        match c {
                            '@' => {
                                println!("escaped @");
                                input.next();
                                return Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, Some(Rc::new(VecPeekableRustHtmlToken::new(vec![token.clone()])))));
                            },
                            '&' => {
                                input.next();
                                let mut tokens = vec![token.clone()];
                                // recurse to get the next token
                                let next = self.convert(input.clone(), context.clone(), ct.clone())?;
                                if let Some(next_stream) = next.1 {
                                    while let Some(t) = next_stream.next() {
                                        tokens.push(t.clone());
                                    }
                                }
                                return Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, Some(Rc::new(VecPeekableRustHtmlToken::new(tokens)))));
                            }
                            _ => {
                                panic!("Cannot handle reserved char {:?} after @", c)
                            }
                        }
                    },
                    RustHtmlToken::Literal(l, p) => {
                        input.next();
                        return Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, Some(Rc::new(VecPeekableRustHtmlToken::new(vec![RustHtmlToken::AppendToHtml(vec![token.clone()])])))));
                    },
                    RustHtmlToken::Group(delimiter, stream, group) => {
                        match delimiter {
                            Delimiter::Brace => {
                                input.next();
                                self.get_parser().get_converter_middle().convert(stream.clone(), context, ct)
                            },
                            _ => {
                                // panic
                                panic!("Cannot handle group type {:?} after @", delimiter)
                            }
                        }
                    },
                    _ => {
                        // panic
                        panic!("No identifier found after @ (found {:?})", token)
                    }
                }
            }
            None => {
                // panic
                panic!("No token found after @")
            }
        }
    }

    fn set_parser(&self, parser: Rc<dyn IParserV3>) {
        self.parser.replace(Some(parser));
    }
}