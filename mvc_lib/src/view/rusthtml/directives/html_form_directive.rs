use std::collections::HashMap;
use std::rc::Rc;
use std::vec;

use proc_macro::{Ident, TokenTree, Punct, Group};

use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
use crate::view::rusthtml::peekable_tokentree::{IPeekableTokenTree, PeekableTokenTree};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
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
    fn parse_form_function_call(self: &Self, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: &dyn IPeekableTokenTree) -> Result<RustHtmlDirectiveResult, RustHtmlError<'static>> {
        // println!("parsing form function call");

        // parse method
        // expecting string literal
        let method =
            if let Some(token_method) = it.peek() {
                match token_method {
                    TokenTree::Literal(literal) => {
                        it.next();
                        literal.to_string()
                    },
                    TokenTree::Ident(ident) => {
                        it.next();
                        ident.to_string()
                    },
                    TokenTree::Group(group) => {
                        // this is the main form render closure function.
                        // just use default method and action.
                        "POST".to_string()
                    }
                    _ => return Err(RustHtmlError::from_string(format!("expected string literal or identifier for method, not \"{}\"", token_method)))
                }
            } else {
                return Err(RustHtmlError::from_str("expected string literal, not EOF"));
            };

        // println!("method: {}", method);

        let mut form_render_fn_tokens: Option<Vec<RustHtmlToken>> = None;
        let mut attributes: Option<HashMap<String, Vec<RustHtmlToken>>> = None;
        let mut action: Option<String> = None;
        let mut route_values: Option<HashMap<String, Vec<RustHtmlToken>>> = None;

        // check for comma separator between method and action
        if Self::check_for_comma(it) {
            // skip comma
            it.next();

            // parse action
            // expecting string literal
            action =
                if let Some(token_action) = it.peek() {
                    if let TokenTree::Literal(literal) = token_action {
                        // skip literal
                        it.next();

                        // unescape string literal
                        Some(snailquote::unescape(&literal.to_string()).unwrap())
                    } else {
                        None
                    }
                } else {
                    None
                };

            if None == action {
                // set action based on controller name, action name, and route values
            }

            // check for comma separator between action and form attributes
            if Self::check_for_comma(it) {
                // skip comma
                it.next();

                // check for an object and parse form attributes
                attributes = Self::try_parse_object_html_attributes(it).clone()?;

                // check for comma separator between form attributes and action route values.
                if Self::check_for_comma(it) {
                    // skip comma
                    it.next();

                    // parse action route values
                    route_values = self.try_parse_object_route_values(it).clone()?;
                    
                    // check for comma separator between action route values and form render closure.
                    if Self::check_for_comma(it) {
                        // skip comma
                        it.next();
                    }
                }
            }
        }

        // parse form closure
        let mut form_render_fn_token_values = vec![];
        self.parse_form_render_closure(parser, &mut form_render_fn_token_values, it)?;
        form_render_fn_tokens = Some(form_render_fn_token_values);

        // add opening form tag
        output.extend_from_slice(&vec![
            RustHtmlToken::HtmlTagStart("form".to_string(), None),
            RustHtmlToken::HtmlTagAttributeName("method".to_string(), None),
            RustHtmlToken::HtmlTagAttributeEquals('=', None),
            RustHtmlToken::HtmlTagAttributeValue(Some(method), None),
            RustHtmlToken::HtmlTagAttributeName("action".to_string(), None),
            RustHtmlToken::HtmlTagAttributeEquals('=', None),
            RustHtmlToken::HtmlTagAttributeValue(action, None),
        ]);

        for attr_kvp in attributes.unwrap() {
            let attr_name = attr_kvp.0;
            let attr_values = attr_kvp.1;
            output.push(RustHtmlToken::HtmlTagAttributeName(attr_name, None));
            output.push(RustHtmlToken::HtmlTagAttributeEquals('=', None));
            output.push(RustHtmlToken::HtmlTagAttributeValue(None, Some(attr_values)));
        }

        output.push(RustHtmlToken::HtmlTagCloseStartChildrenPunct('>', None));

        // add form render closure tokens if present
        if let Some(form_render_tokens) = form_render_fn_tokens {
            output.extend_from_slice(&form_render_tokens);
        }

        // add closing form tag
        output.push(RustHtmlToken::HtmlTagEnd("form".to_string(), None));

        Ok(RustHtmlDirectiveResult::OkContinue)
    }

    fn parse_form_render_closure(self: &Self, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: &dyn IPeekableTokenTree) -> Result<(), RustHtmlError<'static>> {
        // expecting closure to render contents of form
        // must start with () to indicate it is a function
        match it.next() {
            Some(token) => {
                match token {
                    TokenTree::Group(group) => {
                        if group.delimiter() == proc_macro::Delimiter::Parenthesis {
                            // parse closure tokens in {}
                            match it.next() {
                                Some(token) => {
                                    match token {
                                        TokenTree::Group(group) => {
                                            let inner_it = Rc::new(PeekableTokenTree::new(group.stream()));
                                            match parser.parse_tokenstream_to_rusthtmltokens(true, inner_it, parser.get_context().get_is_raw_tokenstream()) {
                                                Ok(tokens) => {
                                                    output.extend_from_slice(tokens.as_slice());
                                                    return Ok(());
                                                },
                                                Err(RustHtmlError(err)) => {
                                                    return Err(RustHtmlError::from_string(err.as_ref().to_string()));
                                                }
                                            }
                                        },
                                        _ => return Err(RustHtmlError::from_string(format!("expected closure, not \"{}\"", token)))
                                    }
                                },
                                None => return Err(RustHtmlError::from_str("expected closure, not EOF"))
                            }
                        } else {
                            return Err(RustHtmlError::from_string(format!("expected braces, not \"{}\"", group)))
                        }
                    },
                    _ => return Err(RustHtmlError::from_string(format!("expected closure, not \"{}\"", token)))
                }
            },
            None => return Err(RustHtmlError::from_str("expected closure, not EOF"))
        }
    }
    
    fn try_parse_object_route_values(self: &Self, it: &dyn IPeekableTokenTree) -> Result<Option<HashMap<String, Vec<RustHtmlToken>>>, RustHtmlError<'static>> {
        if let Some(group_object) = Self::peek_group_with_braces(it) {
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
    
    fn try_parse_object_html_attributes(it: &dyn IPeekableTokenTree) -> Result<Option<HashMap<String, Vec<RustHtmlToken>>>, RustHtmlError<'static>> {
        if let Some(group_object) = Self::peek_group_with_braces(it) {
            // skip group after peeking
            it.next();

            // parse object from group tokens
            Self::parse_object_html_attributes(group_object)
        } else {
            Ok(None)
        }
    }
    
    fn peek_group_with_braces(it: &dyn IPeekableTokenTree) -> Option<Group> {
        if let Some(token) = it.peek() {
            if let TokenTree::Group(group) = token {
                if group.delimiter() == proc_macro::Delimiter::Brace {
                    return Some(group);
                }
            }
        }
    
        None
    }

    fn parse_object_html_attributes(group_object: proc_macro::Group) -> Result<Option<HashMap<String, Vec<RustHtmlToken>>>, RustHtmlError<'static>> {
        let mut object_attributes = HashMap::new();
        let mut object_attribute_name: Option<String> = None;
        let mut object_attribute_values: Vec<RustHtmlToken> = vec![];
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
                            if let Some(attribute_name) = object_attribute_name {
                                object_attribute_values.push(RustHtmlToken::Literal(Some(literal), Some(attribute_name)));
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
    
    fn check_for_comma(it: &dyn IPeekableTokenTree) -> bool {
        if let Some(token) = it.peek() {
            if let TokenTree::Punct(punct) = &token {
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

    fn execute(self: &Self, identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        // parse form function parameter values
        // top level ()
        // print!("parsing form function call");

        if let Some(token) = it.next() {
            if let TokenTree::Group(group) = token {
                return self.parse_form_function_call(parser, output, &PeekableTokenTree::new(group.stream()));
            } else {
                return Err(RustHtmlError::from_string(format!("expected function call group, not \"{}\"", token)));
            }
        } else {
            return Err(RustHtmlError::from_str("expected function call group, not EOF"));
        }
    }
}