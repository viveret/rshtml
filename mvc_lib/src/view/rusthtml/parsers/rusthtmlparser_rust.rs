use std::cell::RefCell;
use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{TokenTree, Punct, Delimiter, Group, Ident, TokenStream, Literal};

use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::parsers::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::parsers::peekable_tokentree::StreamPeekableTokenTree;

use super::peekable_tokentree::VecPeekableTokenTree;
use super::rusthtmlparser_all::{IRustHtmlParserAssignSharedParts, IRustHtmlParserAll};


pub trait IRustHtmlParserRust: IRustHtmlParserAssignSharedParts {
    fn parse_rust(self: &Self, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    fn convert(self: &Self, token: TokenTree) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn convert_rust(self: &Self, tokens: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn convert_stream(self: &Self, tokens: TokenStream, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn convert_vec(self: &Self, tokens: Vec<TokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn convert_group(self: &Self, group: &Group, expect_return_html: bool, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn convert_literal(self: &Self, literal: &Literal, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn convert_punct(self: &Self, punct: &Punct) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    // fn loop_next_and_convert_rust(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    fn parse_string_with_quotes(self: &Self, peek_or_next: bool, identifier: &Ident, it: Rc<dyn IPeekableTokenTree>) -> Result<String, RustHtmlError>;
    
    fn parse_type_identifier(self: &Self, it: Rc<dyn IPeekableTokenTree>, cancellation_token: Rc<dyn ICancellationToken>) -> Result<Rc<dyn IPeekableTokenTree>, RustHtmlError>;
    fn parse_rust_identifier_expression(self: &Self, add_first_ident: bool, identifier_token: &TokenTree, last_token_was_ident: bool, it: Rc<dyn IPeekableTokenTree>, cancellation_token: Rc<dyn ICancellationToken>) -> Result<Rc<dyn IPeekableTokenTree>, RustHtmlError>;
    // fn convert_rust_identifier_expression(self: &Self, tokens: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    //fn parse_rust_literal_expression(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    //fn parse_rust_group_expression(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_rust_string_or_ident(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_rust_string_or_ident_or_punct_or_group(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_rust_string_or_ident_or_punct_or_group_or_literal(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    

    // assert that the next token is a punct. if it is, return nothing. otherwise, return the unexpected token.
    // c: the punct to expect.
    // it: the iterator to use.
    // returns: nothing or the unexpected token.
    fn expect_punct(self: &Self, c: char, it: Rc<dyn IPeekableTokenTree>) -> Result<(TokenTree, Punct), Option<TokenTree>>;
}


pub struct RustHtmlParserRust {
    parser: RefCell<Option<Rc<dyn IRustHtmlParserAll>>>,
}

impl RustHtmlParserRust {
    pub fn new() -> Self {
        Self {
            parser: RefCell::new(None),
        }
    }
}

impl IRustHtmlParserAssignSharedParts for RustHtmlParserRust {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        *self.parser.borrow_mut() = Some(parser);
    }
}

impl IRustHtmlParserRust for RustHtmlParserRust {
    fn parse_rust(self: &Self, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        loop {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_str("cancelled"));
            }

            let next_token = it.peek();
            if let Some(ref token) = next_token {
                match token {
                    TokenTree::Ident(ident) => {
                        let tokens = self.parse_rust_identifier_expression(true, token, false, it.clone(), ct.clone())?;
                        output.extend_from_slice(&self.convert_rust(tokens, ct.clone())?);
                    },
                    TokenTree::Punct(punct) => {
                        output.extend_from_slice(&self.convert_punct(&punct)?);
                    },
                    TokenTree::Literal(literal) => {
                        output.extend_from_slice(&self.convert_literal(&literal, ct.clone())?);
                    },
                    TokenTree::Group(group) => {
                        output.extend_from_slice(&self.convert_group(&group, false, ct.clone())?);
                    },
                }
            } else {
                break;
            }
        }
        Ok(output)
    }

    fn parse_type_identifier(self: &Self, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Rc<dyn IPeekableTokenTree>, RustHtmlError> {
        let mut output = vec![];
        loop {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_str("cancelled"));
            }
            let next_token = it.peek();
            if let Some(ref token) = next_token {
                match token {
                    TokenTree::Ident(ident) => {
                        output.push(it.next().unwrap());

                        // peek for next 3 punct tokens
                        // if it is a colon, then push it
                        let mut colons = vec![];
                        for i in 0..3 {
                            if let Some(peek_colon) = it.peek_nth(i) {
                                match &peek_colon {
                                    TokenTree::Punct(punct) => {
                                        match punct.as_char() {
                                            ':' => {
                                                colons.push(peek_colon);
                                            },
                                            _ => break,
                                        }
                                    },
                                    _ => break,
                                }
                            } else {
                                break;
                            }
                        }

                        // if only one is colon, then break
                        // if none are colon, then break
                        // if two then push them to type_parts
                        // if more than two then error
                        match colons.len() {
                            0 => break,
                            1 => break,
                            2 => {
                                it.next();
                                it.next();
                                output.extend_from_slice(&colons);
                            },
                            _ => {
                                return Err(RustHtmlError::from_string(format!("unexpected colon count: {}", colons.len())));
                            }
                        }
                    },
                    TokenTree::Punct(punct) => {
                        match punct.as_char() {
                            '<' => {
                                output.push(it.next().unwrap());
                                let inner = self.parse_type_identifier(it.clone(), ct)?;
                                output.extend_from_slice(inner.to_splice());
                                
                                // assert that next token is '>'
                                match self.expect_punct('>', it) {
                                    Ok((t, _c)) => {
                                        output.push(t);
                                    },
                                    Err(None) => {
                                        return Err(RustHtmlError::from_string(format!("unexpected end of token stream")));
                                    },
                                    Err(Some(token)) => {
                                        return Err(RustHtmlError::from_string(format!("unexpected token: {:?}", token)));
                                    }
                                }
                                break;
                            },
                            ':' => {
                                output.push(it.next().unwrap());
                            },
                            _ => {
                                return Err(RustHtmlError::from_string(format!("unexpected punct: {:?}", token)));
                            }
                        }
                    },
                    _ => {
                        output.push(it.next().unwrap());
                    }
                }
            } else {
                break;
            }
        }
        Ok(Rc::new(VecPeekableTokenTree::new(output)))
    }

    fn convert(self: &Self, token: TokenTree) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        match token {
            TokenTree::Ident(ident) => {
                output.push(RustHtmlToken::Identifier(ident));
            },
            TokenTree::Punct(punct) => {
                output.push(RustHtmlToken::ReservedChar(punct.as_char(), punct));
            },
            TokenTree::Literal(literal) => {
                output.push(RustHtmlToken::Literal(Some(literal), None));
            },
            TokenTree::Group(group) => {
                output.push(RustHtmlToken::Group(group.delimiter(), group));
            },
        }
        Ok(output)
    }

    fn convert_vec(self: &Self, tokens: Vec<TokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        for token in tokens {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_str("cancelled"));
            }
            output.extend_from_slice(self.convert(token)?.as_slice());
        }
        Ok(output)
    }

    fn parse_rust_identifier_expression(self: &Self, add_first_ident: bool, identifier_token: &TokenTree, last_token_was_ident: bool, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Rc<dyn IPeekableTokenTree>, RustHtmlError> {
        let mut output = vec![];
        if add_first_ident {
            output.push(identifier_token.clone());
        }
        // this needs to be an argument
        let mut last_token_was_ident = last_token_was_ident;
        loop {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_str("cancelled"));
            }

            let token_option = it.peek();
            if let Some(token) = token_option {
                match token {
                    TokenTree::Ident(_ident) => {
                        if last_token_was_ident {
                            break;
                        } else {
                            output.push(it.next().unwrap());
                            last_token_was_ident = true;
                            continue;
                        }
                    },
                    TokenTree::Punct(punct) => {
                        let c = punct.as_char();
                        match c {
                            '.' | '?' | '!' | '_' | ':' | '&' => {
                                if last_token_was_ident {
                                    output.push(it.next().unwrap());
                                    last_token_was_ident = false;
                                } else {
                                    break;
                                }
                            },
                            _ => {
                                break;
                            }
                        }
                    },
                    _ => {
                        break;
                    }
                }
            } else {
                break;
            }

            last_token_was_ident = false;
        }
        Ok(Rc::new(VecPeekableTokenTree::new(output)))
    }

    fn parse_rust_string_or_ident(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!("parse_rust_string_or_ident")
    }

    fn parse_rust_string_or_ident_or_punct_or_group(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!("parse_rust_string_or_ident_or_punct_or_group")
    }

    fn parse_rust_string_or_ident_or_punct_or_group_or_literal(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!("parse_rust_string_or_ident_or_punct_or_group_or_literal")
    }

    fn expect_punct(self: &Self, c: char, it: Rc<dyn IPeekableTokenTree>) -> Result<(TokenTree, Punct), Option<TokenTree>> {
        if let Some(actual_c_token) = it.peek() {
            match actual_c_token { 
                TokenTree::Punct(ref punct) => {
                    let actual_c = punct.as_char();
                    if actual_c == c {
                        it.next();
                        Ok((actual_c_token.clone(), punct.clone()))
                    } else {
                        Err(Some(actual_c_token))
                    }
                },
                _ => Err(Some(actual_c_token.clone()))
            }
        } else {
            Err(None)
        }
    }

    fn convert_group(self: &Self, group: &Group, expect_return_html: bool, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        if ct.is_cancelled() {
            return Err(RustHtmlError::from_str("cancelled"));
        }
        
        let delimiter = group.delimiter();
        let mut output = vec![];
        if delimiter == Delimiter::Brace {
            let it = Rc::new(StreamPeekableTokenTree::new(group.stream()));
            let mut inner_tokens = vec![];
            
            // prefix and postfix with html_output decorators
            if expect_return_html {
                let tokens = self.convert_stream(quote::quote! { let html_output = HtmlBuffer::new(); }, ct.clone())?;
                inner_tokens.extend_from_slice(&tokens);
            }
            
            let inner2 = self.parse_rust(it, ct.clone())?;
            inner_tokens.extend_from_slice(&inner2);
            
            if expect_return_html {
                let tokens = self.convert_stream(quote::quote! { html_output.collect_html() }, ct)?;
                inner_tokens.extend_from_slice(&tokens);
            }

            output.push(RustHtmlToken::GroupParsed(delimiter, inner_tokens));
        } else {
            output.push(RustHtmlToken::Group(delimiter, group.clone()));
        }
        Ok(output)
    }

    // fn loop_next_and_convert_rust(self: &Self, it: Rc<dyn IPeekableTokenTree>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
    //     let mut output = vec![];
    //     loop {
    //         let next_token = it.peek();
    //         if let Some(ref token) = next_token {
    //             match token {
    //                 TokenTree::Ident(ident) => {
    //                     let tokens = self.parse_rust_identifier_expression(true, token, false, it.clone())?;
    //                     output.extend_from_slice(&self.parse_rust(tokens)?);
    //                 },
    //                 TokenTree::Punct(punct) => {
    //                     let tokens = self.convert_punct(punct)?;
    //                     output.extend_from_slice(&tokens);
    //                 },
    //                 TokenTree::Literal(literal) => {
    //                     output.extend_from_slice(&self.convert_literal(&literal)?);
    //                 },
    //                 TokenTree::Group(group) => {
    //                     output.extend_from_slice(&self.convert_group(group, false)?);
    //                 },
    //             }
    //         } else {
    //             break;
    //         }
    //     }
    //     Ok(output)
    // }

    fn parse_string_with_quotes(self: &Self, peek_or_next: bool, identifier: &Ident, it: Rc<dyn IPeekableTokenTree>) -> Result<String, RustHtmlError> {
        let r = if peek_or_next { it.peek() } else { it.next() };
        if let Some(expect_string_token) = r {
            match expect_string_token {
                TokenTree::Literal(literal) => Ok(snailquote::unescape(&literal.to_string()).unwrap()),
                _ => Err(RustHtmlError::from_string(format!("unexpected token after {} directive: {:?}", identifier, expect_string_token))),
            }
        } else {
            Err(RustHtmlError::from_string(format!("unexpected end of token stream after {} directive", identifier)))
        }
    }

    fn convert_stream(self: &Self, tokens: TokenStream, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        for token in tokens {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_str("cancelled"));
            }
            output.extend_from_slice(&self.convert(token)?);
        }
        Ok(output)
    }

    fn convert_literal(self: &Self, literal: &Literal, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        Ok(vec![RustHtmlToken::Literal(Some(literal.clone()), None)])
    }

    fn convert_punct(self: &Self, punct: &Punct) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        Ok(vec![RustHtmlToken::ReservedChar(punct.as_char(), punct.clone())])
    }

    fn convert_rust(self: &Self, tokens: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        loop {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_str("cancelled"));
            }

            let next_token = tokens.peek();
            match next_token {
                Some(token) => {
                    output.extend(self.convert(token)?);
                },
                None => {
                    break;
                }
            }
        }
        Ok(output)
    }
}