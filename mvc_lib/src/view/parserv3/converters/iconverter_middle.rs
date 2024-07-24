use std::{cell::RefCell, rc::Rc};

use core_lib::asyncly::icancellation_token::ICancellationToken;

use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::vec_peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::parserv3::parserv3::IParserV3;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

pub trait IConverterMiddle {
    fn convert(&self,
        input: Rc<dyn IPeekableRustHtmlToken>,
        context: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<Rc<dyn IPeekableRustHtmlToken>, RustHtmlError>;

    fn set_parser(&self, parser: Rc<dyn IParserV3>);
}

pub struct ConverterMiddle {
    middle: Rc<dyn IConverterMiddle>
}

impl ConverterMiddle {
    pub fn new() -> Self {
        Self {
            middle: Rc::new(ConverterMiddleChain::new_default())
        }
    }
}

impl IConverterMiddle for ConverterMiddle {
    fn convert(&self,
        input: Rc<dyn IPeekableRustHtmlToken>,
        context: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<Rc<dyn IPeekableRustHtmlToken>, RustHtmlError> {
        self.middle.convert(input, context, ct)
    }

    fn set_parser(&self, parser: Rc<dyn IParserV3>) {
        self.middle.set_parser(parser);
    }
}

pub struct ConverterMiddleChain {
    chain: Vec<Rc<dyn IConverterMiddle>>
}

impl ConverterMiddleChain {
    pub fn new(chain: Vec<Rc<dyn IConverterMiddle>>) -> Self {
        Self {
            chain
        }
    }

    pub fn new_default() -> Self {
        Self::new(vec![
            Rc::new(ConverterNormal::new()),
            // Rc::new(ConverterMiddle2::new()),
            // Rc::new(ConverterMiddle3::new())
        ])
    }

    pub fn set_parser(&self, parser: Rc<dyn IParserV3>) {
        for converter in &self.chain {
            converter.set_parser(parser.clone());
        }
    }
}

impl IConverterMiddle for ConverterMiddleChain {
    fn convert(&self,
        input: Rc<dyn IPeekableRustHtmlToken>,
        context: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<Rc<dyn IPeekableRustHtmlToken>, RustHtmlError> {
        let mut input = input;
        for converter in &self.chain {
            let result = converter.convert(input, context.clone(), ct.clone());
            match result {
                Ok(new_input) => {
                    input = new_input;
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(input)
    }

    fn set_parser(&self, parser: Rc<dyn IParserV3>) {
        for converter in &self.chain {
            converter.set_parser(parser.clone());
        }
    }
}

pub struct ConverterNormal {
    parser: RefCell<Option<Rc<dyn IParserV3>>>
}

impl ConverterNormal {
    pub fn new() -> Self {
        Self {
            parser: RefCell::new(None)
        }
    }

    fn get_parser(&self) -> Rc<dyn IParserV3> {
        self.parser.borrow().as_ref().expect("could not get parser from ConverterNormal").clone()
    }

    fn convert_html(&self,
        token: &RustHtmlToken,
        input: Rc<dyn IPeekableRustHtmlToken>,
        context: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<Rc<dyn IPeekableRustHtmlToken>, RustHtmlError> {
        match token {
            RustHtmlToken::Group(d, s, g) => {
                // recurse
                match self.convert(s.clone(), context.clone(), ct.clone()) {
                    Ok(new_input) => {
                        Ok(Rc::new(VecPeekableRustHtmlToken::new(vec![RustHtmlToken::Group(d.clone(), new_input, g.clone())])))
                    },
                    Err(e) => {
                        Err(e)
                    }
                }
            },
            RustHtmlToken::Identifier(_) | RustHtmlToken::Literal(_, _) => {
                Ok(Rc::new(VecPeekableRustHtmlToken::new(vec![token.clone()])))
            },
            RustHtmlToken::ReservedChar(c, p) => {
                match c {
                    '@' => {
                        self.get_parser().get_converter_directives().convert(input, context, ct)
                    },
                    '.' | ',' | ';' | '!' | '=' | '<' | '>' | '/' | '&' => {
                        Ok(Rc::new(VecPeekableRustHtmlToken::new(vec![token.clone()])))
                    },
                    _ => {
                        panic!("convert_html unknown punctuation: {}", c);
                    }
                }
            },
            _ => {
                panic!("convert_html unknown token");
            }
        }
    }

    fn convert_rust(&self,
        token: &RustHtmlToken,
        input: Rc<dyn IPeekableRustHtmlToken>,
        context: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<Rc<dyn IPeekableRustHtmlToken>, RustHtmlError> {
        match token {
            RustHtmlToken::Group(d, s, g) => {
                // recurse
                match self.convert(s.clone(), context, ct) {
                    Ok(new_input) => {
                        Ok(Rc::new(VecPeekableRustHtmlToken::new(vec![RustHtmlToken::Group(d.clone(), new_input, g.clone())])))
                    },
                    Err(e) => {
                        Err(e)
                    }
                }
            },
            RustHtmlToken::Identifier(_) | RustHtmlToken::Literal(_, _) => {
                Ok(Rc::new(VecPeekableRustHtmlToken::new(vec![token.clone()])))
            },
            RustHtmlToken::ReservedChar(c, p) => {
                match c {
                    '.' | ',' | ';' | '!' | '=' | '<' | '>' | '/' | '&' => {
                        Ok(Rc::new(VecPeekableRustHtmlToken::new(vec![token.clone()])))
                    },
                    _ => {
                        panic!("convert_rust unknown punctuation: {}", c);
                    }
                }
            },
            _ => {
                panic!("convert_rust unknown token: {:?}", token);
            }
        }
    }
}

impl IConverterMiddle for ConverterNormal {
    fn convert(&self,
        input: Rc<dyn IPeekableRustHtmlToken>,
        context: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<Rc<dyn IPeekableRustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        loop {
            let token = input.next();
            if token.is_none() {
                break;
            }
            let token = token.expect("peeked token");
            let result = if context.get_is_in_html_mode() {
                self.convert_html(&token, input.clone(), context.clone(), ct.clone())
            } else {
                self.convert_rust(&token, input.clone(), context.clone(), ct.clone())
            };
            match result {
                Ok(new_input) => {
                    output.extend(new_input.to_vec());
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(Rc::new(VecPeekableRustHtmlToken::new(output)))
    }

    fn set_parser(&self, parser: Rc<dyn IParserV3>) {
        self.parser.replace(Some(parser));
    }
}