use std::{cell::RefCell, borrow::Cow};
use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use core_lib::sys::call_tracker::CallstackTrackerScope;
use core_macro_lib::{callstack_tracker_scope_and_assert, nameof_member_fn};
use proc_macro2::{TokenTree, Literal, Punct};

use crate::view::rusthtml::html_tag_parse_context::HtmlTagParseContext;
use crate::view::rusthtml::ihtml_tag_parse_context::IHtmlTagParseContext;
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::peekable_rusthtmltoken::IPeekableRustHtmlToken;
use super::rusthmtl_expand_loop_result::RustHtmlExpandLoopResult;
use super::rusthtmlparser_all::{IRustHtmlParserAll, IRustHtmlParserAssignSharedParts};



pub trait IRustHtmlParserHtml: IRustHtmlParserAssignSharedParts {
    fn parse_html(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableRustHtmlToken>, cancellation_token: Rc<dyn ICancellationToken>) -> RustHtmlExpandLoopResult;

    fn parse_html_tag(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableRustHtmlToken>, cancellation_token: Rc<dyn ICancellationToken>) -> RustHtmlExpandLoopResult;
    fn parse_html_node(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableRustHtmlToken>, cancellation_token: Rc<dyn ICancellationToken>) -> RustHtmlExpandLoopResult;
    fn parse_html_attr_key(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableRustHtmlToken>, cancellation_token: Rc<dyn ICancellationToken>) -> RustHtmlExpandLoopResult;
    fn parse_html_attr_val(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableRustHtmlToken>, cancellation_token: Rc<dyn ICancellationToken>) -> RustHtmlExpandLoopResult;

    fn parse_html_child_nodes(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableRustHtmlToken>, cancellation_token: Rc<dyn ICancellationToken>) -> RustHtmlExpandLoopResult;

    fn on_kvp_defined(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, cancellation_token: Rc<dyn ICancellationToken>) -> RustHtmlExpandLoopResult;

    // iterate the iterator by one step (next) and expand a token tree to RustHtml tokens in the context of a HTML tag.
    // token_option: the token to expand.
    // parse_ctx: the parse context.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: whether we should break the outer loop or not, or an error.
    fn next_and_parse_html_tag(
        self: &Self,
        token: &RustHtmlToken,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        it: Rc<dyn IPeekableRustHtmlToken>,
        ct: Rc<dyn ICancellationToken>,
    ) -> Result<bool, RustHtmlError>;

    // called when a HTML tag is parsed.
    // punct: the punct token.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // returns: whether we should break the outer loop or not, or an error.
    fn on_html_tag_parsed(
        self: &Self,
        end_punct: &Option<RustHtmlToken>,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        cancellation_token: Rc<dyn ICancellationToken>
    ) -> RustHtmlExpandLoopResult;

    // called when a HTML node is parsed.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // returns: whether we should break the outer loop or not, or an error.
    fn on_html_node_parsed(
        self: &Self,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        ctx: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> RustHtmlExpandLoopResult;

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
    ) -> RustHtmlExpandLoopResult;

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
        it: Rc<dyn IPeekableRustHtmlToken>,
        cancellation_token: Rc<dyn ICancellationToken>
    ) -> RustHtmlExpandLoopResult;


    fn expand_html_entry_to_rusthtmltoken(self: &Self, c: char, punct: &Punct, it: Rc<dyn IPeekableRustHtmlToken>,  ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;
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
    fn parse_html(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> RustHtmlExpandLoopResult {
        callstack_tracker_scope_and_assert!(ctx, RustHtmlParserHtml::parse_html);        
        let mut output = vec![];
        let html_ctx = Rc::new(HtmlTagParseContext::new(Some(ctx.clone())));
        loop {
            if ct.is_cancelled() {
                let callstack = ctx.get_call_stack().to_string();
                return RustHtmlExpandLoopResult::Err(RustHtmlError::from_string(format!("parse_html cancelled at callstack: {}", callstack)));
            }

            let next_token = it.peek();
            if let Some(token) = next_token {
                let (tokens, break_loop) = match token {
                    RustHtmlToken::Group(delimiter, stream, group) => {
                        todo!("parse_html TokenTree::Group")
                    },
                    RustHtmlToken::Identifier(ident) => {
                        todo!("parse_html TokenTree::Ident")
                    },
                    RustHtmlToken::Literal(literal, s) => {
                        if let Some(literal) = literal {
                            self.convert_html_literal_to_rusthtmltoken(literal, html_ctx.clone(), ct.clone())?
                        } else if let Some(s) = s {
                            todo!("parse_html TokenTree::Literal (string)")
                        } else {
                            return RustHtmlExpandLoopResult::Err(RustHtmlError::from_string(format!("parse_html TokenTree::Literal (None)")));
                        }
                    },
                    RustHtmlToken::ReservedChar(c, punct) => {
                        self.convert_html_punct_to_rusthtmltoken(&punct, html_ctx.clone(), it.clone(), ct.clone())?
                    },
                    _ => {
                        return RustHtmlExpandLoopResult::Err(RustHtmlError::from_string(format!("Unexpected token in parse_html: {:?}", token)));
                    }
                };
                output.extend(tokens);
                if break_loop {
                    break;
                }
            }
        }
        Ok((output, false))
    }

    fn parse_html_tag(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> RustHtmlExpandLoopResult {
        // parse tag name
        // parse attributes (key value pairs)
        // parse closing puncts
        todo!("parse_html_tag")
    }

    // TODO: add tests
    fn parse_html_node(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> RustHtmlExpandLoopResult {
        let mut output = vec![];
        let mut parse_ctx = Rc::new(HtmlTagParseContext::new(Some(ctx))) as Rc<dyn IHtmlTagParseContext>;
        todo!("parse_html_node");
        Ok((output, false))
    }

    // TODO: add tests
    fn parse_html_attr_key(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> RustHtmlExpandLoopResult {
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
                    RustHtmlToken::Identifier(ident) => {
                        if last_was_ident {
                            break;
                        } else {
                            output.push(RustHtmlToken::ReservedIndent(ident.to_string(), ident.clone()));
                            it.next();
                            last_was_ident = true;
                        }
                    },
                    RustHtmlToken::ReservedChar(c, punct) => {
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
        Ok((output, false))
    }

    // TODO: add tests
    fn parse_html_attr_val(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> RustHtmlExpandLoopResult {
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
                    RustHtmlToken::Identifier(ident) => {
                        if last_was_ident {
                            break;
                        } else {
                            output.push(RustHtmlToken::ReservedIndent(ident.to_string(), ident.clone()));
                            it.next();
                            last_was_ident = true;
                        }
                    },
                    RustHtmlToken::ReservedChar(c, punct) => {
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
                    RustHtmlToken::Literal(literal, s) => {
                        output.push(RustHtmlToken::Literal(literal.clone(), None));
                        it.next();
                        break;
                    },
                    _ => {
                        return Err(RustHtmlError::from_string(format!("Unexpected token while parsing HTML tag attr val '{}' attribute val: {:?}", ctx.tag_name_as_str(), token)));
                    }
                }
            }
        }
        Ok((output, false))
    }

    fn parse_html_child_nodes(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> RustHtmlExpandLoopResult {
        todo!("parse_html_child_nodes")
    }

    fn convert_html_literal_to_rusthtmltoken(
        self: &Self, 
        literal: &Literal,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> RustHtmlExpandLoopResult {
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
                RustHtmlExpandLoopResult::Ok((vec![], false))
            }
        } else {
            Err(RustHtmlError(Cow::Owned(format!("Cannot use literal for tag name"))))
        }
    }

    fn on_kvp_defined(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> RustHtmlExpandLoopResult {
        let r = ctx.on_kvp_defined();
        match r {
            Ok(x) => {
                RustHtmlExpandLoopResult::Ok((x, false))
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
        it: Rc<dyn IPeekableRustHtmlToken>,
        ct: Rc<dyn ICancellationToken>
    ) -> RustHtmlExpandLoopResult {
        let main_ctx = parse_ctx.get_main_context();
        let _scope = CallstackTrackerScope::enter(main_ctx.get_call_stack(), nameof::name_of_type!(RustHtmlParserHtml), nameof_member_fn!(Self::convert_html_punct_to_rusthtmltoken));
        if ct.is_cancelled() {
            return Err(RustHtmlError::from_string(format!("convert_html_punct_to_rusthtmltoken cancelled")));
        }

        let mut output = vec![];
        let c = punct.as_char();
        if parse_ctx.is_parsing_attrs() {
            parse_ctx.get_main_context().add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::convert_html_punct_to_rusthtmltoken), c));
            match c {
                '>' => {
                    return self.on_html_tag_parsed(parse_ctx, ct);
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
                        RustHtmlToken::ReservedChar(c, closing_punct) => {
                            if *c == '>' {
                                parse_ctx.set_is_self_contained_tag(true);
                                parse_ctx.add_tag_end_punct(punct);
                                return self.on_html_tag_parsed(parse_ctx, ct);
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
                        return self.on_kvp_defined(parse_ctx, ct);
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
                    // output to console for debugging
                    print!("directive_token: {:?}", directive_token);

                    // fixme: this needs to be fixed, it is not checking directive logic
                    match directive_token {
                        RustHtmlToken::Identifier(ref ident) => {
                            let parser = self.parser.borrow().as_ref().unwrap().get_rust_parser();
                            match parser
                                .parse_rust_identifier_expression(
                                    true, 
                                    &directive_token,
                                    false,
                                    it.clone(), parse_ctx.get_main_context(), ct.clone()) {
                                Ok(rust_ident_exp) => {
                                    let parser = self.parser.borrow().as_ref().unwrap().get_rust_or_html_parser();
                                    let rushtml_ident_expr = rust_ident_exp.to_splice().to_vec();

                                    parse_ctx.set_html_attr_val_rust(rushtml_ident_expr);
                                },
                                Err(e) => {
                                    return Err(RustHtmlError::from_string(format!("error parsing rust ident exp: {}", e)));
                                }
                            }
                        },
                        RustHtmlToken::Literal(ref literal, ref s) => {
                            if let Some(literal) = literal {
                                parse_ctx.set_html_attr_val_literal(literal);                                
                            } else if let Some(s) = s {
                                parse_ctx.set_html_attr_val_literal(&Literal::string(s));
                            } else {
                                return Err(RustHtmlError::from_string(format!("convert_html_punct_to_rusthtmltoken Unexpected token after @: {:?}", directive_token)));
                            }
                        },
                        _ => {
                            return Err(RustHtmlError::from_string(format!("Unexpected directive token after '@' in html attribute val parse: {:?}", directive_token)));
                        }
                    }

                    // can't just call this, need to wrap in if
                    if parse_ctx.is_kvp_defined() {
                        let (tokens, break_loop) = self.on_kvp_defined(parse_ctx, ct)?;
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
                    return self.on_html_tag_parsed(parse_ctx, ct);
                },
                '/' => {
                    if parse_ctx.has_tag_name() {
                        let expect_closing_punct = it.next().unwrap();
                        return match expect_closing_punct {
                            RustHtmlToken::ReservedChar(c, closing_punct) => {
                                if closing_punct.as_char() == '>' {
                                    parse_ctx.set_is_self_contained_tag(true);
                                    parse_ctx.add_tag_end_punct(punct);
                                    return self.on_html_tag_parsed(parse_ctx, ct);
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
                '@' => {
                    // key name is a directive, must be a string
                    let directive_token_before = it.next().unwrap();
                    let directive_token = it.next().unwrap();
                    let callstack = parse_ctx.get_main_context().get_call_stack().to_string();
                    todo!("convert_html_punct_to_rusthtmltoken directive_token: {:?} {:?} {:?}", directive_token_before, directive_token, callstack);
                },
                _ => {
                    return Err(RustHtmlError::from_string(format!("Unexpected character '{}' (expected one of '>', '/', '-', '_', '!')", c)));
                },
            }
        }
        Ok((output, false)) // do not break
    }

    fn on_html_tag_parsed(
        self: &Self,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> RustHtmlExpandLoopResult {
        if ct.is_cancelled() {
            return Err(RustHtmlError::from_string(format!("on_html_tag_parsed cancelled")));
        }

        let mut output = vec![];

        if parse_ctx.is_opening_tag() {
            if parse_ctx.is_kvp_defined() {
                let (tokens, break_loop) = self.on_kvp_defined(parse_ctx.clone(), ct.clone())?;
                output.extend(tokens);
            }
        }

        for tag_helper in parse_ctx.get_main_context().get_tag_parsed_handler() {
            if tag_helper.matches(parse_ctx.tag_name_as_str().as_str(), parse_ctx.is_opening_tag()) {
                match tag_helper.on_tag_parsed(parse_ctx.clone(), ct.clone()) {
                    Ok((tokens, should_break)) => {
                        if should_break {
                            break;
                        } else {
                            output.extend(tokens);
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
            return Ok((output, true)); // parse_ctx.is_void_tag() break if void tag, no children
        } else {
            return Ok((output, true)); // break when closing
        }
    }

    // expand a Rust HTML entry to a RustHtml token.
    // punct: the punctuation to expand.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: nothing or an error.
    fn expand_html_entry_to_rusthtmltoken(self: &Self, c: char, punct: &Punct, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        callstack_tracker_scope_and_assert!(ctx, RustHtmlParserHtml::expand_html_entry_to_rusthtmltoken);

        let is_in_html_mode = ctx.get_is_in_html_mode();
        if is_in_html_mode || self.parser.borrow().unwrap().get_expander().is_start_of_current_expression_ctx(ctx.clone()) {
            // the below context is orphaned by not passing the parent html tag parse context.
            // this is usually fine. but we need to pass the main context to call add_operation_to_ooo_log
            // let ctx = Rc::new(HtmlTagParseContext::new(Some(ctx.clone())));
            let htmlctx = HtmlTagParseContext::new_and_attach(ctx.clone());
            let output_inner = Rc::new(RefCell::new(vec![]));
            ctx.push_output_buffer(output_inner.clone());
            // it.enable_log_next("expand_html_entry_to_rusthtmltoken");
            loop {
                if ct.is_cancelled() {
                    return Err(RustHtmlError::from_string(format!("Task Cancelled")));
                }

                let token_option = it.next();
                if let Some(token) = token_option {
                    if self.next_and_parse_html_tag(&token, htmlctx.clone(), it.clone(), ct.clone())? {
                        // println!("expand_html_entry_to_rusthtmltoken: breaking on {:?}", token);
                        break;
                    }
                } else {
                    break;
                }
            }
            // it.disable_log_next();

            let mut add_inner = true;
            if htmlctx.is_opening_tag() && !htmlctx.is_void_tag() && !htmlctx.is_self_contained_tag() {
                // parse inner elements / code until we find closing tag
                ctx.htmltag_scope_stack_push(htmlctx.tag_name_as_str());
                loop {
                    if ct.is_cancelled() {
                        return Err(RustHtmlError::from_string(format!("Task Cancelled")));
                    }

                    // might need to pass true to ctx.push_is_in_html_mode
                    if self.next_and_expand(it.clone(), ctx.clone(), ct.clone())? {
                        break;
                    }
                    match output_inner.borrow().last() {
                        Some(RustHtmlToken::HtmlTagEnd(tag_end, _tag_end_tokens)) => {
                            if tag_end == &htmlctx.tag_name_as_str() {
                                break;
                            }
                        },
                        _ => {
                        }
                    }
                }
                let last_scope_from_stack = ctx.htmltag_scope_stack_pop().expect("expected tag name on stack");
                if last_scope_from_stack != htmlctx.tag_name_as_str() {
                    return Err(RustHtmlError::from_string(format!("Mismatched HTML tags (found {} but expected {})", last_scope_from_stack, htmlctx.tag_name_as_str())));
                }

                if let Some(output_inner_last) = output_inner.borrow().last() {
                    if let RustHtmlToken::HtmlTagEnd(_tag_end, _tag_end_tokens) = output_inner_last {
                        add_inner = self.get_parser().get_html_parser().on_html_node_parsed(htmlctx, ctx.clone(), ct)?;
                    }
                }
            }

            ctx.pop_output_buffer();
            if add_inner {
                ctx.push_output_tokens(output_inner.borrow().as_slice())?;
            }
        } else {
            ctx.push_output_token(RustHtmlToken::ReservedChar(c, punct.clone()))?;
        }

        Ok(())
    }
}
