use std::cell::RefCell;
use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use core_lib::sys::call_tracker::CallstackTrackerScope;
use core_macro_lib::{callstack_tracker_scope_and_assert, nameof_member_fn};
use proc_macro2::{TokenTree, Punct, Delimiter, Group, Ident, TokenStream, Literal};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;

use super::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use super::rusthtmlparser_all::{IRustHtmlParserAssignSharedParts, IRustHtmlParserAll};


pub trait IRustHtmlParserExpander: IRustHtmlParserAssignSharedParts {
    fn convert_rust_to_rusthtml(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn expand_rshtml(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    fn expand_rshtmltoken(&self, token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn expand_rshtml_punct(&self, punct: &Punct, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
}

pub struct RustHtmlParserExpander {
    parser: RefCell<Option<Rc<dyn IRustHtmlParserAll>>>,
}

impl RustHtmlParserExpander {
    pub fn new() -> Self {
        Self {
            parser: RefCell::new(None),
        }
    }

    fn expand_rshtmltoken(&self, token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>{
        callstack_tracker_scope_and_assert!(ctx, RustHtmlParserExpander::expand_rshtmltoken);
        let parserparser = self.get_parser().get_rust_or_html_parser();
        let r = parserparser.parse_rust_or_html(it, ctx.clone(), ct);
        match r {
            Ok(tokens) => {
                Ok(tokens)
            },
            Err(RustHtmlError(err)) => {
                Err(RustHtmlError::from_string(err.into_owned()))
            }
        }
    }

    fn get_parser(&self) -> Rc<dyn IRustHtmlParserAll> {
        self.parser.borrow().as_ref().expect("self.parser was None").clone()
    }
}

impl IRustHtmlParserAssignSharedParts for RustHtmlParserExpander {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        *self.parser.borrow_mut() = Some(parser);
    }
}

impl IRustHtmlParserExpander for RustHtmlParserExpander {
    fn convert_rust_to_rusthtml(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        callstack_tracker_scope_and_assert!(ctx, RustHtmlParserExpander::convert_rust_to_rusthtml);

        let mut output = vec![];
        loop {
            if ct.is_cancelled() {
                let callstack = ctx.get_call_stack().to_string();
                return Err(RustHtmlError::from_string(format!("parse_rust cancelled at {}", callstack)));
            }

            let next_token = it.peek();
            if let Some(ref token) = next_token {
                match token {
                    ::Ident(ident) => {
                        // consume the token from the stream
                        it.next();
                        let converter = self.get_parser().get_converter();
                        match converter.convert_ident(&ident) {
                            Ok(ident) => {
                                output.push(ident);
                            },
                            Err(RustHtmlError(err)) => {
                                return Err(RustHtmlError::from_string(err.into_owned()));
                            }
                        }
                    },
                    TokenTree::Punct(punct) => {
                        // consume the token from the stream
                        it.next(); // this should be conditional in case we peek a terminal token
                        // for example, <div>hello</div> should not consume the </div> token
                        // let converter = self.get_parser().get_converter();
                        // match converter.convert_punct(&punct) {
                        //     Ok(punct) => {
                        //         output.push(punct);
                        //     },
                        //     Err(RustHtmlError(err)) => {
                        //         return Err(RustHtmlError::from_string(err.into_owned()));
                        //     }
                        // }
                        self.expand_rshtml_punct(punct, it.clone(), ctx.clone(), ct.clone())?;
                    },
                    TokenTree::Literal(literal) => {
                        // consume the token from the stream
                        it.next();
                        let converter = self.get_parser().get_converter();
                        match converter.convert_literal(&literal, ct.clone()) {
                            Ok(literal) => {
                                output.push(literal);
                            },
                            Err(RustHtmlError(err)) => {
                                return Err(RustHtmlError::from_string(err.into_owned()));
                            }
                        }
                    },
                    TokenTree::Group(group) => {
                        // consume the token from the stream
                        it.next();
                        let converter = self.get_parser().get_converter();
                        match converter.convert_group(&group, false, ctx.clone(), ct.clone()) {
                            Ok(group) => {
                                output.push(group);
                            },
                            Err(RustHtmlError(err)) => {
                                return Err(RustHtmlError::from_string(err.into_owned()));
                            }
                        }
                    },
                }
            } else {
                break;
            }
        }
        Ok(output)
    }

    fn expand_rshtml(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        callstack_tracker_scope_and_assert!(ctx, RustHtmlParserExpander::expand_rusthtml);
        let mut output = vec![];

        loop {
            let next_token = it.peek();
            match next_token {
                Some(token) => {
                    let tokens = self.expand_rshtmltoken(token, it.clone(), ctx.clone(), ct.clone())?;
                    output.extend_from_slice(&tokens);
                },
                None => {
                    break;
                }
            }
        }

        Ok(output)
    }

    fn expand_rshtmltoken(&self, token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        callstack_tracker_scope_and_assert!(ctx, RustHtmlParserExpander::expand_rshtmltoken);
        
        match token {
            RustHtmlToken::ReservedChar(c, p) => {
                self.expand_rshtml_punct(p, it.clone(), ctx.clone(), ct.clone())
            },
            _ => {
                Err(RustHtmlError::from_string(format!("Unexpected token: {:?}", token)))
            }
        }
    }

    fn expand_rshtml_punct(&self, punct: &Punct, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let c = punct.as_char();
        match c {
            '@' => {
                self.expand_rust_entry_to_rusthtmltoken(c, punct, is_in_html_mode, output, it, ctx.clone(), ct)?
            },
            '<' => {
                self.expand_html_entry_to_rusthtmltoken(c, punct, true, output, it, ctx.clone(), ct)?
            },
            '}' if !is_in_html_mode => {
                return Ok(true); // do not continue
            },
            '>' if !is_in_html_mode => {
                output.push(RustHtmlToken::ReservedChar(c, punct.clone()));
            },
            '|' if !is_in_html_mode => {
                output.push(RustHtmlToken::ReservedChar(c, punct.clone()));

                // peek ahead to see if this is a || -> or something else
                if self.peek_reserved_chars_in_str("|->", output, it.clone())? {
                    // peek for HtmlString identifier that signals the function will return HtmlString
                    if let Some(next_token) = it.peek() {
                        match next_token {
                            RustHtmlToken::Identifier(next_ident) => {
                                if next_ident.to_string() == "HtmlString" {
                                    // this is a function that returns HtmlString
                                    it.next();
                                    output.push(RustHtmlToken::Identifier(next_ident.clone()));

                                    // parse the rest of the function, which should be in a {} group
                                    if let Some(group_token) = it.next() {
                                        match group_token {
                                            RustHtmlToken::Group(d, stream, group) if *d == Delimiter::Brace => {
                                                self.convert_group_to_rusthtmltoken(d, group, stream.clone(), true, is_in_html_mode, output, ctx.clone(), ct)?;
                                                return Ok(false);
                                            },
                                            _ => {
                                                return Err(RustHtmlError::from_string(format!("Expected {{ after |->")));
                                            }
                                        }
                                    } else {
                                        return Err(RustHtmlError::from_string(format!("Expected {{ after |->")));
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }
            },
            _ => {
                if is_in_html_mode {
                    output.push(RustHtmlToken::HtmlTextNode(punct.as_char().to_string(), punct.span().clone()));
                } else {
                    output.push(RustHtmlToken::ReservedChar(c, punct.clone()));
                }
            },
        }
    }
}