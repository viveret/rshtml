use std::rc::Rc;

use crate::view::rusthtml::ihtml_tag_parse_context::IHtmlTagParseContext;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::{RustHtmlToken, RustHtmlIdentOrPunct};
use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;

use super::inode_parsed::IHtmlNodeParsed;

// The EnvironmentHtmlNodeParsed struct is used to parse the environment tag.
// The environment tag is used to conditionally render a section of the view based on the environment name.
pub struct EnvironmentHtmlNodeParsed {}

impl EnvironmentHtmlNodeParsed {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl IHtmlNodeParsed for EnvironmentHtmlNodeParsed {
    fn matches(&self, tag_name: &str) -> bool {
        return tag_name == "environment";
    }

    fn on_node_parsed(&self, tag_context: Rc<dyn IHtmlTagParseContext>, html_context: Rc<dyn IRustHtmlParserContext>) -> Result<bool, RustHtmlError> {
        // look for include or exclude attributes
        let mut _keep_or_remove: Option<bool> = None;

        match tag_context.get_html_attr("include") {
            Some(ref token) => {
                match token {
                    RustHtmlToken::HtmlTagAttributeValue(value_string, value_literal, v_parts, rust_value) => {
                        if let Some(rust_value) = rust_value {
                            for v in rust_value {
                                match v {
                                    RustHtmlToken::Literal(literal, string) => {
                                        let literal_as_str = snailquote::unescape(& if let Some(literal) = literal { literal.to_string() } else { string.clone().unwrap_or_default() }).expect("snailquote::unescape");
                                        _keep_or_remove = Some(html_context.get_environment_name() == literal_as_str);
                                    },
                                    _ => panic!("Unexpected token for environment tag value (rust value): {:?}", token),
                                }
                            }
                        } else if let Some(v_parts) = v_parts {
                            for v in v_parts {
                                match v {
                                    RustHtmlIdentOrPunct::Ident(ident) => {
                                        if html_context.get_environment_name() == ident.to_string() {
                                            _keep_or_remove = Some(true);
                                        } else {
                                            // println!("self.environment_name ({}) DOES NOT match literal_as_str ({})", self.environment_name, literal_as_str);
                                            _keep_or_remove = Some(false);
                                        }
                                    },
                                    _ => panic!("Unexpected token for environment tag value (v_parts): {:?}", token),
                                }
                            }
                        } else {
                            if let Some(v) = value_string {
                                let v_as_str = snailquote::unescape(&v).expect("snailquote::unescape");
                                // println!("v_as_str: {}", v_as_str);
    
                                _keep_or_remove = Some(html_context.get_environment_name() == v_as_str);
                            } else if let Some(value_literal) = value_literal {
                                _keep_or_remove = Some(html_context.get_environment_name() == value_literal.to_string());
                            } else {
                                panic!("Unexpected token for environment tag (value_string): {:?}", token);
                            }
                        }
                    }
                    RustHtmlToken::Literal(literal, string) => {
                        let literal_as_str = snailquote::unescape(& if let Some(literal) = literal { literal.to_string() } else { string.clone().unwrap_or_default() }).expect("snailquote::unescape");
                        // println!("literal_as_str: {}", literal_as_str);
                        _keep_or_remove = Some(html_context.get_environment_name() == literal_as_str);
                    },
                    _ => panic!("Unexpected token for environment tag (token): {:?}", token),
                }
            },
            None => {
                // println!("environment tag does not have include field");
            }
        }
        
        match tag_context.get_html_attr("exclude") {
            Some(ref token) => {
                match token {
                    RustHtmlToken::HtmlTagAttributeValue(value_string, value_literal, v_parts, rust_value) => {
                        if let Some(v_parts) = v_parts {
                            for v in v_parts {
                                match v {
                                    RustHtmlIdentOrPunct::Ident(ident) => {
                                        if html_context.get_environment_name() != ident.to_string() {
                                            _keep_or_remove = Some(true);
                                        } else {
                                            // println!("self.environment_name ({}) DOES match literal_as_str ({})", self.environment_name, literal_as_str);
                                            _keep_or_remove = Some(false);
                                        }
                                    },
                                    _ => panic!("Unexpected token for environment tag value (v_parts): {:?}", token),
                                }
                            }
                        } else if let Some(rust_value) = rust_value {
                            for v in rust_value {
                                match v {
                                    RustHtmlToken::Literal(literal, string) => {
                                        let literal_as_str = snailquote::unescape(& if let Some(literal) = literal { literal.to_string() } else { string.clone().unwrap_or_default() }).expect("snailquote::unescape");
                                        // println!("literal_as_str: {}", literal_as_str);
                                        if html_context.get_environment_name() != literal_as_str {
                                            _keep_or_remove = Some(true);
                                        } else {
                                            // println!("self.environment_name ({}) DOES match literal_as_str ({})", self.environment_name, literal_as_str);
                                            _keep_or_remove = Some(false);
                                        }
                                    },
                                    _ => panic!("Unexpected token for environment tag value (rust_value): {:?}", token),
                                }
                            }
                        }
                        if let Some(value) = value_string {
                            let value_as_str = snailquote::unescape(&value).expect("snailquote::unescape");
                            // println!("value_as_str: {}", value_as_str);

                            if html_context.get_environment_name() != value_as_str {
                                _keep_or_remove = Some(true);
                            } else {
                                // println!("self.environment_name ({}) DOES match value_as_str ({})", self.environment_name, value_as_str);
                                _keep_or_remove = Some(false);
                            }
                        } else if let Some(value_literal) = value_literal {
                            if html_context.get_environment_name() != value_literal.to_string() {
                                _keep_or_remove = Some(true);
                            } else {
                                // println!("self.environment_name ({}) DOES match value_as_str ({})", self.environment_name, value_as_str);
                                _keep_or_remove = Some(false);
                            }
                        } else {
                            panic!("Unexpected token for environment tag (invalid or unsupported): {:?}", token);
                        }
                    }
                    RustHtmlToken::Literal(literal, string) => {
                        let literal_as_str = snailquote::unescape(& if let Some(literal) = literal { literal.to_string() } else { string.clone().unwrap_or_default() }).expect("snailquote::unescape");
                        // println!("literal_as_str: {}", literal_as_str);
                        if html_context.get_environment_name() != literal_as_str {
                            _keep_or_remove = Some(true);
                        } else {
                            // println!("self.environment_name ({}) DOES match literal_as_str ({})", self.environment_name, literal_as_str);
                            _keep_or_remove = Some(false);
                        }
                    },
                    _ => panic!("Unexpected token for environment tag (token): {:?}", token),
                }
            },
            None => {
                // println!("environment tag does not have exclude field");
            }
        }

        let binding = tag_context.get_main_context().get_output_buffer().expect("tag_context.get_main_context().get_output_buffer()");
        let mut output = binding.borrow_mut();
        
        return match _keep_or_remove {
            Some(_keep_or_remove) => {
                if _keep_or_remove {
                    // keep - don't add outer environment tags but do add inner elements
                    loop {
                        match output.first().expect("output.first()") {
                            RustHtmlToken::HtmlTagCloseVoidPunct(_) |
                            RustHtmlToken::HtmlTagCloseSelfContainedPunct |
                            RustHtmlToken::HtmlTagCloseStartChildrenPunct => {
                                output.remove(0);
                                break;
                            },
                            _ => {
                                output.remove(0);
                            }
                        }
                    }
                    
                    match output.last() {
                        Some(RustHtmlToken::HtmlTagEnd(tag_end, _tag_end_tokens)) => {
                            if tag_end == &tag_context.tag_name_as_str() {
                                let _pop_result = output.pop();
                            } else {
                                panic!("mismatch while processing environment HTML tag (found {})", tag_end);
                            }
                        },
                        Some(_) => {
                            panic!("unexpected token while processing environment HTML tag (found {:?})", output.last().expect("output.last()"));
                        },
                        _ => {}
                    }

                    // do not add environment tag start but do add child nodes
                    Ok(true)
                } else {
                    // do not add anything
                    Ok(false)
                }
            },
            None => {
                Err(RustHtmlError::from_string(format!("rust html tag environment expects attribute 'include' or 'exclude' to be defined (attrs: {:?})", tag_context.get_html_attrs())))
            }
        }
    }
}