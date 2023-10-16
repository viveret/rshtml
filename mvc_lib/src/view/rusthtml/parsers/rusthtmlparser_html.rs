use std::{cell::RefCell, borrow::Cow};
use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use core_macro_lib::nameof_member_fn;
use proc_macro2::{TokenTree, Literal, Punct};

use crate::view::rusthtml::html_tag_parse_context::HtmlTagParseContext;
use crate::view::rusthtml::ihtml_tag_parse_context::IHtmlTagParseContext;
use crate::view::rusthtml::parsers::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::rusthtmlparser_all::{IRustHtmlParserAll, IRustHtmlParserAssignSharedParts};



pub trait IRustHtmlParserHtml: IRustHtmlParserAssignSharedParts {
    fn parse_html(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableTokenTree>, cancellation_token: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    fn parse_html_tag(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, cancellation_token: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_html_node(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableTokenTree>, cancellation_token: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_html_attr_key(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, cancellation_token: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_html_attr_val(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, cancellation_token: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    fn parse_html_child_nodes(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, cancellation_token: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    fn on_kvp_defined(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, cancellation_token: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    // called when a HTML tag is parsed.
    // punct: the punct token.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // returns: whether we should break the outer loop or not, or an error.
    fn on_html_tag_parsed(
        self: &Self,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        output: &mut Vec<RustHtmlToken>,
        cancellation_token: Rc<dyn ICancellationToken>
    ) -> Result<bool, RustHtmlError>;

    // convert a Rust literal to a RustHtml token in the context of a HTML tag.
    // literal: the literal to convert.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // returns: nothing or an error.
    fn convert_html_literal_to_rusthtmltoken(
        self: &Self, 
        literal: &Literal,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    // convert a Rust punct to a RustHtml token in the context of a HTML tag.
    // punct: the punct to convert.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: whether we should break the outer loop or not, or an error.
    fn convert_html_punct_to_rusthtmltoken(
        self: &Self, 
        punct: &Punct,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        output: &mut Vec<RustHtmlToken>, 
        it: Rc<dyn IPeekableTokenTree>,
        cancellation_token: Rc<dyn ICancellationToken>
    ) -> Result<bool, RustHtmlError>;
}


pub struct RustHtmlParserHtml {
    parser: RefCell<Option<Rc<dyn IRustHtmlParserAll>>>,
}

impl RustHtmlParserHtml {
    pub fn new() -> Self {
        Self {
            parser: RefCell::new(None),
        }
    }
}



impl IRustHtmlParserAssignSharedParts for RustHtmlParserHtml {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        *self.parser.borrow_mut() = Some(parser);
    }
}

impl IRustHtmlParserHtml for RustHtmlParserHtml {
    // TODO: add tests
    fn parse_html(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        loop {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_string(format!("parse_html cancelled")));
            }

            let next_token = it.peek();
            if let Some(token) = next_token {
                match token {
                    TokenTree::Group(group) => {
                        todo!("parse_html TokenTree::Group")
                    },
                    TokenTree::Ident(ident) => {
                        todo!("parse_html TokenTree::Ident")
                    },
                    TokenTree::Literal(literal) => {
                        todo!("parse_html TokenTree::Literal")
                    },
                    TokenTree::Punct(punct) => {
                        todo!("parse_html TokenTree::Punct")
                    },
                }
            }
        }
        Ok(output)
    }

    fn parse_html_tag(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        // parse tag name
        // parse attributes (key value pairs)
        // parse closing puncts
        todo!("parse_html_tag")
    }

    // TODO: add tests
    fn parse_html_node(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        let mut parse_ctx = Rc::new(HtmlTagParseContext::new(Some(ctx))) as Rc<dyn IHtmlTagParseContext>;
        todo!("parse_html_node");
        Ok(output)
    }

    // TODO: add tests
    fn parse_html_attr_key(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        // must be an identifier punct combo
        let mut output = vec![];
        let mut last_was_ident = false;
        loop {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_string(format!("parse_html_attr_key cancelled")));
            }

            let token = it.peek();
            if let Some(ref token) = token {
                match token {
                    TokenTree::Ident(ident) => {
                        if last_was_ident {
                            break;
                        } else {
                            output.push(RustHtmlToken::ReservedIndent(ident.to_string(), ident.clone()));
                            it.next();
                            last_was_ident = true;
                        }
                    },
                    TokenTree::Punct(punct) => {
                        let c = punct.as_char();
                        match c {
                            '-' | '_' => {
                                output.push(RustHtmlToken::ReservedChar(punct.as_char(), punct.clone()));
                                it.next();
                                last_was_ident = false;
                            },
                            _ => {
                                return Err(RustHtmlError::from_string(format!("Unexpected token while parsing HTML tag attr key '{}' attribute key: {:?}", ctx.tag_name_as_str(), token)));
                            }
                        }
                    },
                    _ => {
                        return Err(RustHtmlError::from_string(format!("Unexpected token while parsing HTML tag attr key '{}' attribute key: {:?}", ctx.tag_name_as_str(), token)));
                    }
                }
            }
        }
        Ok(output)
    }

    // TODO: add tests
    fn parse_html_attr_val(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        // can only be literal or identifier punct combo
        let mut output = vec![];
        let mut last_was_ident = false;
        loop {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_string(format!("parse_html_attr_val cancelled")));
            }

            let token = it.peek();
            if let Some(ref token) = token {
                match token {
                    TokenTree::Ident(ident) => {
                        if last_was_ident {
                            break;
                        } else {
                            output.push(RustHtmlToken::ReservedIndent(ident.to_string(), ident.clone()));
                            it.next();
                            last_was_ident = true;
                        }
                    },
                    TokenTree::Punct(punct) => {
                        let c = punct.as_char();
                        match c {
                            '-' | '_' => {
                                output.push(RustHtmlToken::ReservedChar(punct.as_char(), punct.clone()));
                                it.next();
                                last_was_ident = false;
                            },
                            _ => {
                                return Err(RustHtmlError::from_string(format!("Unexpected token while parsing HTML tag attr val '{}' attribute val: {:?}", ctx.tag_name_as_str(), token)));
                            }
                        }
                    },
                    TokenTree::Literal(literal) => {
                        output.push(RustHtmlToken::Literal(Some(literal.clone()), None));
                        it.next();
                        break;
                    },
                    _ => {
                        return Err(RustHtmlError::from_string(format!("Unexpected token while parsing HTML tag attr val '{}' attribute val: {:?}", ctx.tag_name_as_str(), token)));
                    }
                }
            }
        }
        Ok(output)
    }

    fn parse_html_child_nodes(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!("parse_html_child_nodes")
    }

    fn convert_html_literal_to_rusthtmltoken(
        self: &Self, 
        literal: &Literal,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        parse_ctx.add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::convert_html_literal_to_rusthtmltoken), literal.to_string()));
        
        if parse_ctx.is_parsing_attrs() {
            if parse_ctx.is_parsing_attr_val() {
                if parse_ctx.is_key_defined() {
                    parse_ctx.set_html_attr_val_literal(literal);
                    self.on_kvp_defined(parse_ctx, ct)
                } else {
                    Err(RustHtmlError::from_string(format!("was supposed to call on_kvp_defined but key was None (literal: {:?})", literal)))
                }
            } else {
                parse_ctx.set_html_attr_key_literal(literal);
                let s = snailquote::unescape(&literal.to_string()).unwrap();
                parse_ctx.html_attr_key_push_str(&s);
                parse_ctx.set_parse_attr_val(true);
                Ok(vec![])
            }
        } else {
            Err(RustHtmlError(Cow::Owned(format!("Cannot use literal for tag name"))))
        }
    }

    fn on_kvp_defined(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let r = ctx.on_kvp_defined();
        match r {
            Ok(x) => {
                Ok(x)
            },
            Err(RustHtmlError(e)) => {
                Err(RustHtmlError::from_string(format!("error on_kvp_defined: {}", e)))
            }
        }
    }

    fn convert_html_punct_to_rusthtmltoken(
        self: &Self, 
        punct: &Punct,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        output: &mut Vec<RustHtmlToken>, 
        it: Rc<dyn IPeekableTokenTree>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<bool, RustHtmlError> {
        if ct.is_cancelled() {
            return Err(RustHtmlError::from_string(format!("convert_html_punct_to_rusthtmltoken cancelled")));
        }

        let c = punct.as_char();
        if parse_ctx.is_parsing_attrs() {
            parse_ctx.get_main_context().add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::convert_html_punct_to_rusthtmltoken), c));
            match c {
                '>' => {
                    return self.on_html_tag_parsed(parse_ctx, output, ct);
                },
                '=' => {
                    if parse_ctx.is_key_defined() {
                        parse_ctx.set_equals_punct(punct);
                    } else {
                        // need some context here
                        let next_token = it.peek().unwrap();
                        return Err(RustHtmlError::from_string(format!("convert_html_punct_to_rusthtmltoken Unexpected '=' before {:?} (key was None)", next_token)));
                    }
                },
                '/' => {
                    let expect_closing_punct = it.next().unwrap();
                    match expect_closing_punct {
                        TokenTree::Punct(closing_punct) => {
                            if closing_punct.as_char() == '>' {
                                parse_ctx.set_is_self_contained_tag(true);
                                parse_ctx.add_tag_end_punct(punct);
                                return self.on_html_tag_parsed(parse_ctx, output, ct);
                            } else {
                                return Err(RustHtmlError::from_string(format!("convert_html_punct_to_rusthtmltoken Unexpected character '{}' (expected '>', prev: '{}')", closing_punct, c)));
                            }
                        },
                        _ => {
                            return Err(RustHtmlError::from_string(format!("convert_html_punct_to_rusthtmltoken Unexpected token after /: {}", c)));
                        },
                    }
                },
                '"' => {
                    if parse_ctx.has_html_attr_key() {
                        parse_ctx.set_parse_attr_val(true);
                    } else if parse_ctx.has_html_attr_val() {
                        let tokens = self.on_kvp_defined(parse_ctx, ct)?;
                        output.extend(tokens);
                    }
                },
                '-' => {
                    if parse_ctx.is_parsing_attr_val() {
                        parse_ctx.html_attr_val_ident_push_punct(punct);
                    } else {
                        parse_ctx.html_attr_key_ident_push_punct(punct);
                        parse_ctx.html_attr_key_push_str(format!("{}", c).as_str());
                    }
                },
                '@' => {
                    // escaping the html to insert value
                    let directive_token = it.next().unwrap();

                    // fixme: this needs to be fixed, it is not checking directive logic
                    match directive_token {
                        TokenTree::Ident(ref ident) => {
                            let parser = self.parser.borrow().as_ref().unwrap().get_rust_parser();
                            match parser
                                .parse_rust_identifier_expression(
                                    true, 
                                    &directive_token,
                                    false,
                                    it, ct.clone()) {
                                Ok(rust_ident_exp) => {
                                    let parser = self.parser.borrow().as_ref().unwrap().get_rust_or_html_parser();
                                    let rushtml_ident_expr = parser.convert_vec(&rust_ident_exp.to_splice().to_vec());

                                    parse_ctx.set_html_attr_val_rust(rushtml_ident_expr);
                                },
                                Err(e) => {
                                    return Err(RustHtmlError::from_string(format!("error parsing rust ident exp: {}", e)));
                                }
                            }
                        },
                        TokenTree::Literal(ref literal) => {
                            parse_ctx.set_html_attr_val_literal(literal);
                        },
                        _ => {
                            return Err(RustHtmlError::from_string(format!("Unexpected directive token after '@' in html attribute val parse: {:?}", directive_token)));
                        }
                    }

                    // can't just call this, need to wrap in if
                    if parse_ctx.is_kvp_defined() {
                        let tokens = self.on_kvp_defined(parse_ctx, ct)?;
                        output.extend(tokens);
                    }
                },
                _ => {
                    let current_val = if parse_ctx.has_html_attr_val_ident() {
                        format!("ignoring {:?}", parse_ctx.get_html_attr_val_ident())
                    } else {
                        parse_ctx.get_html_attr_val_literal().as_ref().unwrap().to_string()
                    };
                    return Err(RustHtmlError::from_string(format!(
                        "Unexpected punct '{}' while parsing HTML tag '{}' attributes \
                        (read {:?}, current key: {}, current val: {:?})", c, parse_ctx.tag_name_as_str(),
                        parse_ctx.get_html_attrs(), parse_ctx.get_html_attr_key(), current_val)));
                }
            }
        } else {
            match c {
                '>' => {
                    return self.on_html_tag_parsed(parse_ctx, output, ct);
                },
                '/' => {
                    if parse_ctx.has_tag_name() {
                        let expect_closing_punct = it.next().unwrap();
                        return match expect_closing_punct {
                            TokenTree::Punct(closing_punct) => {
                                if closing_punct.as_char() == '>' {
                                    parse_ctx.set_is_self_contained_tag(true);
                                    parse_ctx.add_tag_end_punct(punct);
                                    return self.on_html_tag_parsed(parse_ctx, output, ct);
                                } else {
                                    Err(RustHtmlError::from_string(format!("Unexpected character '{}' (expected '>', prev: '{}')", closing_punct, c)))
                                }
                            },
                            _ => {
                                Err(RustHtmlError::from_string(format!("convert_html_punct_to_rusthtmltoken Unexpected token after / (tag_name = {}): {:?}", parse_ctx.tag_name_as_str(), expect_closing_punct)))
                            },
                        };
                    } else {
                        parse_ctx.set_is_opening_tag(false);
                    }
                },
                '-' | '_' | '!' => {
                    parse_ctx.tag_name_push_punct(punct);
                },
                _ => {
                    return Err(RustHtmlError::from_string(format!("Unexpected character '{}'", c)));
                },
            }
        }
        Ok(false) // do not break
    }

    fn on_html_tag_parsed(
        self: &Self,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        output: &mut Vec<RustHtmlToken>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<bool, RustHtmlError> {
        if ct.is_cancelled() {
            return Err(RustHtmlError::from_string(format!("on_html_tag_parsed cancelled")));
        }

        if parse_ctx.is_opening_tag() {
            if parse_ctx.is_kvp_defined() {
                let tokens = self.on_kvp_defined(parse_ctx.clone(), ct)?;
                output.extend(tokens);
            }
        }

        for tag_helper in parse_ctx.get_main_context().get_tag_parsed_handler() {
            if tag_helper.matches(parse_ctx.tag_name_as_str().as_str(), parse_ctx.is_opening_tag()) {
                match tag_helper.on_tag_parsed(parse_ctx.clone(), output) {
                    Ok(should_break) => {
                        if should_break {
                            break;
                        }
                    },
                    Err(e) => {
                        return Err(RustHtmlError::from_string(format!("error while processing tag helper: {}", e)));
                    }
                }
                break;
            }
        }

        if parse_ctx.is_opening_tag() {
            output.push(
                if parse_ctx.is_void_tag() {
                    let punct = parse_ctx.get_tag_end_punct();
                    RustHtmlToken::HtmlTagCloseVoidPunct(punct.map(|punct| (punct.as_char(), punct.clone())))
                } else if parse_ctx.is_self_contained_tag() {
                    RustHtmlToken::HtmlTagCloseSelfContainedPunct
                } else {
                    RustHtmlToken::HtmlTagCloseStartChildrenPunct
                }
            );
            return Ok(true); // parse_ctx.is_void_tag() break if void tag, no children
        } else {
            return Ok(true); // break when closing
        }
    }
}
