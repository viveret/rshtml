use std::collections::HashMap;
use std::rc::Rc;
use std::vec;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::{Ident, TokenTree, Group, Delimiter, Literal};

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::peekable_rusthtmltoken::{VecPeekableRustHtmlToken, IPeekableRustHtmlToken};
use crate::view::rusthtml::parser_parts::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::parser_parts::peekable_tokentree::{IPeekableTokenTree, StreamPeekableTokenTree};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::rusthtml_token::{RustHtmlToken, RustHtmlIdentAndPunctOrLiteral, RustHtmlIdentOrPunct};
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use super::irusthtml_directive::IRustHtmlDirective;



// The "form" directive is used to render a form.
pub struct HtmlFormDirective {}

impl HtmlFormDirective {
    pub fn new() -> Self {
        Self {}
    }

    // parse the form function call and add the form tokens to the output.
    // the form function call is called between the form opening and closing tags.
    fn parse_form_function_call(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError<'static>> {
        // println!("parsing form function call");

        // parse method
        // expecting string literal
        let method =
            if let Some(token_method) = it.peek() {
                match token_method {
                    RustHtmlToken::Literal(literal, s) => {
                        it.next();
                        if let Some(literal) = literal {
                            (literal.to_string(), Some(literal.clone()), None)
                        } else if let Some(s) = s {
                            (s.clone(), None, None)
                        } else {
                            return Err(RustHtmlError::from_str("expected string / literal, both are None"));
                        }
                    },
                    RustHtmlToken::Identifier(ident) => {
                        it.next();
                        (ident.to_string(), None, Some(ident.clone()))
                    },
                    RustHtmlToken::Group(_delimiter, _stream, _group) => {
                        // this is the main form render closure function.
                        // just use default method and action.
                        ("POST".to_string(), None, None)
                    },
                    _ => return Err(RustHtmlError::from_string(format!("expected string literal or identifier for method, not \"{:?}\"", token_method)))
                }
            } else {
                return Err(RustHtmlError::from_str("expected string literal, not EOF"));
            };

        let mut _form_render_fn_tokens: Option<Vec<RustHtmlToken>> = None;
        let mut attributes: Option<HashMap<String, Vec<RustHtmlIdentAndPunctOrLiteral>>> = None;
        let mut action: Option<Literal> = None;
        let mut action_string: Option<String> = None;
        let mut _route_values: Option<HashMap<String, Vec<RustHtmlToken>>> = None;

        // check for comma separator between method and action
        if Self::check_for_comma(it.clone()) {
            // skip comma
            it.next();

            // parse action
            // expecting string literal
            action =
                if let Some(token_action) = it.peek() {
                    if let RustHtmlToken::Literal(literal, s) = token_action {
                        // skip literal
                        it.next();

                        if let Some(literal) = literal {
                            Some(literal.clone())
                        } else if let Some(s) = s {
                            Some(Literal::string(s))
                        } else {
                            return Err(RustHtmlError::from_str("expected string / literal, both are None"));
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

            if let Some(ref action) = action {
                action_string.replace(action.to_string());
            } else {
                // set action based on controller name, action name, and route values
            }

            // check for comma separator between action and form attributes
            if Self::check_for_comma(it.clone()) {
                // skip comma
                it.next();

                // check for an object and parse form attributes
                attributes = Self::try_parse_object_html_attributes(it.clone()).clone()?;

                // check for comma separator between form attributes and action route values.
                if Self::check_for_comma(it.clone()) {
                    // skip comma
                    it.next();

                    // parse action route values
                    _route_values = self.try_parse_object_route_values(it.clone()).clone()?;
                    
                    // check for comma separator between action route values and form render closure.
                    if Self::check_for_comma(it.clone()) {
                        // skip comma
                        it.next();
                    }
                }
            }
        }

        // parse form closure
        let mut form_render_fn_token_values = vec![];
        self.parse_form_render_closure(ctx, parser, &mut form_render_fn_token_values, it.clone(), ct)?;
        _form_render_fn_tokens = Some(form_render_fn_token_values);

        // add opening form tag
        output.extend_from_slice(&vec![
            RustHtmlToken::HtmlTagStart("form".to_string(), None),
            RustHtmlToken::HtmlTagAttributeName("method".to_string(), None),
            RustHtmlToken::HtmlTagAttributeEquals('=', None),
            RustHtmlToken::HtmlTagAttributeValue(Some(method.0), method.1, None, None),
            RustHtmlToken::HtmlTagAttributeName("action".to_string(), None),
            RustHtmlToken::HtmlTagAttributeEquals('=', None),
            RustHtmlToken::HtmlTagAttributeValue(action_string, action, None, None),
        ]);

        for attr_kvp in attributes.unwrap() {
            let attr_name = attr_kvp.0;
            let attr_values = attr_kvp.1;
            output.push(RustHtmlToken::HtmlTagAttributeName(attr_name, None));
            output.push(RustHtmlToken::HtmlTagAttributeEquals('=', None));

            match attr_values.first().unwrap() {
                RustHtmlIdentAndPunctOrLiteral::Literal(literal) => {
                    // let s = snailquote::unescape(&literal.to_string()).unwrap();
                    let s = literal.to_string();
                    output.push(RustHtmlToken::HtmlTagAttributeValue(Some(s), Some(literal.clone()), None, None));
                },
                RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(ident_or_punct) => {
                    let s = ident_or_punct.iter().map(|x| match x {
                        RustHtmlIdentOrPunct::Ident(ident) => ident.to_string(),
                        RustHtmlIdentOrPunct::Punct(punct) => punct.to_string(),
                    }).collect::<String>();
                    output.push(RustHtmlToken::HtmlTagAttributeValue(Some(s), None, Some(ident_or_punct.clone()), None));
                }
            }
        }

        output.push(RustHtmlToken::HtmlTagCloseStartChildrenPunct);

        // add form render closure tokens if present
        if let Some(form_render_tokens) = _form_render_fn_tokens {
            output.extend_from_slice(&form_render_tokens);
        }

        // add closing form tag
        output.push(RustHtmlToken::HtmlTagEnd("form".to_string(), None));

        Ok(RustHtmlDirectiveResult::OkContinue)
    }

    fn parse_form_render_closure(self: &Self,
        ctx: Rc<dyn IRustHtmlParserContext>,
        parser: Rc<dyn IRustHtmlParserAll>,
        output: &mut Vec<RustHtmlToken>,
        it: Rc<dyn IPeekableRustHtmlToken>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<(), RustHtmlError<'static>> {
        // expecting closure to render contents of form
        // must start with () to indicate it is a function
        match it.next() {
            Some(token) => {
                match token {
                    RustHtmlToken::Group(delimiter, stream, group) => {
                        if group.clone().unwrap().delimiter() == Delimiter::Parenthesis {
                            // parse closure tokens in {}
                            match it.next() {
                                Some(token) => {
                                    match token {
                                        RustHtmlToken::Group(delimiter, inner_it, group) => {
                                            match parser.get_html_parser().parse_html(ctx, inner_it.clone(), ct) {
                                                Ok((tokens, break_loop)) => {
                                                    output.extend(tokens);
                                                    return Ok(());
                                                },
                                                Err(RustHtmlError(err)) => {
                                                    return Err(RustHtmlError::from_string(err.as_ref().to_string()));
                                                }
                                            }
                                        },
                                        _ => return Err(RustHtmlError::from_string(format!("expected closure, not \"{:?}\"", token)))
                                    }
                                },
                                None => return Err(RustHtmlError::from_str("expected closure, not EOF"))
                            }
                        } else {
                            return Err(RustHtmlError::from_string(format!("expected braces, not \"{:?}\"", group)))
                        }
                    },
                    _ => return Err(RustHtmlError::from_string(format!("expected closure, not \"{:?}\"", token)))
                }
            },
            None => return Err(RustHtmlError::from_str("expected closure, not EOF"))
        }
    }
    
    fn try_parse_object_route_values(self: &Self, it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Option<HashMap<String, Vec<RustHtmlToken>>>, RustHtmlError<'static>> {
        if let Some(group_object) = Self::peek_group_with_braces(it.clone()) {
            // skip group after peeking
            it.next();

            // parse object from group tokens
            Self::parse_object_route_values(group_object)
        } else {
            Ok(None)
        }
    }
    
    fn parse_object_route_values(group_object: Group) -> Result<Option<HashMap<String, Vec<RustHtmlToken>>>, RustHtmlError<'static>> {
        let mut object_route_values = HashMap::new();
        let mut object_route_value_name: Option<String> = None;
        let mut object_route_value_values: Vec<RustHtmlToken> = vec![];
        let mut it =  group_object.stream().into_iter().peekable();
        loop {
            match it.next() {
                Some(token) => {
                    match token {
                        TokenTree::Ident(ident) => {
                            if object_route_value_name.is_none() {
                                object_route_value_name = Some(ident.to_string());
                            } else {
                                return Err(RustHtmlError::from_string(format!("expected route value, not \"{}\"", ident)));
                            }
                        },
                        TokenTree::Literal(literal) => {
                            if let Some(route_value_name) = object_route_value_name {
                                object_route_value_values.push(RustHtmlToken::Literal(Some(literal), Some(route_value_name)));
                                object_route_value_name = None;
                            } else {
                                return Err(RustHtmlError::from_string(format!("expected route value name, not \"{}\"", literal)));
                            }
                        },
                        TokenTree::Punct(punct) => {
                            if punct.as_char() == ',' {
                                if let Some(route_value_name) = object_route_value_name {
                                    object_route_values.insert(route_value_name, object_route_value_values);
                                    object_route_value_name = None;
                                    object_route_value_values = vec![];
                                } else {
                                    return Err(RustHtmlError::from_string(format!("expected route value name, not \"{}\"", punct)));
                                }
                            } else {
                                return Err(RustHtmlError::from_string(format!("expected \",\", not \"{}\"", punct)));
                            }
                        },
                        _ => return Err(RustHtmlError::from_string(format!("expected route value name, not \"{}\"", token)))
                    }
                },
                None => {
                    break;
                },
            }
        }
    
        Ok(Some(object_route_values))
    }
    
    fn try_parse_object_html_attributes(it: Rc<dyn IPeekableRustHtmlToken>) -> Result<Option<HashMap<String, Vec<RustHtmlIdentAndPunctOrLiteral>>>, RustHtmlError<'static>> {
        if let Some(group_object) = Self::peek_group_with_braces(it.clone()) {
            // skip group after peeking
            it.next();

            // parse object from group tokens
            Self::parse_object_html_attributes(group_object)
        } else {
            Ok(None)
        }
    }
    
    fn peek_group_with_braces(it: Rc<dyn IPeekableRustHtmlToken>) -> Option<Group> {
        if let Some(token) = it.peek() {
            if let RustHtmlToken::Group(delimiter, stream, group) = token {
                if *delimiter == Delimiter::Brace {
                    return Some(group.clone().unwrap().clone());
                }
            }
        }
    
        None
    }

    fn parse_object_html_attributes(group_object: Group) -> Result<Option<HashMap<String, Vec<RustHtmlIdentAndPunctOrLiteral>>>, RustHtmlError<'static>> {
        let mut object_attributes = HashMap::new();
        let mut object_attribute_name: Option<String> = None;
        let mut object_attribute_values: Vec<RustHtmlIdentAndPunctOrLiteral> = vec![];
        let mut it =  group_object.stream().into_iter().peekable();
        loop {
            match it.next() {
                Some(token) => {
                    // need to verify this since it was generated by copilot 
                    match token {
                        TokenTree::Ident(ident) => {
                            if object_attribute_name.is_none() {
                                object_attribute_name = Some(ident.to_string());
                            } else {
                                return Err(RustHtmlError::from_string(format!("expected attribute value, not \"{}\"", ident)));
                            }
                        },
                        TokenTree::Literal(literal) => {
                            if object_attribute_name.is_some() {
                                object_attribute_values.push(RustHtmlIdentAndPunctOrLiteral::Literal(literal));
                                object_attribute_name = None;
                            } else {
                                return Err(RustHtmlError::from_string(format!("expected attribute name, not \"{}\"", literal)));
                            }
                        },
                        TokenTree::Punct(punct) => {
                            if punct.as_char() == ',' {
                                if let Some(attribute_name) = object_attribute_name {
                                    object_attributes.insert(attribute_name, object_attribute_values);
                                    object_attribute_name = None;
                                    object_attribute_values = vec![];
                                } else {
                                    return Err(RustHtmlError::from_string(format!("expected attribute name, not \"{}\"", punct)));
                                }
                            } else {
                                return Err(RustHtmlError::from_string(format!("expected \",\", not \"{}\"", punct)));
                            }
                        },
                        _ => return Err(RustHtmlError::from_string(format!("expected attribute name, not \"{}\"", token)))
                    }
                },
                None => break,
            }
        }
        Ok(Some(object_attributes))
    }
    
    fn check_for_comma(it: Rc<dyn IPeekableRustHtmlToken>) -> bool {
        if let Some(token) = it.peek() {
            if let RustHtmlToken::ReservedChar(c, punct) = &token {
                if punct.as_char() == ',' {
                    return true;
                }
            }
        }
    
        false
    }
}

impl IRustHtmlDirective for HtmlFormDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "form"
    }

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, _identifier: &Ident, _ident_token: &RustHtmlToken, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableRustHtmlToken>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // parse form function parameter values
        // top level ()
        // print!("parsing form function call");

        if let Some(token) = it.next() {
            if let RustHtmlToken::Group(delimiter, inner_it, group) = token {
                return self.parse_form_function_call(context, parser, output, inner_it.clone(), ct);
            } else {
                return Err(RustHtmlError::from_string(format!("expected function call group, not \"{:?}\"", token)));
            }
        } else {
            return Err(RustHtmlError::from_str("expected function call group, not EOF"));
        }
    }
}