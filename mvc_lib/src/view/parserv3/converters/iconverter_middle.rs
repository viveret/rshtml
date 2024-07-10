use std::{cell::RefCell, rc::Rc};

use core_lib::asyncly::icancellation_token::ICancellationToken;

use crate::view::{parserv3::parserv3::IParserV3, rusthtml::{irusthtml_parser_context::IRustHtmlParserContext, parser_parts::peekable_rusthtmltoken::{IPeekableRustHtmlToken, VecPeekableRustHtmlToken}, rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken}};

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
                todo!("convert_html group")
            },
            RustHtmlToken::Identifier(_i) => {
                todo!("convert_html identifier")
            },
            RustHtmlToken::ReservedChar(c, p) => {
                todo!("convert_html punctuation")
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
                todo!("convert_rust group")
            },
            RustHtmlToken::Identifier(i) => {
                todo!("convert_rust identifier")
            },
            RustHtmlToken::ReservedChar(c, p) => {
                match c {
                    '@' => {
                        self.get_parser().get_converter_directives().convert(input, context, ct)
                    },
                    _ => {
                        panic!("convert_rust unknown punctuation: {}", c);
                    }
                }
            },
            _ => {
                panic!("convert_rust unknown token");
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
                self.convert_html(token, input.clone(), context.clone(), ct.clone())
            } else {
                self.convert_rust(token, input.clone(), context.clone(), ct.clone())
            };
            match result {
                Ok(new_input) => {
                    output.extend_from_slice(new_input.to_splice());
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