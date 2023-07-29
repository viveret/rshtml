use std::rc::Rc;

use crate::view::rusthtml::{html_tag_parse_context::HtmlTagParseContext, rusthtml_parser_context::IRustHtmlParserContext, rusthtml_token::{RustHtmlToken, RustHtmlIdentOrPunct}, rusthtml_error::RustHtmlError};

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

    fn on_node_parsed(&self, tag_context: &HtmlTagParseContext, html_context: Rc<dyn IRustHtmlParserContext>, output: &mut Vec<RustHtmlToken>) -> Result<bool, RustHtmlError> {
        // look for include or exclude attributes
        let mut keep_or_remove: Option<bool> = None;

        match tag_context.html_attrs.get("include") {
            Some(token) => {
                match token.clone().unwrap() {
                    RustHtmlToken::HtmlTagAttributeValue(value_string, value_literal, v_parts, rust_value) => {
                        if let Some(rust_value) = rust_value {
                            for v in rust_value {
                                match v {
                                    RustHtmlToken::Literal(literal, string) => {
                                        let literal_as_str = snailquote::unescape(& if let Some(literal) = literal { literal.to_string() } else { string.unwrap_or_default() }).unwrap();
                                        keep_or_remove = Some(html_context.get_environment_name() == literal_as_str);
                                    },
                                    _ => panic!("Unexpected token for environment tag value: {:?}", token),
                                }
                            }
                        } else if let Some(v_parts) = v_parts {
                            for v in v_parts {
                                match v {
                                    RustHtmlIdentOrPunct::Ident(ident) => {
                                        if html_context.get_environment_name() == ident.to_string() {
                                            keep_or_remove = Some(true);
                                        } else {
                                            // println!("self.environment_name ({}) DOES NOT match literal_as_str ({})", self.environment_name, literal_as_str);
                                            keep_or_remove = Some(false);
                                        }
                                    },
                                    _ => panic!("Unexpected token for environment tag value: {:?}", token),
                                }
                            }
                        } else {
                            if let Some(v) = value_string {
                                let v_as_str = snailquote::unescape(&v).unwrap();
                                // println!("v_as_str: {}", v_as_str);
    
                                keep_or_remove = Some(html_context.get_environment_name() == v_as_str);
                            } else if let Some(value_literal) = value_literal {
                                keep_or_remove = Some(html_context.get_environment_name() == value_literal.to_string());
                            } else {
                                panic!("Unexpected token for environment tag: {:?}", token);
                            }
                        }
                    }
                    _ => panic!("Unexpected token for environment tag: {:?}", token),
                }
            },
            None => {
                // println!("environment tag does not have include field");
            }
        }
        
        match tag_context.html_attrs.get("exclude") {
            Some(token) => {
                match token.clone().unwrap() {
                    RustHtmlToken::HtmlTagAttributeValue(value_string, value_literal, v_parts, rust_value) => {
                        if let Some(v_parts) = v_parts {
                            for v in v_parts {
                                match v {
                                    RustHtmlIdentOrPunct::Ident(ident) => {
                                        if html_context.get_environment_name() != ident.to_string() {
                                            keep_or_remove = Some(true);
                                        } else {
                                            // println!("self.environment_name ({}) DOES match literal_as_str ({})", self.environment_name, literal_as_str);
                                            keep_or_remove = Some(false);
                                        }
                                    },
                                    _ => panic!("Unexpected token for environment tag value: {:?}", token),
                                }
                            }
                        } else if let Some(rust_value) = rust_value {
                            for v in rust_value {
                                match v {
                                    RustHtmlToken::Literal(literal, string) => {
                                        let literal_as_str = snailquote::unescape(& if let Some(literal) = literal { literal.to_string() } else { string.unwrap_or(String::default()) }).unwrap();
                                        // println!("literal_as_str: {}", literal_as_str);
                                        if html_context.get_environment_name() != literal_as_str {
                                            keep_or_remove = Some(true);
                                        } else {
                                            // println!("self.environment_name ({}) DOES match literal_as_str ({})", self.environment_name, literal_as_str);
                                            keep_or_remove = Some(false);
                                        }
                                    },
                                    _ => panic!("Unexpected token for environment tag value: {:?}", token),
                                }
                            }
                        }
                        if let Some(value) = value_string {
                            let value_as_str = snailquote::unescape(&value).unwrap();
                            // println!("value_as_str: {}", value_as_str);

                            if html_context.get_environment_name() != value_as_str {
                                keep_or_remove = Some(true);
                            } else {
                                // println!("self.environment_name ({}) DOES match value_as_str ({})", self.environment_name, value_as_str);
                                keep_or_remove = Some(false);
                            }
                        } else if let Some(value_literal) = value_literal {
                            if html_context.get_environment_name() != value_literal.to_string() {
                                keep_or_remove = Some(true);
                            } else {
                                // println!("self.environment_name ({}) DOES match value_as_str ({})", self.environment_name, value_as_str);
                                keep_or_remove = Some(false);
                            }
                        } else {
                            panic!("Unexpected token for environment tag: {:?}", token);
                        }
                    }
                    _ => panic!("Unexpected token for environment tag: {:?}", token),
                }
            },
            None => {
                // println!("environment tag does not have exclude field");
            }
        }
        
        return match keep_or_remove {
            Some(keep_or_remove) => {
                if keep_or_remove {
                    // keep - don't add outer environment tags but do add inner elements
                    loop {
                        match output.first().unwrap() {
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
                    
                    match output.last().unwrap() {
                        RustHtmlToken::HtmlTagEnd(tag_end, _tag_end_tokens) => {
                            if tag_end == &tag_context.tag_name_as_str() {
                                let _pop_result = output.pop();
                                // println!("output.pop(): {:?}", pop_result);
                            } else {
                                println!("mismatch while processing environment HTML tag (found {})", tag_end);
                            }
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
                Err(RustHtmlError::from_string(format!("rust html tag environment expects attribute 'include' or 'exclude' to be defined (attrs: {:?})", tag_context.html_attrs)))
            }
        }
    }
}