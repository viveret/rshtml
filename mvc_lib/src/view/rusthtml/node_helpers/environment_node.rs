use std::rc::Rc;

use crate::view::parserv3::contexts::ihtml_tag_parse_context::IHtmlTagParseContext;
use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::converters::inode_parsed::IHtmlNodeParsed;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::{RustHtmlIdentAndPunctOrLiteral, RustHtmlIdentOrPunct, RustHtmlToken};

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

    fn on_node_parsed(&self, tag_context: Rc<dyn IHtmlTagParseContext>) -> Result<bool, RustHtmlError> {
        // look for include or exclude attributes
        let mut keep_or_remove: Option<bool> = None;

        let html_context = tag_context.get_main_context();

        match tag_context.get_html_attr("include") {
            Some(ref tokens) => {
                for token in tokens {
                    match token {
                        RustHtmlToken::HtmlTagAttributeValue(value_string, value_literal, v_parts, rust_value) => {
                            if let Some(rust_value) = rust_value {
                                for v in rust_value {
                                    match v {
                                        RustHtmlToken::Literal(literal, string) => {
                                            let literal_as_str = snailquote::unescape(& if let Some(literal) = literal { literal.to_string() } else { string.clone().unwrap_or_default() }).expect("snailquote::unescape");
                                            keep_or_remove = Some(html_context.get_environment_name() == literal_as_str);
                                        },
                                        _ => panic!("Unexpected token for environment tag value (rust value): {:?}", token),
                                    }
                                }
                            } else if let Some(v_parts) = v_parts {
                                match &v_parts {
                                    RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(ident_and_punct) => {
                                        for v in ident_and_punct {
                                            match v {
                                                RustHtmlIdentOrPunct::Ident(ident) => {
                                                    if html_context.get_environment_name() == ident.to_string() {
                                                        keep_or_remove = Some(true);
                                                    } else {
                                                        println!("self.environment_name ({}) DOES NOT match ident.to_string() ({})", html_context.get_environment_name(), ident.to_string());
                                                        keep_or_remove = Some(false);
                                                    }
                                                },
                                                RustHtmlIdentOrPunct::Punct(punct) => todo!(),
                                            }
                                        }
                                    },
                                    _ => panic!("Unexpected token for environment tag value (v_parts): {:?}", token),
                                }
                            } else {
                                if let Some(v) = value_string {
                                    let v_as_str = snailquote::unescape(&v).expect("snailquote::unescape");
                                    println!("v_as_str: {}", v_as_str);
        
                                    keep_or_remove = Some(html_context.get_environment_name() == v_as_str);
                                } else if let Some(value_literal) = value_literal {
                                    keep_or_remove = Some(html_context.get_environment_name() == value_literal.to_string());
                                } else {
                                    panic!("Unexpected token for environment tag (value_string): {:?}", token);
                                }
                            }
                        },
                        RustHtmlToken::Literal(literal, string) => {
                            let literal_as_str = snailquote::unescape(& if let Some(literal) = literal { literal.to_string() } else { string.clone().unwrap_or_default() }).expect("snailquote::unescape");
                            println!("literal_as_str: {}", literal_as_str);
                            keep_or_remove = Some(html_context.get_environment_name() == literal_as_str);
                        },
                        _ => panic!("Unexpected token for environment tag (token): {:?}", token),
                    }
                }
            },
            None => {
                // println!("environment tag does not have include field");
            }
        }
        
        match tag_context.get_html_attr("exclude") {
            Some(ref tokens) => {
                for token in tokens {
                    match token {
                        RustHtmlToken::HtmlTagAttributeValue(value_string, value_literal, v_parts, rust_value) => {
                            if let Some(v_parts) = v_parts {
                                match &v_parts {
                                    RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(ident_and_punct) => {
                                        for v in ident_and_punct {
                                            match v {
                                                RustHtmlIdentOrPunct::Ident(ident) => {
                                                    if html_context.get_environment_name() != ident.to_string() {
                                                        keep_or_remove = Some(true);
                                                    } else {
                                                        println!("self.environment_name ({}) DOES match ident.to_string() ({})", html_context.get_environment_name(), ident.to_string());
                                                        keep_or_remove = Some(false);
                                                    }
                                                },
                                                RustHtmlIdentOrPunct::Punct(punct) => todo!(),
                                            }
                                        }
                                    },
                                    _ => panic!("Unexpected token for environment tag value (v_parts): {:?}", token),
                                }
                            } else if let Some(rust_value) = rust_value {
                                for v in rust_value {
                                    match v {
                                        RustHtmlToken::Literal(literal, string) => {
                                            let literal_as_str = snailquote::unescape(& if let Some(literal) = literal { literal.to_string() } else { string.clone().unwrap_or_default() }).expect("snailquote::unescape");
                                            println!("literal_as_str: {}", literal_as_str);
                                            if html_context.get_environment_name() != literal_as_str {
                                                keep_or_remove = Some(true);
                                            } else {
                                                println!("self.environment_name ({}) DOES match literal_as_str ({})", html_context.get_environment_name(), literal_as_str);
                                                keep_or_remove = Some(false);
                                            }
                                        },
                                        _ => panic!("Unexpected token for environment tag value (rust_value): {:?}", token),
                                    }
                                }
                            }
                            if let Some(value) = value_string {
                                let value_as_str = snailquote::unescape(&value).expect("snailquote::unescape");
                                println!("value_as_str: {}", value_as_str);

                                if html_context.get_environment_name() != value_as_str {
                                    keep_or_remove = Some(true);
                                } else {
                                    println!("self.environment_name ({}) DOES match value_as_str ({})", html_context.get_environment_name(), value_as_str);
                                    keep_or_remove = Some(false);
                                }
                            } else if let Some(value_literal) = value_literal {
                                if html_context.get_environment_name() != value_literal.to_string() {
                                    keep_or_remove = Some(true);
                                } else {
                                    println!("self.environment_name ({}) DOES match value_as_str ({})", html_context.get_environment_name(), value_literal.to_string());
                                    keep_or_remove = Some(false);
                                }
                            } else {
                                panic!("Unexpected token for environment tag (invalid or unsupported): {:?}", token);
                            }
                        }
                        RustHtmlToken::Literal(literal, string) => {
                            let literal_as_str = snailquote::unescape(& if let Some(literal) = literal { literal.to_string() } else { string.clone().unwrap_or_default() }).expect("snailquote::unescape");
                            println!("literal_as_str: {}", literal_as_str);
                            if html_context.get_environment_name() != literal_as_str {
                                keep_or_remove = Some(true);
                            } else {
                                println!("self.environment_name ({}) DOES match literal_as_str ({})", html_context.get_environment_name(), literal_as_str);
                                keep_or_remove = Some(false);
                            }
                        },
                        _ => panic!("Unexpected token for environment tag (token): {:?}", token),
                    }
                }
            },
            None => {
                // println!("environment tag does not have exclude field");
            }
        }

        // let output = tag_context.get_main_context().get_output_buffer().expect("tag_context.get_main_context().get_output_buffer()");
        
        return match keep_or_remove {
            Some(keep_or_remove) => {
                if keep_or_remove {
                    // do not add environment tag but do add child nodes
                    tag_context.set_only_add_inner(true);
                    Ok(true)
                } else {
                    // do not add anything
                    tag_context.set_ignore(true);
                    Ok(false)
                }
            },
            None => {
                Err(RustHtmlError::from_string(format!("rust html tag environment expects attribute 'include' or 'exclude' to be defined (attrs: {:?})", tag_context.get_html_attrs())))
            }
        }
    }
}