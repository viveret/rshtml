use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use core_lib::sys::call_tracker::CallstackTrackerScope;
use core_macro_lib::{callstack_tracker_scope_and_assert, nameof_member_fn};
use proc_macro2::{TokenTree, Punct, Delimiter, Group, Ident, TokenStream, Literal};

use crate::view::rusthtml::html_tag_parse_context::HtmlTagParseContext;
use crate::view::rusthtml::ihtml_tag_parse_context::IHtmlTagParseContext;
use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::{RustHtmlToken, RustHtmlIdentOrPunctOrGroup, RustHtmlIdentAndPunctAndGroupOrLiteral};
use crate::view::rusthtml::parser_parts::peekable_tokentree::IPeekableTokenTree;

use super::peekable_rusthtmltoken::{IPeekableRustHtmlToken, VecPeekableRustHtmlToken};
use super::rusthtmlparser_all::{IRustHtmlParserAssignSharedParts, IRustHtmlParserAll};

pub trait IRustHtmlParserExpander: IRustHtmlParserAssignSharedParts {
    fn preprocess_rusthtmltokens(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn postprocess_rusthtmltokens(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn preprocess_tokenstream(self: &Self, tokens: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError>;
    fn postprocess_tokenstream(self: &Self, tokens: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError>;
    
    // fn parse_rust_or_html(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    // fn expand_vec(&self, tokens: &Vec<RustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Vec<RustHtmlToken>;

    fn peek_path_str(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<String, RustHtmlError>;
    fn next_path_str(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<String, RustHtmlError>;

    fn get_opening_delim(self: &Self, delim: &Delimiter) -> &'static str;

    // get the delimiter as a string containing the closing delimiter.
    // delimiter: the delimiter to get the closing char for.
    // returns: the closing delimiter.
    fn get_closing_delim(self: &Self, delim: &Delimiter) -> &'static str;

    fn expand_tokentree_to_rusthtmltoken(self: &Self, token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>,  ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<bool, RustHtmlError>;
    fn expand_punct_to_rusthtmltoken(self: &Self, punct: &Punct, it: Rc<dyn IPeekableRustHtmlToken>,  ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<bool, RustHtmlError>;

    fn expand_rshtml(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;

    fn expand_rshtmltoken(&self, token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;
    fn expand_rshtml_punct(&self, punct: &Punct, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<bool, RustHtmlError>;

    // expand a Rust entry to a RustHtml token.
    // punct: the punctuation to expand.
    // is_in_html_mode: whether we are in HTML mode or not.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: nothing or an error.
    fn expand_rust_entry_to_rusthtmltoken(self: &Self, _c: char, _punct: &Punct, _it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>,  ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;

    // expand a RustHtml language directive in Rust to a RustHtml token.
    // token: the token to expand.
    // prefix_token_option: the prefix token, if any.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: whether we should break the outer loop or not, or an error.
    fn expand_rust_directive_to_rusthtmltoken(self: &Self, token: &RustHtmlToken, prefix_token_option: Option<RustHtmlToken>, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>,  ct: Rc<dyn ICancellationToken>) -> Result<bool, RustHtmlError>;

    // expand a RustHtml language directive group in Rust to a RustHtml token.
    // group: the group to expand.
    // prefix_token_option: the prefix token, if any.
    // output: the destination for the RustHtml tokens.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: nothing or an error.
    fn expand_rust_directive_group_to_rusthtmltoken(self: &Self, delimiter: &Delimiter, group: &Option<Group>, stream: Rc<dyn IPeekableRustHtmlToken>, _prefix_token_option: Option<RustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;

    // expand a RustHtml language directive identifier in Rust to a RustHtml token.
    // identifier: the identifier to expand.
    // prefix_token_option: the prefix token, if any.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: nothing or an error.
    fn expand_rust_directive_identifier_to_rusthtmltoken(self: &Self, identifier: &Ident, ident_token: &RustHtmlToken, prefix_token_option: Option<RustHtmlToken>, it: Rc<dyn IPeekableRustHtmlToken>,  ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;

    // expand an external token stream into RustHtml tokens.
    // path: the path to the external token stream.
    // output: the destination for the RustHtml tokens.
    // returns: nothing or an error.
    fn expand_external_tokenstream(self: &Self, path: &String, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;

    // expand an external token stream into RustHtml tokens.
    // path: the path to the external token stream.
    // returns: nothing or an error.
    fn expand_external_rshtml_string(self: &Self, input_str: &String, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;

    // returns if the current output is the start of a new expression or not.
    // returns: if the current output is the start of a new expression or not.
    fn is_start_of_current_expression(self: &Self, output: &[RustHtmlToken]) -> bool;

    // returns if the current output is the start of a new expression or not.
    // ctx: the context to use.
    // returns: if the current output is the start of a new expression or not.
    fn is_start_of_current_expression_ctx(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>) -> bool;

    // parse a Rust string literal with quotes.
    // identifier: the identifier to expand.
    // it: the iterator to use.
    // returns: the string or an error.
    fn parse_string_with_quotes(self: &Self, peek_or_next: bool, identifier: Ident, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<String, RustHtmlError>;
    
    // parse Rust identifier expression and expand it to RustHtml tokens.
    // identifier: the identifier to expand.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn parse_identifier_expression(self: &Self, add_first_ident: bool, _identifier: &Ident, identifier_token: &RustHtmlToken, last_token_was_ident: bool, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;

    // get the next token and parse it as a literal or identifier expression that can be expanded to RustHtml tokens.
    // identifier: the identifier to expand.
    // it: the iterator to use.
    // returns: the expanded tokens or an error.
    fn expand_string_or_ident(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlIdentAndPunctAndGroupOrLiteral, RustHtmlError>;

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

    // expand RustHtml tokens to a RustHtml identifier or punct or group.
    // tokens: the tokens to expand.
    // returns: the expanded tokens or an error.
    fn expand_rusthtmltokens_to_ident_or_punct_or_group(
        self: &Self, tokens: Vec<RustHtmlToken>,
        ctx: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<Vec<RustHtmlIdentOrPunctOrGroup>, RustHtmlError>;

    // expand a Rust identifier to a RustHtml token in the context of a HTML tag.
    // ident: the identifier to expand.
    // parse_ctx: the parse context.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn expand_html_ident_to_rusthtmltoken(
        self: &Self, 
        ident: &Ident,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        it: Rc<dyn IPeekableRustHtmlToken>,
        ct: Rc<dyn ICancellationToken>,
    ) -> Result<(), RustHtmlError>;
    
    // // expand a Rust literal to a RustHtml token in the context of a HTML tag.
    // // literal: the literal to expand.
    // // parse_ctx: the parse context.
    // // returns: nothing or an error.
    // fn expand_html_literal_to_rusthtmltoken(
    //     self: &Self, 
    //     literal: &Literal,
    //     parse_ctx: Rc<dyn IHtmlTagParseContext>,
    //     ct: Rc<dyn ICancellationToken>
    // ) -> Result<(), RustHtmlError>;

    // // expand a Rust punct to a RustHtml token in the context of a HTML tag.
    // // punct: the punct to expand.
    // // parse_ctx: the parse context.
    // // it: the iterator to use.
    // // is_raw_tokenstream: whether the token stream is raw or not.
    // // returns: whether we should break the outer loop or not, or an error.
    // fn expand_html_punct_to_rusthtmltoken(
    //     self: &Self, 
    //     punct: &Punct,
    //     parse_ctx: Rc<dyn IHtmlTagParseContext>,
    //     it: Rc<dyn IPeekableRustHtmlToken>,
    //     ct: Rc<dyn ICancellationToken>
    // ) -> Result<bool, RustHtmlError>;

    // called when a HTML tag attribute key/value pair is defined.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // returns: nothing.
    fn on_kvp_defined(
        self: &Self,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<(), RustHtmlError>;

    // parse a Rust type identifier from a stream of tokens.
    // it: the iterator to use.
    // returns: the type identifier or an error.
    fn parse_type_identifier(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Rc<dyn IPeekableRustHtmlToken>, RustHtmlError>;
    
    // parse a Rust type identifier from a stream of tokens.
    fn on_html_tag_parsed(
        self: &Self,
        end_punct: Option<&Punct>,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        ctx: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<bool, RustHtmlError>;

    // called when a HTML node is parsed.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // returns: whether we should break the outer loop or not, or an error.
    fn on_html_node_parsed(
        self: &Self,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        ctx: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<bool, RustHtmlError>;

    // expand a Rust group, identifier, or literal to RustHtml tokens.
    // token: the token to expand.
    // output: the destination for the RustHtml tokens.
    // returns: nothing or an error.
    fn expand_copy(self: &Self, token: &RustHtmlToken, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;

    // expand a RustHtml identifier or punct or group or literal to Rust tokens.
    // tag: the tag to expand.
    // returns: the expanded tokens or an error.
    fn expand_ident_and_punct_and_group_or_literal_to_tokenstream(self: &Self, tag: &RustHtmlIdentAndPunctAndGroupOrLiteral) -> Result<Rc<dyn IPeekableRustHtmlToken>, RustHtmlError>;

    fn peek_reserved_chars_in_str(self: &Self, arg: &'static str, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<bool, RustHtmlError>;

    fn peek_reserved_char(self: &Self, expected_char: char, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<bool, RustHtmlError>;

    fn expand_tokenstream_to_rusthtmltokens(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>,  ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn loop_next_and_expand(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>,  ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;
    fn next_and_expand(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>,  ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<bool, RustHtmlError>;
    fn expand_html_entry_to_rusthtmltoken(self: &Self, c: char, punct: &Punct, it: Rc<dyn IPeekableRustHtmlToken>,  ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;

    // expand a Rust group to a RustHtml token.
    // group: the group to expand.
    // is_in_html_mode: whether we are in HTML mode or not.
    // output: the destination for the RustHtml tokens.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: nothing or an error.
    fn expand_group_to_rusthtmltoken(self: &Self, delimiter: &Delimiter, group: &Option<Group>, group_stream: Rc<dyn IPeekableRustHtmlToken>, expect_return_html: bool, ctx: Rc<dyn IRustHtmlParserContext>,  ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;
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

    fn get_parser(&self) -> Rc<dyn IRustHtmlParserAll> {
        self.parser.borrow().as_ref().expect("self.parser was None").clone()
    }

    fn expand_group(&self, delimiter: &Delimiter, it: Rc<dyn IPeekableRustHtmlToken>, group: &Option<Group>, is_in_html_mode: bool, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        let mut inner_tokens = vec![];
        let inner_tokens_sharable = Rc::new(RefCell::new(inner_tokens));
        ctx.push_output_buffer(inner_tokens_sharable);
        self.loop_next_and_expand(it, ctx, ct)?;
        ctx.pop_output_buffer();
        if inner_tokens.len() > 0 {
            ctx.push_output_token(RustHtmlToken::Group(delimiter.clone(), Rc::new(VecPeekableRustHtmlToken::new(inner_tokens)), None));
        }
        Ok(())
    }
}

impl IRustHtmlParserAssignSharedParts for RustHtmlParserExpander {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        *self.parser.borrow_mut() = Some(parser);
    }
}

impl IRustHtmlParserExpander for RustHtmlParserExpander {
    // fn expand_rust_to_rusthtml(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableTokenTree>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
    //     callstack_tracker_scope_and_assert!(ctx, RustHtmlParserExpander::expand_rust_to_rusthtml);

    //     let mut output = vec![];
    //     loop {
    //         if ct.is_cancelled() {
    //             let callstack = ctx.get_call_stack().to_string();
    //             return Err(RustHtmlError::from_string(format!("parse_rust cancelled at {}", callstack)));
    //         }

    //         let next_token = it.peek();
    //         if let Some(token) = next_token {
    //             match token {
    //                 TokenTree::Ident(ident) => {
    //                     // consume the token from the stream
    //                     it.next();
    //                     let expander = self.get_parser().get_expander();
    //                     match expander.expand_ident(&ident) {
    //                         Ok(ident) => {
    //                             output.push(ident);
    //                         },
    //                         Err(RustHtmlError(err)) => {
    //                             return Err(RustHtmlError::from_string(err.into_owned()));
    //                         }
    //                     }
    //                 },
    //                 TokenTree::Punct(punct) => {
    //                     // consume the token from the stream
    //                     it.next(); // this should be conditional in case we peek a terminal token
    //                     // for example, <div>hello</div> should not consume the </div> token
    //                     // let expander = self.get_parser().get_expander();
    //                     // match expander.expand_punct(&punct) {
    //                     //     Ok(punct) => {
    //                     //         output.push(punct);
    //                     //     },
    //                     //     Err(RustHtmlError(err)) => {
    //                     //         return Err(RustHtmlError::from_string(err.into_owned()));
    //                     //     }
    //                     // }
    //                     self.expand_rshtml_punct(punct, it.clone(), ctx.clone(), ct.clone())?;
    //                 },
    //                 TokenTree::Literal(literal) => {
    //                     // consume the token from the stream
    //                     it.next();
    //                     let expander = self.get_parser().get_expander();
    //                     match expander.expand_literal(&literal, ct.clone()) {
    //                         Ok(literal) => {
    //                             output.push(literal);
    //                         },
    //                         Err(RustHtmlError(err)) => {
    //                             return Err(RustHtmlError::from_string(err.into_owned()));
    //                         }
    //                     }
    //                 },
    //                 TokenTree::Group(group) => {
    //                     // consume the token from the stream
    //                     it.next();
    //                     let expander = self.get_parser().get_expander();
    //                     match expander.expand_group(&group, false, ctx.clone(), ct.clone()) {
    //                         Ok(group) => {
    //                             output.push(group);
    //                         },
    //                         Err(RustHtmlError(err)) => {
    //                             return Err(RustHtmlError::from_string(err.into_owned()));
    //                         }
    //                     }
    //                 },
    //             }
    //         } else {
    //             break;
    //         }
    //     }
    //     Ok(output)
    // }

    fn expand_rshtml(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        callstack_tracker_scope_and_assert!(ctx, RustHtmlParserExpander::expand_rusthtml);

        loop {
            let next_token = it.peek();
            match next_token {
                Some(token) => {
                    self.expand_rshtmltoken(token, it.clone(), ctx.clone(), ct.clone())?;
                },
                None => {
                    break;
                }
            }
        }

        Ok(())
    }

    fn expand_rshtmltoken(&self, token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        callstack_tracker_scope_and_assert!(ctx, RustHtmlParserExpander::expand_rshtmltoken);
        
        match token {
            RustHtmlToken::ReservedChar(c, p) => {
                match self.expand_rshtml_punct(p, it.clone(), ctx.clone(), ct.clone()) {
                    Ok(b) => {
                        return Ok(());
                    },
                    Err(RustHtmlError(err)) => {
                        return Err(RustHtmlError::from_string(err.into_owned()));
                    }
                }
            },
            _ => {
                Err(RustHtmlError::from_string(format!("Unexpected token: {:?}", token)))
            }
        }
    }

    fn expand_rshtml_punct(&self, punct: &Punct, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<bool, RustHtmlError> {
        let c = punct.as_char();
        let is_in_html_mode = ctx.get_is_in_html_mode();
        match c {
            '@' => {
                self.expand_rust_entry_to_rusthtmltoken(c, punct, it, ctx.clone(), ct)?
            },
            '<' => {
                ctx.push_is_in_html_mode(true);
                self.expand_html_entry_to_rusthtmltoken(c, punct, it, ctx.clone(), ct)?
            },
            '}' if !is_in_html_mode => {
                return Ok(true); // do not continue
            },
            '>' if !is_in_html_mode => {
                ctx.push_output_token(RustHtmlToken::ReservedChar(c, punct.clone()));
            },
            '|' if !is_in_html_mode => {
                ctx.push_output_token(RustHtmlToken::ReservedChar(c, punct.clone()));

                // peek ahead to see if this is a || -> or something else
                if self.peek_reserved_chars_in_str("|->", ctx.clone(), it.clone())? {
                    // peek for HtmlString identifier that signals the function will return HtmlString
                    if let Some(next_token) = it.peek() {
                        match next_token {
                            RustHtmlToken::Identifier(next_ident) => {
                                if next_ident.to_string() == "HtmlString" {
                                    // this is a function that returns HtmlString
                                    it.next();
                                    ctx.push_output_token(RustHtmlToken::Identifier(next_ident.clone()));

                                    // parse the rest of the function, which should be in a {} group
                                    if let Some(group_token) = it.next() {
                                        match group_token {
                                            RustHtmlToken::Group(d, stream, group) if *d == Delimiter::Brace => {
                                                self.expand_group_to_rusthtmltoken(d, group, stream.clone(), true, ctx.clone(), ct)?;
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
                    ctx.push_output_token(RustHtmlToken::HtmlTextNode(punct.as_char().to_string(), punct.span().clone()));
                } else {
                    ctx.push_output_token(RustHtmlToken::ReservedChar(c, punct.clone()));
                }
            },
        }
        Ok(false) // continue
    }

    fn expand_rust_entry_to_rusthtmltoken(self: &Self, _c: char, _punct: &Punct, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>,  ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        if let Some(directive_token) = it.next() {
            self.expand_rust_directive_to_rusthtmltoken(directive_token, None, it.clone(), ctx, ct)?;
        }
        Ok(())
    }
    
    fn expand_rust_directive_to_rusthtmltoken(self: &Self, token: &RustHtmlToken, prefix_token_option: Option<RustHtmlToken>, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>,  ct: Rc<dyn ICancellationToken>) -> Result<bool, RustHtmlError>  {
        if ct.is_cancelled() {
            return Err(RustHtmlError::from_string(format!("Task Cancelled")));
        }

        match token {
            RustHtmlToken::Identifier(ref ident) => {
                self.expand_rust_directive_identifier_to_rusthtmltoken(ident, &token, prefix_token_option, it, ctx, ct)?;
            },
            RustHtmlToken::Group(d, stream, group) => {
                self.expand_rust_directive_group_to_rusthtmltoken(d, group, stream.clone(), prefix_token_option, ctx, ct)?;
            },
            RustHtmlToken::Literal(literal, s) => {
                ctx.push_output_token(RustHtmlToken::AppendToHtml(vec![RustHtmlToken::Literal(literal.clone(), s.clone())]));
                // self.expand_rusthtml_literal_to_rusthtmltoken(group, it);
            },
            RustHtmlToken::ReservedChar(c, punct) => {
                let c = punct.as_char();
                match c {
                    '@' => {
                        // escape '@'
                        ctx.push_output_token(RustHtmlToken::AppendToHtml(vec![RustHtmlToken::ReservedChar(c, punct.clone())]));
                    },
                    '&' => {
                        let prefix_token = RustHtmlToken::ReservedChar(c, punct.clone());
                        
                        let next_token = it.next();
                        if let Some(token) = next_token {
                            return self.expand_rust_directive_to_rusthtmltoken(token, Some(prefix_token), it.clone(), ctx, ct);
                        }
                    },
                    _ => {
                        return Err(RustHtmlError::from_string(format!("unexpected directive char: {}", c)));
                    }
                }
            },
            _ => {
                return Err(RustHtmlError::from_string(format!("unexpected directive token: {:?}", token)));
            }
        }
        Ok(true)
    }
    

    fn expand_rust_directive_group_to_rusthtmltoken(self: &Self, delimiter: &Delimiter, group: &Option<Group>, stream: Rc<dyn IPeekableRustHtmlToken>, _prefix_token_option: Option<RustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        let mut inner_tokens = vec![];
        // let it = Rc::new(StreamPeekableRustHtmlToken::new(group.stream()));
        let it = stream;
        ctx.push_is_in_html_mode(false);
        self.loop_next_and_expand(it, ctx, ct)?;
        if inner_tokens.len() > 0 {
            match delimiter {
                Delimiter::Brace => {
                    ctx.push_output_tokens(&inner_tokens);
                },
                Delimiter::Parenthesis => {
                    ctx.push_output_token(RustHtmlToken::AppendToHtml(inner_tokens));
                },
                _ => {
                    return Err(RustHtmlError::from_string(format!("unexpected delimiter: {:?}", delimiter)));
                },
            }
        }
        Ok(())
    }


    fn expand_rust_directive_identifier_to_rusthtmltoken(self: &Self, identifier: &Ident, ident_token: &RustHtmlToken, prefix_token_option: Option<RustHtmlToken>, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        if let Some(directive) = ctx.try_get_directive(identifier.to_string()) {
            let r = directive.execute(ctx.clone(), &identifier, ident_token, self.parser.borrow().as_ref().unwrap().clone(), it, ct);
            match r {
                Ok(r) => {
                    match r {
                        RustHtmlDirectiveResult::OkContinue => { },
                        RustHtmlDirectiveResult::OkBreak => { },
                        RustHtmlDirectiveResult::OkBreakAppendHtml => ctx.push_output_token(RustHtmlToken::AppendToHtml(vec![])),
                    }
                },
                Err(RustHtmlError(e)) => {
                    return Err(RustHtmlError::from_string(format!("error executing directive: {}", e)));
                }
            }
        } else {
            let mut inner_tokens = vec![];
            let inner_tokens_sharable = Rc::new(RefCell::new(inner_tokens));
            ctx.push_output_buffer(inner_tokens_sharable);
            if let Some(prefix_token) = prefix_token_option {
                inner_tokens.push(prefix_token);
            }
            self.parse_identifier_expression(true, identifier, ident_token, true, it, ctx, ct)?;
            ctx.pop_output_buffer();
            ctx.push_output_token(RustHtmlToken::AppendToHtml(inner_tokens));
        }
        Ok(())
    }


    // parse a token stream to RustHtml tokens.
    // is_in_html_mode: whether we are in HTML mode or not.
    // it: the token stream to parse.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: the RustHtml tokens.
    fn expand_tokenstream_to_rusthtmltokens(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut rusthtml_tokens = Vec::new();
        let sharable_buffer = Rc::new(RefCell::new(rusthtml_tokens));
        ctx.push_output_buffer(sharable_buffer);
        self.loop_next_and_expand(it, ctx, ct)?;
        ctx.pop_output_buffer();
        Ok(rusthtml_tokens)
    }

    // loop through the token stream and expand it to RustHtml tokens.
    // is_in_html_mode: whether we are in HTML mode or not.
    // output: the destination for the RustHtml tokens.
    // it: the token stream to parse.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: nothing or error.
    fn loop_next_and_expand(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>,  ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        loop {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_string(format!("Task Cancelled")));
            }
            if self.next_and_expand(it.clone(), ctx.clone(), ct.clone())? {
                break;
            }
        }
        Ok(())
    }

    // iterate the iterator by one step (next) and expand a token tree to RustHtml tokens.
    // is_in_html_mode: whether we are in HTML mode or not.
    // output: the destination for the RustHtml tokens.
    // it: the token stream to parse.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: whether we should break the outer loop or not, or an error.
    fn next_and_expand(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>,  ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<bool, RustHtmlError> {
        callstack_tracker_scope_and_assert!(ctx, RustHtmlParserExpander::next_and_expand);
        if ct.is_cancelled() {
            return Err(RustHtmlError::from_string(format!("Task Cancelled")));
        }

        let token_option = it.next();

        if let Some(token) = token_option {
            if self.expand_tokentree_to_rusthtmltoken(token, it.clone(), ctx.clone(), ct)? {
                return Ok(true); // break outer loop
            }
        } else {
            return Ok(true); // break outer loop
        }

        Ok(false)
    }


    // expands a tokentree to a RustHtml token.
    // token: the token to expand.
    // is_in_html_mode: whether we are in HTML mode or not.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: whether we should break the outer loop or not, or an error.
    fn expand_tokentree_to_rusthtmltoken(self: &Self, token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>,  ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<bool, RustHtmlError> {
        callstack_tracker_scope_and_assert!(ctx, RustHtmlParserExpander::expand_tokentree_to_rusthtmltoken);
        if ct.is_cancelled() {
            return Err(RustHtmlError::from_string(format!("Task Cancelled")));
        }

        let is_in_html_mode = ctx.get_is_in_html_mode();
        match token {
            RustHtmlToken::Identifier(ident) => {
                if is_in_html_mode {
                    ctx.push_output_token(RustHtmlToken::HtmlTextNode(ident.to_string(), ident.span().clone()));
                } else {
                    ctx.push_output_token(RustHtmlToken::Identifier(ident.clone()));
                }
            },
            RustHtmlToken::Literal(literal, s) => {
                if is_in_html_mode {
                    ctx.push_output_token(RustHtmlToken::HtmlTextNode(literal.clone().unwrap().to_string(), literal.clone().unwrap().span().clone()));
                } else {
                    ctx.push_output_token(RustHtmlToken::Literal(literal.clone(), None));
                }
            },
            RustHtmlToken::ReservedChar(c, punct) => {
                if self.expand_punct_to_rusthtmltoken(punct, it, ctx.clone(), ct)? {
                    return Ok(true);
                }
            },
            RustHtmlToken::Group(d, stream, group) => {
                self.expand_group_to_rusthtmltoken(d, group, stream.clone(), false, ctx.clone(), ct)?;
            },
            _ => {
                panic!("expand_tokentree_to_rusthtmltoken: unhandled token: {:?}", token);
            }
        }
        Ok(false) // continue
    }
    
    // expand a Rust punctuation to a RustHtml token.
    // punct: the punctuation to expand.
    // is_in_html_mode: whether we are in HTML mode or not.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: whether we should break the outer loop or not, or an error.
    fn expand_punct_to_rusthtmltoken(self: &Self, punct: &Punct, it: Rc<dyn IPeekableRustHtmlToken>,  ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<bool, RustHtmlError> {
        callstack_tracker_scope_and_assert!(ctx, RustHtmlParserExpander::expand_punct_to_rusthtmltoken);
        if ct.is_cancelled() {
            return Err(RustHtmlError::from_string(format!("Task Cancelled")));
        }

        // todo: expand and do not expand

        Ok(false)
    }

    // fn parse_rust_or_html(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
    //     callstack_tracker_scope_and_assert!(ctx, RustHtmlParserRustOrHtml::parse_rust_or_html);
        
    //     if ct.is_cancelled() {
    //         return Err(RustHtmlError::from_str("RustHtmlParserRustOrHtml: cancellation_token is cancelled"));
    //     }
        
    //     if let Some(shared_parser) = self.shared_parser.borrow().as_ref() {

    //     } else {
    //         Err(RustHtmlError::from_str("RustHtmlParserRustOrHtml: shared_parser is None"))
    //     }
    // }

    fn peek_path_str(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>,identifier: &Ident, ident_token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<String, RustHtmlError> {
        callstack_tracker_scope_and_assert!(ctx, RustHtmlParserExpander::peek_path_str);
        let mut path = std::path::PathBuf::new();
        let cwd = std::env::current_dir().expect("couldn't get current working directory");
        path.push(cwd);
        
        // do match instead
        match self.parser.borrow().as_ref()
                    .expect("shared_parser was None")
                    .get_rust_parser()
                    .parse_string_with_quotes(true, identifier, it) {
            Ok(relative_path) => {
                path.push(relative_path.clone());
            },
            Err(RustHtmlError(err)) => {
                return Err(RustHtmlError::from_string(err.into_owned()));
            }
        }

        Ok(path.to_str().expect("couldn't awd").to_string())
    }

    fn next_path_str(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &RustHtmlToken, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<String, RustHtmlError> {
        callstack_tracker_scope_and_assert!(ctx, RustHtmlParserExpander::next_path_str);
        let mut path = std::path::PathBuf::new();
        let cwd = std::env::current_dir().expect("couldn't get current working directory");
        path.push(cwd);
        match self.parser.borrow().as_ref()
                    .expect("shared_parser was None")
                    .get_rust_parser()
                    .parse_string_with_quotes(true, identifier, it) {
            Ok(relative_path) => {
                path.push(relative_path.clone());
            },
            Err(RustHtmlError(err)) => {
                return Err(RustHtmlError::from_string(err.into_owned()));
            }
        }

        Ok(path.to_str().expect("couldn't awd").to_string())
    }

    fn peek_reserved_chars_in_str(self: &Self, arg: &'static str, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<bool, RustHtmlError> {
        for c in arg.chars() {
            if !self.peek_reserved_char(c, ctx.clone(), it.clone())? {
                return Ok(false);
            }
        }
    
        Ok(true)
    }

    fn preprocess_rusthtmltokens(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!()
    }

    fn postprocess_rusthtmltokens(self: &Self, rusthtml_tokens: &Vec<RustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!()
    }

    fn preprocess_tokenstream(self: &Self, tokens: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError> {
        todo!()
    }

    fn postprocess_tokenstream(self: &Self, tokens: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError> {
        todo!()
    }

    fn get_opening_delim(self: &Self, delimiter: &Delimiter) -> &'static str {
        match delimiter {
            Delimiter::Brace => "{",
            Delimiter::Bracket => "[",
            Delimiter::Parenthesis => "(",
            Delimiter::None => "",
        }
    }

    fn get_closing_delim(self: &Self, delimiter: &Delimiter) -> &'static str {
        match delimiter {
            Delimiter::Brace => "}",
            Delimiter::Bracket => "]",
            Delimiter::Parenthesis => ")",
            Delimiter::None => "",
        }
    }

    // expand a Rust HTML entry to a RustHtml token.
    // punct: the punctuation to expand.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: nothing or an error.
    fn expand_html_entry_to_rusthtmltoken(self: &Self, c: char, punct: &Punct, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        let is_in_html_mode = ctx.get_is_in_html_mode();
        if is_in_html_mode || self.is_start_of_current_expression_ctx(ctx.clone()) {
            // the below context is orphaned by not passing the parent html tag parse context.
            // this is usually fine. but we need to pass the main context to call add_operation_to_ooo_log
            // let ctx = Rc::new(HtmlTagParseContext::new(Some(ctx.clone())));
            let htmlctx = HtmlTagParseContext::new_and_attach(ctx.clone());
            let mut output_inner = vec![];
            let output_inner_sharable = Rc::new(RefCell::new(output_inner));
            ctx.push_output_buffer(output_inner_sharable);
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
                    match output_inner.last() {
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

                if let Some(output_inner_last) = output_inner.last() {
                    if let RustHtmlToken::HtmlTagEnd(_tag_end, _tag_end_tokens) = output_inner_last {
                        add_inner = self.on_html_node_parsed(htmlctx, ctx, ct)?;
                    }
                }
            }

            if add_inner {
                ctx.pop_output_buffer();
                ctx.push_output_tokens(&output_inner);
            }
        } else {
            ctx.push_output_token(RustHtmlToken::ReservedChar(c, punct.clone()));
        }

        Ok(())
    }

    // expand a Rust group to a RustHtml token.
    // group: the group to expand.
    // is_in_html_mode: whether we are in HTML mode or not.
    // output: the destination for the RustHtml tokens.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: nothing or an error.
    fn expand_group_to_rusthtmltoken(self: &Self, delimiter: &Delimiter, group: &Option<Group>, group_stream: Rc<dyn IPeekableRustHtmlToken>, expect_return_html: bool, ctx: Rc<dyn IRustHtmlParserContext>,  ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        // let it = Rc::new(StreamPeekableRustHtmlToken::new(group.stream()));
        let it = group_stream;
        if ctx.get_is_in_html_mode() {
            ctx.add_operation_to_ooo_log(format!("expand_group_to_rusthtmltoken: {:?}", group));
            let c_start = self.get_opening_delim(delimiter);
            let c_end = self.get_closing_delim(delimiter);

            ctx.push_output_token(RustHtmlToken::HtmlTextNode(c_start.to_string(), group.clone().unwrap().span()));
            // might need to pass true to ctx.push_is_in_html_mode
            self.loop_next_and_expand(it, ctx, ct)?;
            ctx.push_output_token(RustHtmlToken::HtmlTextNode(c_end.to_string(), group.clone().unwrap().span()));

            Ok(())
        } else {
            match self.expand_group(delimiter, group_stream, group, expect_return_html, ctx, ct) {
                Ok(_) => {
                    Ok(())
                },
                Err(RustHtmlError(e)) => {
                    Err(RustHtmlError::from_string(format!("error expanding group: {}", e)))
                }
            }
        }
    }


    // fn peek_path_str(self: &Self, identifier: &Ident, identifier_token: &TokenTree,  it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<String, RustHtmlError> {
    //     match self.new_parser.get_rust_or_html_parser().peek_path_str(self.context.clone(), identifier, identifier_token, it) {
    //         Ok(s) => {
    //             Ok(s)
    //         },
    //         Err(e) => {
    //             Err(RustHtmlError::from_string(format!("error parsing string: {}", e)))
    //         }
    //     }
    // }

    // expand an external token stream into RustHtml tokens.
    // path: the path to the external token stream.
    // output: the destination for the RustHtml tokens.
    // returns: nothing or an error.
    fn expand_external_tokenstream(self: &Self, path: &String, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        callstack_tracker_scope_and_assert!(ctx, RustHtmlParserExpander::expand_external_tokenstream);

        if ct.is_cancelled() {
            return Err(RustHtmlError::from_string(format!("Task Cancelled")));
        }

        match std::fs::read_to_string(path) {
            Ok(input_str) => {
                if ct.is_cancelled() {
                    return Err(RustHtmlError::from_string(format!("Task Cancelled")));
                }
                
                self.expand_external_rshtml_string(&input_str, ctx.clone(), ct)
            },
            Err(_e) => {
                let parent_path = std::path::Path::new(path).parent().expect("expected parent path");
                match std::fs::read_to_string(parent_path) {
                    Ok(input_str) => {
                        if ct.is_cancelled() {
                            return Err(RustHtmlError::from_string(format!("Task Cancelled")));
                        }

                        self.expand_external_rshtml_string(&input_str, ctx.clone(), ct)
                    },
                    Err(e) => {
                        Err(RustHtmlError::from_string(format!("Cannot read {}: {}", path, e)))
                    },
                }
            },
        }
    }

    // expand an external token stream into RustHtml tokens.
    // path: the path to the external token stream.
    // output: the destination for the RustHtml tokens.
    // returns: nothing or an error.
    fn expand_external_rshtml_string(self: &Self, input_str: &String, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        callstack_tracker_scope_and_assert!(ctx, RustHtmlParserExpander::expand_external_rshtml_string);

        let input_result = proc_macro2::TokenStream::from_str(input_str.as_str());
        
        match input_result {
            Ok(input) => {
                let get_converter = &self.get_parser().get_converter();
                let input_converted = get_converter.convert_stream(input, ctx.clone(), ct.clone());
                match input_converted {
                    Ok(input_converted) => {
                        let it = Rc::new(VecPeekableRustHtmlToken::new(input_converted));
                        // might need to pass true to ctx.push_is_in_html_mode
                        let rusthtml_tokens = self.expand_tokenstream_to_rusthtmltokens(it, ctx.clone(), ct)?;
                        ctx.push_output_tokens(&rusthtml_tokens)?;
                        Ok(())
                    },
                    Err(RustHtmlError(e)) => {
                        Err(RustHtmlError::from_string(format!("error converting tokenstream to rusthtmltokens: {}", e)))
                    }
                }
            },
            Err(e) => {
                Err(RustHtmlError::from_string(format!("{}", e)))
            },
        }
    }

    // returns if the current output is the start of a new expression or not.
    // output: the destination for the RustHtml tokens.
    // returns: if the current output is the start of a new expression or not.
    fn is_start_of_current_expression(self: &Self, output: &[RustHtmlToken]) -> bool {
        if output.len() == 0 {
            true
        } else {
            let last = output.last().expect("expected last token");
            match last {
                RustHtmlToken::ReservedChar(c, _punct) => *c == ';',
                RustHtmlToken::Group(..) => true,
                _ => false,
            }
        }
    }

    // returns if the current output is the start of a new expression or not.
    // ctx: the context to use.
    // returns: if the current output is the start of a new expression or not.
    fn is_start_of_current_expression_ctx(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>) -> bool {
        let output = ctx.get_output_buffer();
        if let Some(output) = output {
            let output = output.borrow().as_slice();
            self.is_start_of_current_expression(output)
        } else {
            true
        }
    }

    // parse a Rust string literal with quotes.
    // identifier: the identifier to expand.
    // it: the iterator to use.
    // returns: the string or an error.
    fn parse_string_with_quotes(self: &Self, peek_or_next: bool, identifier: Ident, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<String, RustHtmlError> {
        match self.parser.borrow().as_ref().unwrap().get_rust_parser().parse_string_with_quotes(peek_or_next, &identifier, it) {
            Ok(s) => {
                Ok(s)
            },
            Err(e) => {
                Err(RustHtmlError::from_string(format!("error parsing string: {}", e)))
            }
        }
    }

    // parse Rust identifier expression and expand it to RustHtml tokens.
    // identifier: the identifier to expand.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn parse_identifier_expression(self: &Self, add_first_ident: bool, _identifier: &Ident, identifier_token: &RustHtmlToken, last_token_was_ident: bool, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        // match self.new_parser.get_rust_parser(); (add_first_ident, identifier_token, last_token_was_ident, it, is_raw_tokenstream) 
        match self.parser.borrow().as_ref().unwrap().get_rust_parser().parse_rust_identifier_expression(add_first_ident, identifier_token, last_token_was_ident, it.clone(), ctx.clone(), ct.clone()) {
            Ok(tokens) => {
                // for token in tokens.iter() {
                //     match token {
                //         RustHtmlToken::Literal(literal) => {
                //             output.push(RustHtmlToken::Literal(Some(literal.clone()), None));
                //         },
                //         RustHtmlToken::Ident(ident) => {
                //             output.push(RustHtmlToken::Identifier(ident.clone()));
                //         },
                //         RustHtmlToken::Group(group) => {
                //             let delimiter = group.delimiter();
                //             let mut inner_tokens = vec![];
                //             self.loop_next_and_expand(false, &mut inner_tokens, Rc::new(StreamPeekableRustHtmlToken::new(group.stream())), is_raw_tokenstream)?;
                //             output.push(RustHtmlToken::GroupParsed(delimiter, inner_tokens));
                //         },
                //         RustHtmlToken::Punct(punct) => {
                //             output.push(RustHtmlToken::ReservedChar(punct.as_char(), punct.clone()));
                //         },
                //     }
                // }
                match self.parser.borrow().as_ref().unwrap().get_rust_parser().parse_rust_identifier_expression(add_first_ident, identifier_token, last_token_was_ident, it, ctx.clone(), ct.clone()) {
                    Ok(tokens_parsed) => {
                        ctx.push_output_tokens(tokens_parsed.to_splice());
                        Ok(())
                    },
                    Err(RustHtmlError(e)) => {
                        Err(RustHtmlError::from_string(format!("error parsing identifier expression 1: {}", e)))
                    }
                }
            },
            Err(e) => {
                Err(RustHtmlError::from_string(format!("error parsing identifier expression 2: {}", e)))
            }
        }
    }

    // get the next token and parse it as a literal or identifier expression that can be expanded to RustHtml tokens.
    // identifier: the identifier to expand.
    // it: the iterator to use.
    // returns: the expanded tokens or an error.
    fn expand_string_or_ident(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlIdentAndPunctAndGroupOrLiteral, RustHtmlError> {
        if let Some(expect_string_or_ident_token) = it.next() {
            match expect_string_or_ident_token {
                RustHtmlToken::Literal(literal, s) => {
                    Ok(RustHtmlIdentAndPunctAndGroupOrLiteral::Literal(literal.clone().unwrap()))
                },
                RustHtmlToken::Identifier(ref ident2) => {
                    let mut inner_tokens = vec![];
                    let inner_tokens_sharable = Rc::new(RefCell::new(inner_tokens));
                    ctx.push_output_buffer(inner_tokens_sharable);
                    self.parse_identifier_expression(true, ident2, &expect_string_or_ident_token, true, it.clone(), ctx.clone(), ct.clone())?;
                    Ok(RustHtmlIdentAndPunctAndGroupOrLiteral::IdentAndPunctAndGroup(self.expand_rusthtmltokens_to_ident_or_punct_or_group(inner_tokens, ctx.clone(), ct.clone())?))
                },
                _ => {
                    Err(RustHtmlError::from_string(format!("expand_string_or_ident did not find string or ident")))
                }
            }
        } else {
            Err(RustHtmlError::from_string(format!("expand_string_or_ident did not find string or ident")))
        }
    }

    // expand RustHtml tokens to a RustHtml identifier or punct or group.
    // tokens: the tokens to expand.
    // returns: the expanded tokens or an error.
    fn expand_rusthtmltokens_to_ident_or_punct_or_group(
        self: &Self, tokens: Vec<RustHtmlToken>,
        ctx: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<Vec<RustHtmlIdentOrPunctOrGroup>, RustHtmlError> {
        if tokens.len() == 0 {
            return Err(RustHtmlError::from_string(format!("tokens was empty")));
        }

        let tokens_vec = tokens.iter()
        .map(|x| match x {
            RustHtmlToken::Identifier(ident) => RustHtmlIdentOrPunctOrGroup::Ident(x.clone()),
            RustHtmlToken::ReservedChar(_, punct) => RustHtmlIdentOrPunctOrGroup::Punct(x.clone()),
            RustHtmlToken::Group(d, stream, group) => RustHtmlIdentOrPunctOrGroup::Group(x.clone()),
            RustHtmlToken::GroupParsed(delimiter, tokens) => {
                // let grouped: TokenStream = tokens.iter().map(|x| match x {
                //         RustHtmlToken::Identifier(ident) => TokenTree::Ident(ident.clone()),
                //         RustHtmlToken::ReservedChar(c, punct) => TokenTree::Punct(punct.clone()),
                //         RustHtmlToken::Group(d, stream, group) => TokenTree::Group(group.clone().unwrap()),
                //         _ => panic!("expand_rusthtmltokens_to_ident_or_punct_or_group Unexpected token {:?}", x),
                //     })
                //     .collect();
                // let grouped_rusthtml_result = self.expand_tokenstream(grouped.clone(), ctx.clone(), ct.clone());
                // let grouped_rusthtml_vec = match grouped_rusthtml_result {
                //     Ok(grouped_rusthtml) => grouped_rusthtml,
                //     Err(RustHtmlError(e)) => panic!("expand_rusthtmltokens_to_ident_or_punct_or_group error expanding group: {}", e),
                // };
                let grouped_rusthtml = Rc::new(VecPeekableRustHtmlToken::new(tokens.clone()));
                RustHtmlIdentOrPunctOrGroup::Group(
                    RustHtmlToken::Group(
                        delimiter.clone(),
                        grouped_rusthtml,
                        None,
                    )
                )
            },
            _ => panic!("expand_rusthtmltokens_to_ident_or_punct_or_group Unexpected token {:?}", x),
        })
        .collect();
        Ok(tokens_vec)
    }

    // iterate the iterator by one step (next) and expand a token tree to RustHtml tokens in the context of a HTML tag.
    // token_option: the token to expand.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // is_raw_tokenstream: whether the token stream is raw or not.
    // returns: whether we should break the outer loop or not, or an error.
    fn next_and_parse_html_tag(
        self: &Self,
        token: &RustHtmlToken,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        it: Rc<dyn IPeekableRustHtmlToken>,
        ct: Rc<dyn ICancellationToken>,
    ) -> Result<bool, RustHtmlError> {
        match token {
            RustHtmlToken::Identifier(ident) => {
                // println!("next_and_parse_html_tag: {:?}", token);
                self.expand_html_ident_to_rusthtmltoken(&ident, parse_ctx, it, ct)?;
            },
            RustHtmlToken::Literal(literal, s) => {
                // self.expand_html_literal_to_rusthtmltoken(literal.as_ref().unwrap(), parse_ctx, ct)?;
                parse_ctx.get_main_context().push_output_token(token.clone());
            },
            RustHtmlToken::ReservedChar(c, punct) => {
                // return self.expand_html_punct_to_rusthtmltoken(&punct, parse_ctx, it, ct);
                parse_ctx.get_main_context().push_output_token(token.clone());
            },
            _ => {
                return Err(RustHtmlError::from_string(format!("next_and_parse_html_tag Unexpected token {:?}", token)));
            },
        }
        Ok(false)
    }

    // expand a Rust identifier to a RustHtml token in the context of a HTML tag.
    // ident: the identifier to expand.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    fn expand_html_ident_to_rusthtmltoken(
        self: &Self, 
        ident: &Ident,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        it: Rc<dyn IPeekableRustHtmlToken>,
        ct: Rc<dyn ICancellationToken>,
    ) -> Result<(), RustHtmlError> {
        if parse_ctx.is_parsing_attrs() {
            if parse_ctx.is_parsing_attr_val() {
                parse_ctx.html_attr_val_ident_push(ident);
                self.on_kvp_defined(parse_ctx, ct)?;
            } else {
                parse_ctx.html_attr_key_ident_push(ident);
                parse_ctx.html_attr_key_push_str(&ident.to_string());
            }
        } else {
            parse_ctx.tag_name_push_ident(ident);
            let mut last_token_was_ident = true;
            loop {
                if ct.is_cancelled() {
                    return Err(RustHtmlError::from_string(format!("Task Cancelled")));
                }
                
                if let Some(next_token) = it.peek() {
                    match next_token {
                        RustHtmlToken::ReservedChar(ref c, ref punct) if punct.as_char() == '-' => {
                            parse_ctx.tag_name_push_punct(punct);
                            it.next();
                            last_token_was_ident = false;
                        },
                        RustHtmlToken::Identifier(ident) if last_token_was_ident == false => {
                            parse_ctx.tag_name_push_ident(ident);
                            it.next();
                            last_token_was_ident = true;
                        },
                        _ => {
                            parse_ctx.on_html_tag_name_parsed();
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }

        Ok(())
    }

    // // expand a Rust literal to a RustHtml token in the context of a HTML tag.
    // // literal: the literal to expand.
    // // parse_ctx: the parse context.
    // // output: the destination for the RustHtml tokens.
    // // returns: nothing or an error.
    // fn expand_html_literal_to_rusthtmltoken(
    //     self: &Self, 
    //     literal: &Literal,
    //     parse_ctx: Rc<dyn IHtmlTagParseContext>,
    //     ct: Rc<dyn ICancellationToken>
    // ) -> Result<(), RustHtmlError> {
    //     let html_parser = self.parser.borrow().as_ref().unwrap().get_html_parser();
    //     let x = html_parser.expand_html_literal_to_rusthtmltoken(literal, parse_ctx, ct);
    //     match x {
    //         Ok((y, _)) => {
    //             parse_ctx.get_main_context().push_output_tokens(&y);
    //             Ok(())
    //         },
    //         Err(RustHtmlError(e)) => {
    //             Err(RustHtmlError::from_string(e.into_owned()))
    //         }
    //     }
    // }

    // // expand a Rust punct to a RustHtml token in the context of a HTML tag.
    // // punct: the punct to expand.
    // // parse_ctx: the parse context.
    // // output: the destination for the RustHtml tokens.
    // // it: the iterator to use.
    // // is_raw_tokenstream: whether the token stream is raw or not.
    // // returns: whether we should break the outer loop or not, or an error.
    // fn expand_html_punct_to_rusthtmltoken(
    //     self: &Self, 
    //     punct: &Punct,
    //     parse_ctx: Rc<dyn IHtmlTagParseContext>,
    //     it: Rc<dyn IPeekableRustHtmlToken>,
    //     ct: Rc<dyn ICancellationToken>
    // ) -> Result<bool, RustHtmlError> {
    //     let parser = self.parser.borrow().as_ref().unwrap().get_html_parser();
    //     match parser.expand_html_punct_to_rusthtmltoken(punct, parse_ctx, it, ct) {
    //         Ok((tokens, r)) => {
    //             parse_ctx.get_main_context().push_output_tokens(&tokens);
    //             Ok(r)
    //         },
    //         Err(RustHtmlError(e)) => {
    //             Err(RustHtmlError::from_string(e.into_owned()))
    //         }
    //     }
    // }

    // called when a HTML tag attribute key/value pair is defined.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // returns: nothing.
    fn on_kvp_defined(
        self: &Self,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<(), RustHtmlError> {
        let r = parse_ctx.on_kvp_defined();
        match r {
            Ok(x) => {
                parse_ctx.get_main_context().push_output_tokens(&x);
                Ok(())
            },
            Err(RustHtmlError(e)) => {
                Err(RustHtmlError::from_string(format!("error on_kvp_defined: {}", e)))
            }
        }
    }

    // parse a Rust type identifier from a stream of tokens.
    // it: the iterator to use.
    // returns: the type identifier or an error.
    fn parse_type_identifier(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<Rc<dyn IPeekableRustHtmlToken>, RustHtmlError> {
        let new_parser = self.parser.borrow().as_ref().unwrap().get_rust_parser();
        match new_parser.parse_type_identifier(it, ct) {
            Ok(x) => Ok(x),
            Err(e) => Err(RustHtmlError::from_string(format!("error parsing type identifier: {}", e))),
        }
    }

    fn on_html_tag_parsed(
        self: &Self,
        end_punct: Option<&Punct>,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        ctx: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<bool, RustHtmlError> {
        if let Some(end_punct) = end_punct {
            parse_ctx.add_tag_end_punct(end_punct);   
        }
        let parser = self.parser.borrow().as_ref().unwrap().get_html_parser();
        match parser.on_html_tag_parsed(parse_ctx, ct) {
            Ok((tokens, r)) => {
                ctx.push_output_tokens(&tokens);
                Ok(r)
            },
            Err(RustHtmlError(e)) => {
                Err(RustHtmlError::from_string(e.into_owned()))
            }
        }
    }

    // called when a HTML node is parsed.
    // parse_ctx: the parse context.
    // output: the destination for the RustHtml tokens.
    // returns: whether we should break the outer loop or not, or an error.
    fn on_html_node_parsed(
        self: &Self,
        parse_ctx: Rc<dyn IHtmlTagParseContext>,
        ctx: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<bool, RustHtmlError> {
        for node_helper in ctx.get_node_parsed_handler() {
            if node_helper.matches(parse_ctx.tag_name_as_str().as_str()) {
                match node_helper.on_node_parsed(parse_ctx, ctx.clone()) {
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
        Ok(true)
    }

    // expand a Rust group, identifier, or literal to RustHtml tokens.
    // token: the token to expand.
    // output: the destination for the RustHtml tokens.
    // returns: nothing or an error.
    fn expand_copy(self: &Self, token: &RustHtmlToken, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        ctx.push_output_token(match token.clone() {
            RustHtmlToken::Literal(literal, s) => RustHtmlToken::Literal(literal, s),
            RustHtmlToken::Identifier(ident) => RustHtmlToken::Identifier(ident),
            RustHtmlToken::Group(d, stream, group) => {
                RustHtmlToken::Group(d, stream, group)
            },
            _ => {
                return Err(RustHtmlError::from_string(format!("unexpected token: {:?}", token)));
            },
        });
        Ok(())
    }

    // expand a RustHtml identifier or punct or group or literal to Rust tokens.
    // tag: the tag to expand.
    // returns: the expanded tokens or an error.
    fn expand_ident_and_punct_and_group_or_literal_to_tokenstream(self: &Self, tag: &RustHtmlIdentAndPunctAndGroupOrLiteral) -> Result<Rc<dyn IPeekableRustHtmlToken>, RustHtmlError> {
        Ok(Rc::new(VecPeekableRustHtmlToken::new(match tag {
            RustHtmlIdentAndPunctAndGroupOrLiteral::IdentAndPunctAndGroup(ident_and_punct) => {
                if ident_and_punct.len() == 0 {
                    return Err(RustHtmlError::from_string(format!("ident_and_punct was empty")));
                }
        
                ident_and_punct.iter()
                    .map(|x| match x {
                        RustHtmlIdentOrPunctOrGroup::Ident(ident) => ident.clone(),
                        RustHtmlIdentOrPunctOrGroup::Punct(punct) => punct.clone(),
                        RustHtmlIdentOrPunctOrGroup::Group(group) => group.clone(),
                    })
                    .collect()
            },
            RustHtmlIdentAndPunctAndGroupOrLiteral::Literal(literal) => vec![RustHtmlToken::Literal(Some(literal.clone()), Some(literal.to_string()))],
        })))
    }

    fn peek_reserved_char(self: &Self, expected_char: char, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<bool, RustHtmlError> {
        if let Some(next_token) = it.peek() {
            match next_token {
                RustHtmlToken::ReservedChar(next_char, next_punct) => {
                    if *next_char == expected_char {
                        // this is the expected char, so consume it
                        it.next();
                        ctx.push_output_token(next_token.clone());
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                },
                _ => {
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }
}