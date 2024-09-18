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
        if self.chain.is_empty() {
            panic!("no converters in ConverterMiddleChain");
        }
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
        if ct.is_cancelled() {
            return Err(RustHtmlError::from_cancellationtoken(ct));
        }

        // println!("convert_html: {}", token.to_string());

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
                        context.push_is_in_html_mode(false);
                        let x = self.get_parser().get_converter_directives().convert(input, context.clone(), ct);
                        context.pop_is_in_html_mode();
                        x
                    },
                    '.' | ',' | ':' | ';' | '!' | '=' | '+' | '-' | '/' | '\\' | '|' | '&' => {
                        Ok(Rc::new(VecPeekableRustHtmlToken::new(vec![token.clone()])))
                    },
                    '<' => {
                        // start of tag
                        // what happened to the tag parser?
                        // peek after start of tag
                        // println!("peek tag start: {:?}", input.peek().unwrap());
                        let result = self.get_parser().get_html_parser().parse_tag(input, context, ct)?;
                        if let Some(x) = result.1 {
                            return Ok(x);
                        } else {
                            panic!("oops");
                        }
                    }
                    _ => {
                        let next_token = input.next();
                        let bt = std::backtrace::Backtrace::capture();
                        panic!("convert_html unknown punctuation: {}, next token: {:?}\n{}", c, next_token, ToString::to_string(&bt).as_str());
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
        if ct.is_cancelled() {
            return Err(RustHtmlError::from_cancellationtoken(ct));
        }

        // println!("convert_rust: {}", token.to_string());

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
                    '.' | ',' | ';' | ':' | '!' | '=' | '>' | '/' | '&' | '|' | '-' => {
                        Ok(Rc::new(VecPeekableRustHtmlToken::new(vec![token.clone()])))
                    },
                    '<' => {
                        // check if tag name
                        // let possible_tag_name = input.peek();
                        // if let Some(RustHtmlToken::Identifier(i)) = possible_tag_name {
                        //     match i.to_string().as_str() {
                        //         "h1" | "h2" | "h3" | "p" | "body" | "html" | "div" => {
                        //             panic!("are you sure? {}", Backtrace::capture());
                        //         },
                        //         _ => {
                        //         }
                        //     }
                        // }

                        // is ok
                        // Ok(Rc::new(VecPeekableRustHtmlToken::new(vec![token.clone()])))

                        // assume start of HTML?
                        context.push_is_in_html_mode(true);
                        let r = self.convert_html(token, input, context.clone(), ct);
                        context.pop_is_in_html_mode();
                        r
                    },
                    '@' => {
                        let append_token = RustHtmlToken::AppendToHtml(
                            self.get_parser().get_rust_parser().parse_expression(input, context, ct)?
                        );
                        Ok(Rc::new(VecPeekableRustHtmlToken::new(vec![append_token])))
                    },
                    _ => {
                        let next_token = input.next();
                        panic!("convert_rust unknown punctuation: {}, next token: {:?}", c, next_token);
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
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_cancellationtoken(ct.clone()));
            }

            let token = input.next();
            if token.is_none() {
                break;
            }
            let token = token.expect("peeked token");
            let is_in_html_mode = context.get_is_in_html_mode();
            let result = if is_in_html_mode {
                self.convert_html(&token, input.clone(), context.clone(), ct.clone())
            } else {
                self.convert_rust(&token, input.clone(), context.clone(), ct.clone())
            };
            match result {
                Ok(result_output) => {

                    // check output does not start with @
                    if let Some(token) = result_output.peek() {
                        if let RustHtmlToken::ReservedChar(c, p) = token {
                            if c == '@' {
                                panic!("wtf (is_in_html_mode = {})", is_in_html_mode);
                            }
                        }
                    }

                    output.extend(result_output.to_vec());
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