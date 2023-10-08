use std::cell::RefCell;
use std::rc::Rc;

use proc_macro2::TokenTree;

use crate::view::rusthtml::html_tag_parse_context::HtmlTagParseContext;
use crate::view::rusthtml::ihtml_tag_parse_context::IHtmlTagParseContext;
use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::rusthtmlparser_all::{IRustHtmlParserAll, IRustHtmlParserAssignSharedParts};



pub trait IRustHtmlParserHtml: IRustHtmlParserAssignSharedParts {
    fn parse_html(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    fn parse_html_tag(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_html_node(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_html_attr_key(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
    fn parse_html_attr_val(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;

    fn parse_html_child_nodes(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
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
    fn parse_html(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableTokenTree>, _is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        loop {
            let next_token = it.peek();
            if let Some(token) = next_token {
                match token {
                    TokenTree::Group(group) => {
                        todo!()
                    },
                    TokenTree::Ident(ident) => {
                        todo!()
                    },
                    TokenTree::Literal(literal) => {
                        todo!()
                    },
                    TokenTree::Punct(punct) => {
                        todo!()
                    },
                }
            }
        }
        Ok(output)
    }

    fn parse_html_tag(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        // parse tag name
        // parse attributes (key value pairs)
        // parse closing puncts
        todo!()
    }

    // TODO: add tests
    fn parse_html_node(self: &Self, ctx: Rc<dyn IRustHtmlParserContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        let mut output = vec![];
        let mut parse_ctx = Rc::new(HtmlTagParseContext::new(Some(ctx))) as Rc<dyn IHtmlTagParseContext>;
        todo!();
        Ok(output)
    }

    // TODO: add tests
    fn parse_html_attr_key(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        // must be an identifier punct combo
        let mut output = vec![];
        let mut last_was_ident = false;
        loop {
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
    fn parse_html_attr_val(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        // can only be literal or identifier punct combo
        let mut output = vec![];
        let mut last_was_ident = false;
        loop {
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

    fn parse_html_child_nodes(self: &Self, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<dyn IPeekableTokenTree>, is_raw_tokenstream: bool) -> Result<Vec<RustHtmlToken>, RustHtmlError> {
        todo!()
    }

}
