use std::cell::RefCell;
use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;

use crate::view::parserv3::contexts::html_tag_parse_context::HtmlTagParseContext;
use crate::view::parserv3::contexts::ihtml_tag_parse_context::IHtmlTagParseContext;
use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::parserv3::core::peekable::empty_peekable_rusthtmltoken::EmptyPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::ipeekable_rusthtmltoken::IPeekableRustHtmlToken;
use crate::view::parserv3::core::peekable::vec_peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use crate::view::parserv3::core::rusthtml_directive_result::{RustHtmlDirectiveResult, RustHtmlDirectiveResultV3};
use crate::view::parserv3::parserv3::IParserV3;
use crate::view::rusthtml::rusthtml_token::{RustHtmlIdentAndPunctOrLiteral, RustHtmlIdentOrPunct, RustHtmlToken};
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

pub trait IParserV3HtmlParser {
    fn parse_tag(&self, input: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) -> Result<RustHtmlDirectiveResultV3, RustHtmlError>;

    fn parse_tag_name(&self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlIdentOrPunct>, RustHtmlError>;
    fn parse_tag_attribs(&self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;
    fn on_html_node_parsed(&self, parse_ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError>;

    fn parse_xml_ident(&self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(Vec<RustHtmlToken>, Vec<RustHtmlIdentOrPunct>), RustHtmlError>;
    fn parse_literal_or_xml_name(&self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(Vec<RustHtmlToken>, Vec<RustHtmlIdentOrPunct>), RustHtmlError>;
    fn parse_attrib_value(&self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(Vec<RustHtmlToken>, Vec<RustHtmlIdentOrPunct>), RustHtmlError>;
    
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
    
    fn parse_tag_start_close(&self, input: &Rc<dyn IPeekableRustHtmlToken>, context: &Rc<dyn IRustHtmlParserContext>, ct: &Rc<dyn ICancellationToken>, ctx: &Rc<HtmlTagParseContext>, output: &mut Vec<RustHtmlToken>) -> Result<(), RustHtmlError> {
        self.parse_tag_attribs(input.clone(), ctx.clone(), ct.clone())?;
        let tag_attrs_output_tokens = ctx.get_html_attrs_output();
        output.extend_from_slice(&tag_attrs_output_tokens);
        let is_self_contained_tag = self.check_next_char('/', true, input.clone(), context.clone())?;
        ctx.set_is_self_contained_tag(is_self_contained_tag);
        Ok(if ctx.is_void_tag() {
            let tag_name_str = ctx.tag_name_as_str();
            output.push(RustHtmlToken::HtmlTagCloseVoidPunct(tag_name_str, None))
        } else if is_self_contained_tag {
            output.push(RustHtmlToken::HtmlTagCloseSelfContainedPunct)
        } else {
            output.push(RustHtmlToken::HtmlTagCloseStartChildrenPunct)
        })
    }
    
    fn parse_tag_inner_contents(&self, input: Rc<dyn IPeekableRustHtmlToken>, context: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>, ctx: &Rc<HtmlTagParseContext>, output: &mut Vec<RustHtmlToken>) -> Result<(), RustHtmlError> {
        let mut output_inner = vec![];
        context.htmltag_scope_stack_push(ctx.tag_name_as_str());

        // this loop is not broken when getting to the end tag
        // (the inner convert is greedy and reads to the end of the steam)
        // so it causes a panic when it returns wrong end tag
        loop {
            if ct.is_cancelled() {
                return Err(RustHtmlError::from_cancellationtoken(ct));
            }
    
            println!("output_inner_partial_vec for {} (before)", ctx.tag_name_as_str());
    
            let output_inner_partial = self.get_parser().get_converter_middle().convert(input.clone(), context.clone(), ct.clone())?;
            let output_inner_partial_vec = output_inner_partial.1.unwrap().to_vec();
    
            if output_inner_partial_vec.is_empty() {
                break;
            }
    
            // println!("output_inner_partial_vec for {}: {}", ctx.tag_name_as_str(), output_inner_partial_vec.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    
            if let Some(last) = output_inner_partial_vec.last() {
                let last = last.clone();
                output_inner.extend(output_inner_partial_vec);
                match last {
                    RustHtmlToken::HtmlTagEnd(tag_end, _tag_end_tokens) => {
                        if tag_end == ctx.tag_name_as_str() {
                            println!("found matching end tag {}", tag_end);
                            break;
                        } else {
                            return Err(RustHtmlError::from_string(format!(
                                "Mismatched HTML tags inner (found {} but expected {})",
                                tag_end, ctx.tag_name_as_str()
                            )));
                        }
                    },
                    _ => {
                        return Err(RustHtmlError::from_string(format!(
                            "Unexpected token: {:?}",
                            last
                        )));
                    }
                }
            } else {
                break;
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
        Ok(if ctx.get_add_inner() {
            if ctx.get_only_add_inner().unwrap_or(false) {
                output.clear();
            }
            output.extend_from_slice(&output_inner);
        } else {
            // add ending token
            if let Some(ending_token) = output_inner.last() {
                if let RustHtmlToken::HtmlTagEnd(x, a) = ending_token {
                    output.push(ending_token.clone())
                }
            }
        })
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
        let ctx = Rc::new(HtmlTagParseContext::new(Some(context.clone())));

        // tag name
        // check that it is a closing tag?
        let is_closing_tag = self.check_next_char('/', true, input.clone(), context.clone())?;
        let tag_name = self.parse_tag_name(input.clone(), ctx.clone(), ct.clone())?;
        ctx.set_is_opening_tag(!is_closing_tag);

        // should be tag open, inner elements, tag close
        let mut output = vec![];

        let tag_name_output_token = ctx.on_html_tag_name_parsed(tag_name)?;
        output.push(tag_name_output_token);

        let continue_result = if !is_closing_tag {
            self.parse_tag_start_close(&input, &context, &ct, &ctx, &mut output)?;
        
            println!("parsing start tag {}, is_closing_tag={}, is_self_contained={}", ctx.tag_name_as_str(), is_closing_tag, ctx.is_self_contained_tag());

            // assert next punct is >
            self.check_next_char('>', false, input.clone(), ctx.get_main_context())?;
            
            // tag children
            if !ctx.is_void_tag() && !ctx.is_self_contained_tag() {
                self.parse_tag_inner_contents(input, context, ct, &ctx, &mut output)?;
            }

            RustHtmlDirectiveResult::OkContinue
        } else {
            println!("parsing closing tag {}, is_closing_tag={}, is_self_contained={}", ctx.tag_name_as_str(), is_closing_tag, ctx.is_self_contained_tag());

            // assert next punct is >
            self.check_next_char('>', false, input.clone(), ctx.get_main_context())?;

            RustHtmlDirectiveResult::OkBreak
        };

        if ctx.get_ignore() {
            return Ok(RustHtmlDirectiveResultV3(continue_result, Some(Rc::new(EmptyPeekableRustHtmlToken::new()))));
        } else {
            let tag_stream = Rc::new(VecPeekableRustHtmlToken::new(output));
            Ok(RustHtmlDirectiveResultV3(continue_result, Some(tag_stream)))
        }
    }
    
    fn get_parser(&self) -> Rc<dyn IParserV3> {
        self.parser.borrow().as_ref().unwrap().clone()
    }
    
    fn on_html_node_parsed(&self, context: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
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
    
    fn parse_tag_name(&self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<Vec<RustHtmlIdentOrPunct>, RustHtmlError> {
        // just ident and -
        let mut output = vec![];

        // check for ! which is valid
        if let Some(RustHtmlToken::ReservedChar(c, punct)) = input.peek() {
            if c == '!' {
                output.push(RustHtmlIdentOrPunct::Punct(punct));
                input.next();
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
    
    fn parse_tag_attribs(&self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(), RustHtmlError> {
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

                    // println!("key = {}", attrib_key_string);
    
                    // check for =
                    if let Some(equals_token) = input.peek() {
                        if let RustHtmlToken::ReservedChar(c, p) = &equals_token {
                            if *c == '=' {
                                // proceed
                                input.next();

                                // println!("found equals");

                                let attrib_value = self.parse_attrib_value(input.clone(), ctx.clone(), ct.clone())?;
                                // let attrib_value = self.get_parser().get_rust_parser().parse_string_with_quotes(false, &identifier, input.clone())?;

                                // values must be able to handle string literal or directive value
                                // let attrib_value_group = if attrib_value.0.len() == 1 {
                                //     attrib_value.0.first().unwrap().clone()
                                // } else {
                                //     let attrib_value_stream = Rc::new(VecPeekableRustHtmlToken::new(attrib_value.0));
                                //     RustHtmlToken::Group(proc_macro2::Delimiter::Parenthesis, attrib_value_stream, None)
                                // };
                                // println!("value: {:?}", attrib_value_group);
                                ctx.html_attrs_insert(Some(attrib_key.0), Some(RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(attrib_key.1)), attrib_key_string, 
                                    Some(equals_token.clone()), Some(p.clone()),
                                    None, None, Some(attrib_value.0), Some(RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(attrib_value.1)));
                            } else if *c == '>' {
                                ctx.html_attrs_insert(Some(attrib_key.0), Some(RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(attrib_key.1)), attrib_key_string, None, None, None, None, None, None);
                                break;
                            } else {
                                return Err(RustHtmlError::from_string(format!("parse_tag_attribs unexpected char {}", c)));
                            }
                        } else {
                            return Err(RustHtmlError::from_string(format!("parse_tag_attribs unexpected token {:?}", equals_token)));
                        }
                    } else {
                        ctx.html_attrs_insert(Some(attrib_key.0), Some(RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(attrib_key.1)), attrib_key_string, None, None, None, None, None, None);
                        break;
                    }
                },
                RustHtmlToken::ReservedChar(c, p) => {
                    if c == '/' || c == '>' {
                        // end of tag
                        break;
                    } else if c == '!' {
                        panic!("idk")
                    } else {
                        return Err(RustHtmlError::from_string(format!("unexpected char in parse_tag_attribs: {}", c)));
                    }
                },
                _ => return Err(RustHtmlError::from_string(format!("unexpected token in parse_tag_attribs: {}", token.to_string()))),
            }
        }

        Ok(())
    }
    
    fn parse_xml_ident(&self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(Vec<RustHtmlToken>, Vec<RustHtmlIdentOrPunct>), RustHtmlError> {
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

        if output_tokens.is_empty() {
            panic!("could not parse xml ident (next token is {:?})", input.peek())
        }
        
        Ok((output_tokens, output_typed))
    }
    
    fn parse_literal_or_xml_name(&self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(Vec<RustHtmlToken>, Vec<RustHtmlIdentOrPunct>), RustHtmlError> {
        if let Some(token) = input.peek() {
            match &token {
                RustHtmlToken::Literal(s, l) => {
                    input.next();
                    Ok((vec![token.clone()], vec![]))
                },
                RustHtmlToken::Identifier(i) => self.parse_xml_ident(input, ctx, ct),
                _ => Err(RustHtmlError::from_string(format!("Expected literal or ident, not {}", token.to_string())))
            }
        } else {
            Err(RustHtmlError::from_str("Expected literal or ident, at end of stream"))
        }
    }
    
    fn parse_attrib_value(&self, input: Rc<dyn IPeekableRustHtmlToken>, ctx: Rc<dyn IHtmlTagParseContext>, ct: Rc<dyn ICancellationToken>) -> Result<(Vec<RustHtmlToken>, Vec<RustHtmlIdentOrPunct>), RustHtmlError> {
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