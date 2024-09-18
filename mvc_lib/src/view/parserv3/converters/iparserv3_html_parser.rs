use std::cell::RefCell;
use std::fmt::format;
use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::Ident;

use crate::view::parserv3::contexts::html_tag_parse_context::HtmlTagParseContext;
use crate::view::parserv3::contexts::ihtml_tag_parse_context::IHtmlTagParseContext;
use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::vec_peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::{RustHtmlDirectiveResult, RustHtmlDirectiveResultV3};
use crate::view::parserv3::parserv3::IParserV3;
use crate::view::rusthtml::rusthtml_token::{RustHtmlIdentOrPunct, RustHtmlToken};
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

pub trait IParserV3HtmlParser {
    fn parse_tag(&self, input: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError>;
    fn next_and_parse_html_tag(&self, token: RustHtmlToken, input: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<Option<()>, RustHtmlError>;

    fn parse_tag_name(self: &Self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlIdentOrPunct>, RustHtmlError>;
    fn parse_tag_attribs(self: &Self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;
    fn on_html_node_parsed(self: &Self, parse_ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;

    fn parse_xml_ident(self: &Self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(Vec<RustHtmlToken>, Vec<RustHtmlIdentOrPunct>), RustHtmlError>;
    fn parse_literal_or_xml_name(self: &Self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(Vec<RustHtmlToken>, Vec<RustHtmlIdentOrPunct>), RustHtmlError>;
    fn parse_attrib_value(self: &Self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(Vec<RustHtmlToken>, Vec<RustHtmlIdentOrPunct>), RustHtmlError>;
    
    fn set_parser(&self, parser: Rc<dyn IParserV3>);
    fn get_parser(&self) -> Rc<dyn IParserV3>;
}

pub struct ParserV3HtmlParser {
    parser: RefCell<Option<Rc<dyn IParserV3>>>
}

impl ParserV3HtmlParser {
    pub fn new() -> Self {
        Self { parser: RefCell::new(None) }
    }

    pub fn check_next_char(&self, expected_c: char, optional: bool, input: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>) -> Result<bool, RustHtmlError> {
        if let Some(end_tag_token) = input.peek() {
            if let RustHtmlToken::ReservedChar(c, p) = end_tag_token {
                if c == expected_c {
                    input.next();
                    Ok(true)
                } else if optional {
                    Ok(false)
                } else {
                    Err(RustHtmlError::from_string(format!("Expected {}, received {}", expected_c, c)))
                }

            } else if optional {
                Ok(false)
            } else {
                Err(RustHtmlError::from_string(format!("Expected {} (char), received {:?}", expected_c, end_tag_token)))
            }
        } else if optional {
            Ok(false)
        } else {
            Err(RustHtmlError::from_string(format!("Expected {} (char), received nothing", expected_c)))
        }
    }
}

impl IParserV3HtmlParser for ParserV3HtmlParser {
    fn set_parser(&self, parser: Rc<dyn IParserV3>) {
        self.parser.replace(Some(parser));
    }

    fn parse_tag(&self, input: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError> {
        if ct.is_cancelled() {
            return Err(RustHtmlError::from_cancellationtoken(ct));
        }

        // tag name
        let ctx = Rc::new(HtmlTagParseContext::new(Some(context.clone())));

        // check that it is a closing tag?
        let is_closing_tag = self.check_next_char('/', true, input.clone(), context.clone())?;
        let tag_name = self.parse_tag_name(input.clone(), ctx.clone(), ct.clone())?;
        ctx.set_is_opening_tag(!is_closing_tag);

        // should be tag open, inner elements, tag close
        let mut output = vec![];

        output.push(ctx.on_html_tag_name_parsed(tag_name)?);

        if !is_closing_tag {
            let tag_attributes = self.parse_tag_attribs(input.clone(), ctx.clone(), ct.clone())?;

            // println!("tag_attributes ({}):", ctx.get_html_attrs().len());
            // print tag attributes
            for x in ctx.get_html_attrs() {
                println!("{} = {:?}", x.0, x.1)
            }
    
            let is_self_contained_tag = self.check_next_char('/', true, input.clone(), context.clone())?;
            ctx.set_is_self_contained_tag(is_self_contained_tag);    
            
            // let is_void = self.check_next_char('/', true, input.clone(), ctx.get_main_context())?;
            // ctx.set_is_void_tag(is_void);

            if ctx.is_void_tag() {
                output.push(RustHtmlToken::HtmlTagCloseVoidPunct(None))
            } else if is_self_contained_tag {
                output.push(RustHtmlToken::HtmlTagCloseSelfContainedPunct)
            } else {
                output.push(RustHtmlToken::HtmlTagCloseStartChildrenPunct)
            }
            
        }
        
        // assert next punct is >
        self.check_next_char('>', false, input.clone(), ctx.get_main_context())?;
        
        // let end_token = RustHtmlToken::HtmlTagEnd(ctx.tag_name_as_str(), Some(ctx.get_tag_name()));

        // tag children
        let mut output_inner = vec![];
        if ctx.is_opening_tag() && !ctx.is_void_tag() && !ctx.is_self_contained_tag() {
            // parse inner elements / code until we find closing tag
            context.htmltag_scope_stack_push(ctx.tag_name_as_str());
            loop {
                if ct.is_cancelled() {
                    return Err(RustHtmlError::from_cancellationtoken(ct));
                }

                let output_inner_partial = self.get_parser().get_converter_middle().convert(input.clone(), context.clone(), ct.clone())?;
                let output_inner_partial_vec = output_inner_partial.to_vec();

                if output_inner_partial_vec.is_empty() {
                    break;
                }

                let last = output_inner_partial_vec.last().unwrap().clone();
                output_inner.extend(output_inner_partial_vec);
                match last {
                    RustHtmlToken::HtmlTagEnd(tag_end, _tag_end_tokens) => {
                        if tag_end == ctx.tag_name_as_str() {
                            break;
                        }
                    },
                    _ => {
                    }
                }
            }

            let last_scope_from_stack = context.htmltag_scope_stack_pop().unwrap();
            if last_scope_from_stack != ctx.tag_name_as_str() {
                return Err(RustHtmlError::from_string(format!("Mismatched HTML tags (found {} but expected {})", last_scope_from_stack, ctx.tag_name_as_str())));
            }

            if let Some(output_inner_last) = output_inner.last() {
                if let RustHtmlToken::HtmlTagEnd(_tag_end, _tag_end_tokens) = output_inner_last {
                    self.on_html_node_parsed(ctx.clone(), ct.clone())?;
                }
            }
        }

        if ctx.get_add_inner() {
            output.extend_from_slice(&output_inner);
        } else {
            // add ending token
            if let Some(ending_token) = output_inner.last() {
                if let RustHtmlToken::HtmlTagEnd(x, a) = ending_token {
                    output.push(ending_token.clone())
                }
            }
        }

        let tag_stream = Rc::new(VecPeekableRustHtmlToken::new(output));

        Ok(RustHtmlDirectiveResultV3(RustHtmlDirectiveResult::OkContinue, Some(tag_stream)))
    }
    
    fn next_and_parse_html_tag(&self, token: RustHtmlToken, input: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<Option<()>, RustHtmlError> {
        todo!("next_and_parse_html_tag")
    }
    
    fn get_parser(&self) -> Rc<dyn IParserV3> {
        self.parser.borrow().as_ref().unwrap().clone()
    }
    
    fn on_html_node_parsed(self: &Self, context: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        for node_helper in context.get_main_context().get_node_parsed_handler() {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_cancellationtoken(ct));
            }

            if node_helper.matches(context.tag_name_as_str().as_str()) {
                match node_helper.on_node_parsed(context) {
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

        Ok(())
    }
    
    fn parse_tag_name(self: &Self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlIdentOrPunct>, RustHtmlError> {
        // just ident and -
        let mut output = vec![];

        // check for ! which is valid
        if let Some(RustHtmlToken::ReservedChar(c, punct)) = input.peek() {
            if c == '!' {
                output.push(RustHtmlIdentOrPunct::Punct(punct));
            } else {
                return Err(RustHtmlError::from_string(format!("parse_tag_name unexpected char {}", c)));
            }
        }

        let xml_name = self.parse_xml_ident(input, ctx.clone(), ct)?;
        output.extend(xml_name.1);
        // let xml_name_string = xml_name.1.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
        // println!("xml_name_string: {}", xml_name_string);
        Ok(output)
    }
    
    fn parse_tag_attribs(self: &Self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
        // key (= value)?
        while let Some(token) = input.peek() {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_cancellationtoken(ct));
            }

            match token {
                RustHtmlToken::Identifier(identifier) => {
                    // this moves the stream forward
                    let attrib_key = self.parse_xml_ident(input.clone(), ctx.clone(), ct.clone())?;
                    let attrib_key_string = ctx.fmt_tag_name_as_str(&attrib_key.1);
    
                    // check for =
                    if let Some(equals_token) = input.peek() {
                        if let RustHtmlToken::ReservedChar(c, p) = equals_token {
                            if c == '=' {
                                // proceed
                                input.next();

                                let attrib_value = self.parse_attrib_value(input.clone(), ctx.clone(), ct.clone())?;
                                // let attrib_value = self.get_parser().get_rust_parser().parse_string_with_quotes(false, &identifier, input.clone())?;

                                // values must be able to handle string literal or directive value
                                let attrib_value_group = if attrib_value.0.len() == 1 {
                                    attrib_value.0.first().unwrap().clone()
                                } else {
                                    let attrib_value_stream = Rc::new(VecPeekableRustHtmlToken::new(attrib_value.0));
                                    RustHtmlToken::Group(proc_macro2::Delimiter::Parenthesis, attrib_value_stream, None)
                                };
                                ctx.html_attrs_insert(attrib_key_string, Some(attrib_value_group));
                            } else {
                                return Err(RustHtmlError::from_string(format!("parse_tag_attribs unexpected char {}", c)));
                            }
                        } else {
                            return Err(RustHtmlError::from_string(format!("parse_tag_attribs unexpected token {:?}", equals_token)));
                        }
                    } else {
                        ctx.html_attrs_insert(attrib_key_string, None);
                        break;
                    }
                },
                RustHtmlToken::ReservedChar(c, p) => {
                    if c == '/' || c == '>' {
                        // end of tag
                        break;
                    } else {
                        return Err(RustHtmlError::from_string(format!("unexpected char in parse_tag_attribs: {}", c)));
                    }
                }
                _ => return Err(RustHtmlError::from_string(format!("unexpected token in parse_tag_attribs: {}", token.to_string()))),
            }
        }

        Ok(())
    }
    
    fn parse_xml_ident(self: &Self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(Vec<RustHtmlToken>, Vec<RustHtmlIdentOrPunct>), RustHtmlError> {
        let mut output_tokens = vec![];
        let mut output_typed = vec![];
        let mut last_token_was_ident = false;

        while let Some(token) = input.peek() {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_cancellationtoken(ct));
            }
            
            match &token {
                RustHtmlToken::Identifier(ident) => {
                    if last_token_was_ident {
                        break; // do not allow 2 idents (would be start of new attribute)
                    } else {
                        input.next();
                        output_tokens.push(token.clone());
                        output_typed.push(RustHtmlIdentOrPunct::Ident(ident.clone()));
                        last_token_was_ident = true;
                    }
                },
                RustHtmlToken::ReservedChar(c, punct) => {
                    if *c == '-' {
                        input.next();
                        output_tokens.push(token.clone());
                        output_typed.push(RustHtmlIdentOrPunct::Punct(punct.clone()));
                        last_token_was_ident = false;
                    } else {
                        // end of tag
                        break;
                    }
                },
                _ => {
                    break;
                }
            }
        }

        Ok((output_tokens, output_typed))
    }
    
    fn parse_literal_or_xml_name(self: &Self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(Vec<RustHtmlToken>, Vec<RustHtmlIdentOrPunct>), RustHtmlError> {
        if let Some(token) = input.peek() {
            match &token {
                RustHtmlToken::Literal(s, l) => Ok((vec![token.clone()], vec![])),
                RustHtmlToken::Identifier(i) => self.parse_xml_ident(input, ctx, ct),
                _ => Err(RustHtmlError::from_string(format!("Expected literal or ident, not {}", token.to_string())))
            }
        } else {
            Err(RustHtmlError::from_str("Expected literal or ident, at end of stream"))
        }
    }
    
    fn parse_attrib_value(self: &Self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(Vec<RustHtmlToken>, Vec<RustHtmlIdentOrPunct>), RustHtmlError> {
        // peek and check if is punct @ or else
        if let Some(RustHtmlToken::ReservedChar(c, p)) = input.peek() {
            if c == '@' {
                input.next();
                Ok((self.get_parser().get_rust_parser().parse_expression(input, ctx.get_main_context(), ct)?, vec![]))
            } else {
                Err(RustHtmlError::from_string(format!("parse_attrib_value unexpected character {}", c)))
            }
        } else {
            self.parse_literal_or_xml_name(input, ctx, ct)
        }
    }
}