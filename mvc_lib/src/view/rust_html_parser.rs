// use std::any::{Any, TypeId};
// use std::collections::HashMap;
// use std::cell::RefCell;
// use std::cmp::{min, max};
// use std::error::Error;
// use std::fs::File;
// use std::io::BufRead;
// use std::result::Result;
// use std::rc::Rc;
// use std::sync::{Arc, RwLock};
// use std::vec::Vec;

// use glob::glob;

// use rusthtml::rusthtml_error::RustHtmlError;

// use crate::view::html_string::HtmlString;
// use crate::view::iview::IView;
// use crate::contexts::view_context::IViewContext;
// use crate::contexts::controller_context::IControllerContext;

// use crate::services::service_collection::IServiceCollection;

// const _ReservedKeywords: [&'static str;3] = ["for", "in", "let"];

// impl CompiledRustHtmlTemplateNode {
//     // pub fn render_part(self: &Self, ctx: Arc<RwLock<dyn IViewContext>>, services: Arc<RwLock<dyn IServiceCollection>>) -> Result<Box<HtmlString>, Box<dyn Error + 'static>> {
//     //     // println!("Rendering {:?}: {}", self.node_type, self.part_data);
//     //     let rendered_string = match self.node_type {
//     //         CompiledRustHtmlTemplateNodeType::ViewRoot |
//     //         CompiledRustHtmlTemplateNodeType::RustHtml => {
//     //             let rendered_parts = self.inner_parts
//     //                 .iter()
//     //                 .map(|x| *x.render_part(ctx.clone(), services.clone()).unwrap())
//     //                 .collect::<Vec<HtmlString>>();

//     //             let mut html_content = String::new();
//     //             for part in rendered_parts {
//     //                 html_content.push_str(&part.content.as_str());
//     //             }
//     //             Ok(Box::new(HtmlString::new_from_html(html_content)))
//     //         },
//     //         _ => Err(Box::new(RustHtmlError("Invalid compiled template data part type")) as Box<dyn Error>)
//     //     };
//     //     //println!("Rendered {:?}: {} -> {}", self.node_type, self.part_data.chars().take(20).collect::<String>(), rendered_string.as_ref().unwrap().content.chars().take(20).collect::<String>());
//     //     Ok(rendered_string?)
//     // }

//     pub fn render_body(view_ctx: Arc<RwLock<dyn IViewContext>>, services: Arc<RwLock<dyn IServiceCollection>>) -> Result<Box<HtmlString>, Box<dyn Error + 'static>> {
//         let body_view_option = view_ctx.read().unwrap().get_controller_ctx().borrow().get_view_data_value("Body");
//         match body_view_option {
//             Some(body_view_any) => {
//                 let body_view = body_view_any.as_ref().downcast_ref::<Rc<Box<dyn IView>>>().expect("could not downcast Any to Box<dyn IView>");
//                 // need new context for child view
//                 let new_ctx = view_ctx.read().unwrap().recurse_into_new_context(body_view.clone());
//                 let view_renderer = new_ctx.read().unwrap();
//                 view_renderer.get_view_renderer().render_view(new_ctx.clone(), services.clone())
//             },
//             None => panic!("Body view not found")
//         }
//     }

//     pub fn render_body_write_to_response(view_ctx: Arc<RwLock<dyn IViewContext>>, services: Arc<RwLock<dyn IServiceCollection>>) -> Result<eval::Value, eval::Error> {
//         // let html = Self::render_body(view_ctx.clone(), services)?;
//         // view_ctx.read().unwrap().get_response_ctx().as_ref().borrow_mut().body.extend_from_slice(html.content.as_bytes());
//         // return Ok("");
//         let html = Self::render_body(view_ctx, services).unwrap().content;
//         let mut as_object = HashMap::<String, eval::Value>::new();
//         as_object.insert("html".to_string(), eval::Value::String(html));
//         Ok(eval::to_value(as_object))
//     }
// }

// pub struct RustHtmlParseContext {
//     pub model_type_name: Rc<RefCell<Option<String>>>,
// }

// impl RustHtmlParseContext {
//     pub fn new(outer: &RustHtmlParseContext) -> Self {
//         Self {
//             model_type_name: outer.model_type_name.clone(),
//         }
//     }

//     pub fn new_root() -> Self {
//         Self {
//             model_type_name: Rc::new(RefCell::new(None)),
//         }
//     }
// }

// pub trait RustHtmlPartParser {
//     fn parse_part(self: &Self, data: &String, parent_parser: Option<&dyn RustHtmlPartParser>, parse_ctx: &mut RustHtmlParseContext) -> Result<Box<CompiledRustHtmlTemplateNode>, Box<dyn Error>>;
//     fn peek_end_expression(self: &Self, data: &String, current_parser: &dyn RustHtmlPartParser, parse_ctx: &mut RustHtmlParseContext) -> bool;
// }

// pub struct CommentPartParser {

// }
// impl CommentPartParser {
//     pub fn new() -> Self {
//         Self {}
//     }
// }
// impl RustHtmlPartParser for CommentPartParser {
//     fn parse_part(self: &Self, data: &String, parent_parser: Option<&dyn RustHtmlPartParser>, parse_ctx: &mut RustHtmlParseContext) -> Result<Box<CompiledRustHtmlTemplateNode>, Box<dyn Error>> {
//         //Ok(Box::new(CompiledRustHtmlTemplateNode::new(&data, CompiledRustHtmlTemplateNodeType::StaticHtmlString, vec![])))
//         RustParserHelpers::parse_rust_code(data, false, Some(self), parse_ctx)
//     }
//     fn peek_end_expression(self: &Self, data: &String, current_parser: &dyn RustHtmlPartParser, parse_ctx: &mut RustHtmlParseContext) -> bool {
//         let ch = data.chars().nth(*parse_ctx.i.borrow()).unwrap();
//         return ch == '\n';
//     }
// }

// pub struct RustFunctionCallPartParser {}
// impl RustFunctionCallPartParser {
//     pub fn new() -> Self {
//         Self {}
//     }
// }
// impl RustHtmlPartParser for RustFunctionCallPartParser {
//     fn parse_part(self: &Self, data: &String, parent_parser: Option<&dyn RustHtmlPartParser>, parse_ctx: &mut RustHtmlParseContext) -> Result<Box<CompiledRustHtmlTemplateNode>, Box<dyn Error>> {
//         let start_i = *parse_ctx.i.borrow();

//         // opening '('
//         let start_token = rustc_lexer::first_token(&data[start_i..]);
//         if start_token.kind != rustc_lexer::TokenKind::OpenParen {
//             panic!("Unexpected token '{:?}'", start_token.kind);
//         }
//         *parse_ctx.i.borrow_mut() += start_token.len;

//         let mut var_parts = Vec::new();
//         let mut brace_stack = Vec::<rustc_lexer::TokenKind>::new();
//         let mut expecting_comma_or_close_paren = false;
//         loop {
//             let token_i = *parse_ctx.i.borrow();
//             let token = rustc_lexer::first_token(&data[token_i..]);
//             *parse_ctx.i.borrow_mut() += token.len;
//             let token_str = &data[token_i..token_i+token.len];
//             println!("rust_function_call token_str: '{}' ({} chars)", token_str, token.len);

//             let mut ignore = false;
//             match token.kind {
//                 rustc_lexer::TokenKind::Whitespace => {
//                     ignore = true;
//                 },
//                 rustc_lexer::TokenKind::LineComment => {
//                     ignore = true;
//                 },
//                 rustc_lexer::TokenKind::BlockComment { .. } => {
//                     ignore = true;
//                 },
//                 rustc_lexer::TokenKind::Comma { .. } => {
//                     if expecting_comma_or_close_paren {
//                         expecting_comma_or_close_paren = false;
//                         ignore = true;
//                     } else {
//                         panic!("Unexpected comma");
//                     }
//                 },
//                 rustc_lexer::TokenKind::CloseParen => {
//                     if expecting_comma_or_close_paren {
//                         break;
//                     } else {
//                         panic!("Unexpected close paren");
//                     }
//                 },
//                 rustc_lexer::TokenKind::Literal { kind, suffix_start } => {
//                     ignore = true;
//                     expecting_comma_or_close_paren = true;
//                     // kind has other type LiteralKind
//                     // might have to add as option to CompiledRustHtmlTemplateNode
//                     // expected enum `TokenKind`, found enum `LiteralKind`
//                     var_parts.push(Box::new(CompiledRustHtmlTemplateNode::new_literal(token_i, token.len, kind, &token_str.to_string())));
//                 },
//                 _ => {
//                     panic!("rust_function_call Unexpected token '{}' ({:?})", token_str, token.kind);
//                 }
//             }

//             if ignore == false {
//                 // add parameter or token between parameters
//                 var_parts.push(Box::new(CompiledRustHtmlTemplateNode::new(token_i, token.len, CompiledRustHtmlTemplateNodeType::RustCompileTimeVariable, Some(token.kind), &token_str.to_string(), HashMap::new(), vec![])));
//             }
//         }

//         let content = &data[start_i..*parse_ctx.i.borrow()];
//         println!("rust_function_call parsed '{}'", content);
//         return Ok(Box::new(CompiledRustHtmlTemplateNode::new(start_i, content.len(), CompiledRustHtmlTemplateNodeType::RustFunctionCall, None, &content.to_string(), HashMap::new(), var_parts)));
//     }

//     fn peek_end_expression(self: &Self, data: &String, current_parser: &dyn RustHtmlPartParser, parse_ctx: &mut RustHtmlParseContext) -> bool {
//         let ch = data.chars().nth(*parse_ctx.i.borrow()).unwrap();
//         return ch == ')' || ch == ',';
//     }
// }

// pub struct RustCompileTimeVariablePartParser {}
// impl RustCompileTimeVariablePartParser {
//     pub fn new() -> Self {
//         Self {}
//     }
// }
// impl RustHtmlPartParser for RustCompileTimeVariablePartParser {
//     fn parse_part(self: &Self, data: &String, parent_parser: Option<&dyn RustHtmlPartParser>, parse_ctx: &mut RustHtmlParseContext) -> Result<Box<CompiledRustHtmlTemplateNode>, Box<dyn Error>> {
//         let start_i = *parse_ctx.i.borrow();

//         let mut var_parts = Vec::new();
//         let mut brace_stack = Vec::new();
//         loop {
//             if parent_parser.unwrap().peek_end_expression(data, self, parse_ctx) {
//                 break;
//             }

//             let token_i = *parse_ctx.i.borrow();
//             let token = rustc_lexer::first_token(&data[token_i..]);
//             *parse_ctx.i.borrow_mut() += token.len;
//             let token_str = &data[token_i..token_i+token.len];
//             println!("rust_compile_time_variable token_str: '{}' ({} chars)", token_str, token.len);

//             let mut ignore = false;
//             match token.kind {
//                 rustc_lexer::TokenKind::Whitespace => {
//                     ignore = true;
//                     if !*parse_ctx.is_block_expression.borrow() {
//                         println!("rust_compile_time_variable hit whitespace");
//                         *parse_ctx.i.borrow_mut() -= token.len;
//                         break;
//                     }
//                 },
//                 rustc_lexer::TokenKind::LineComment => {
//                     ignore = true;
//                 },
//                 rustc_lexer::TokenKind::BlockComment { .. } => {
//                     ignore = true;
//                 },
//                 rustc_lexer::TokenKind::Ident |
//                 rustc_lexer::TokenKind::Literal { .. } |
//                 rustc_lexer::TokenKind::Dot => {
//                     ignore = false;
//                 },
//                 rustc_lexer::TokenKind::OpenBrace |
//                 rustc_lexer::TokenKind::OpenBracket => {
//                     brace_stack.push(token.kind);
//                     ignore = false;
//                 },
//                 rustc_lexer::TokenKind::CloseBrace |
//                 rustc_lexer::TokenKind::CloseBracket => {
//                     ignore = false;

//                     let matching_brace = brace_stack.pop().unwrap();
//                     let expected_brace = match matching_brace {
//                         rustc_lexer::TokenKind::OpenBrace => rustc_lexer::TokenKind::CloseBrace,
//                         rustc_lexer::TokenKind::OpenBracket => rustc_lexer::TokenKind::CloseBracket,
//                         _ => panic!("Unexpected token {:?}", matching_brace)
//                     };

//                     if expected_brace != token.kind {
//                         panic!("Mismatched brace or bracket (expected {:?}, found {:?})", expected_brace, token.kind);
//                     }
//                 },
//                 rustc_lexer::TokenKind::OpenParen => {
//                     // starting function call
//                     // gather arguments into inner_parts for RustFunctionCall
//                     // if it ends with ';' then we know it is not returning a value to the caller
//                     *parse_ctx.i.borrow_mut() -= token.len; // backup
//                     var_parts.push(RustFunctionCallPartParser::new().parse_part(data, Some(self), parse_ctx)?);
//                 },
//                 rustc_lexer::TokenKind::Semi => {
//                     // end function call
//                     // does not return value unless the line started with a return, in which we'd already be in the return context.
//                     *parse_ctx.i.borrow_mut() -= token.len; // backup
//                     var_parts.push(Box::new(CompiledRustHtmlTemplateNode::new(token_i, token.len, CompiledRustHtmlTemplateNodeType::RustCompileTimeVariable, Some(token.kind), &token_str.to_string(), HashMap::new(), vec![])));
//                     break;
//                 },
//                 _ => {
//                     panic!("rust_compile_time_variable Unexpected token '{}' ({:?})", token_str, token.kind)
//                 }
//             }

//             if ignore == false {
//                 var_parts.push(Box::new(CompiledRustHtmlTemplateNode::new(token_i, token.len, CompiledRustHtmlTemplateNodeType::RustCompileTimeVariable, Some(token.kind), &token_str.to_string(), HashMap::new(), vec![])));
//             }
//         }

//         let content = &data[start_i..*parse_ctx.i.borrow()];
//         println!("rust_compile_time_var parsed: '{}'", content);
//         return Ok(Box::new(CompiledRustHtmlTemplateNode::new(start_i, content.len(), CompiledRustHtmlTemplateNodeType::RustCompileTimeVariable, None, &content.to_string(), HashMap::new(), var_parts)));
//     }

//     fn peek_end_expression(self: &Self, data: &String, current_parser: &dyn RustHtmlPartParser, parse_ctx: &mut RustHtmlParseContext) -> bool {
//         return false;
//     }
// }

// pub struct ModelDirectivePartParser {
//     //pub keyword_str: Box<String>,
// }
// impl ModelDirectivePartParser {
//     pub fn new() -> Self {
//         Self {}
//     }
// }
// impl RustHtmlPartParser for ModelDirectivePartParser {
//     fn parse_part(self: &Self, data: &String, parent_parser: Option<&dyn RustHtmlPartParser>, parse_ctx: &mut RustHtmlParseContext) -> Result<Box<CompiledRustHtmlTemplateNode>, Box<dyn Error>> {
//         let current_type_name_option = parse_ctx.model_type_name.borrow().clone();
//         match current_type_name_option {
//             Some(mut current_type_name) => {
//                 return Err(Box::new(RustHtmlError("already defined model type")) as Box<dyn Error>);
//             }
//             None => {
//                 let mut type_name = String::new();
//                 let start_i = *parse_ctx.i.borrow();

//                 loop {
//                     let token_i = *parse_ctx.i.borrow();
//                     let try_token_ch = data.chars().nth(token_i);
//                     match try_token_ch {
//                         Some(token_ch) => {
//                             if token_ch == '\n' {
//                                 *parse_ctx.i.borrow_mut() += 1;
//                                 *parse_ctx.line_no.borrow_mut() += 1;
//                                 *parse_ctx.col_no.borrow_mut() = 0;
//                                 println!("model directive hit newline");
//                                 break;
//                             }
//                         },
//                         None => {
//                             break;
//                         }
//                     }

//                     let token = rustc_lexer::first_token(&data[token_i..]);
//                     if token.len == 0 {
//                         panic!("token.len == 0");
//                     }
//                     *parse_ctx.i.borrow_mut() += token.len;
//                     let token_str = &data[token_i..token_i+token.len];
//                     println!("model directive token_str: '{}' ({} chars)", token_str, token.len);

//                     match token.kind {
//                         rustc_lexer::TokenKind::Whitespace |
//                         rustc_lexer::TokenKind::Colon => {
//                             continue;
//                         },
//                         rustc_lexer::TokenKind::Ident => {
//                             if token_i - start_i == 0 {
//                                 // first token should me "model", the directive name
//                                 if token_str == "model" {
//                                     continue;
//                                 } else {
//                                     panic!("Unexpected token in model directive statement: '{}'", token_str);
//                                 }
//                             }
//                             type_name.push_str(token_str);
//                         },
//                         _ => {
//                             panic!("Unexpected token in model directive statement: '{}'", token_str);
//                         }
//                     }
//                 }
                
//                 println!("model directive parsed type name: '{}'", type_name);
//                 parse_ctx.model_type_name.replace(Some(type_name));

//                 let content = &data[start_i..*parse_ctx.i.borrow()];
//                 return Ok(Box::new(CompiledRustHtmlTemplateNode::new(start_i, content.len(), CompiledRustHtmlTemplateNodeType::Directive, None, &content.to_string(), HashMap::new(), vec![])));
//             }
//         }
//     }

//     fn peek_end_expression(self: &Self, data: &String, current_parser: &dyn RustHtmlPartParser, parse_ctx: &mut RustHtmlParseContext) -> bool {
//         return false;
//     }
// }

// pub struct RustForKeywordPartParser {
//     //pub keyword_str: Box<String>,
// }
// impl RustForKeywordPartParser {
//     pub fn new() -> Self {
//         Self {}
//     }

//     pub fn expect_char_and_increment(self: &Self, c: char, data: &String, parse_ctx: &mut RustHtmlParseContext) -> Result<(), Box<dyn Error>> {
//         let c_i = *parse_ctx.i.borrow();
//         let actual_c = data.chars().nth(c_i).expect("Could not get next character");
//         if c != actual_c {
//             return Err(Box::new(RustHtmlError("Unexpected char")));
//         }
//         *parse_ctx.i.borrow_mut() += 1;
//         Ok(())
//     }
// }
// impl RustHtmlPartParser for RustForKeywordPartParser {
//     fn parse_part(self: &Self, data: &String, parent_parser: Option<&dyn RustHtmlPartParser>, parse_ctx: &mut RustHtmlParseContext) -> Result<Box<CompiledRustHtmlTemplateNode>, Box<dyn Error>> {
//         let start_i = *parse_ctx.i.borrow();
//         let mut expecting_item_name = true;
//         let mut expecting_for_type = true;
//         let mut expecting_open_brace = true;
//         let mut expecting_close_brace = true;
//         let mut item_name = String::new();
//         let mut node_attributes = HashMap::new();
//         let mut node_children = Vec::new();

//         //todo: need to skip "for" since the below code does not expect it and looks for the item name first
//         self.expect_char_and_increment('f', data, parse_ctx)?;
//         self.expect_char_and_increment('o', data, parse_ctx)?;
//         self.expect_char_and_increment('r', data, parse_ctx)?;

//         loop {
//             let token_i = *parse_ctx.i.borrow();
//             let token = rustc_lexer::first_token(&data[token_i..]);
//             *parse_ctx.i.borrow_mut() += token.len;
//             let token_str = &data[token_i..token_i+token.len];
//             println!("rust_for token_str: '{}' ({} chars)", token_str, token.len);

//             let mut ignore = false;
//             match token.kind {
//                 rustc_lexer::TokenKind::Whitespace => {
//                     ignore = true;
//                 },
//                 rustc_lexer::TokenKind::LineComment => {
//                     ignore = true;
//                 },
//                 rustc_lexer::TokenKind::BlockComment { .. } => {
//                     ignore = true;
//                 },
//                 rustc_lexer::TokenKind::Ident => {
//                     if expecting_item_name {
//                         item_name = token_str.to_string();
//                         println!("item_name: {}", item_name);
//                         node_attributes.insert("item_name".to_string(), Some(Box::new(CompiledRustHtmlTemplateNode::new_literal(token_i, token.len, rustc_lexer::LiteralKind::Str { terminated: true }, &token_str.to_string()))));
//                         expecting_item_name = false;
//                     } else if expecting_for_type {
//                         if token_str == "in" {
//                             // ok
//                             expecting_for_type = false;
//                         } else {
//                             panic!("Unexpected token in for loop definition: '{}'", token_str);
//                         }
//                     } else if !node_attributes.contains_key("item_source") {
//                         // backup, rewind
//                         *parse_ctx.i.borrow_mut() -= token.len;
//                         let for_source = RustCompileTimeVariablePartParser::new().parse_part(data, Some(self), parse_ctx)?;
//                         node_attributes.insert("item_source".to_string(), Some(for_source));
//                     } else {
//                         panic!("Already defined item_source in for loop definition")
//                     }
//                 },
//                 rustc_lexer::TokenKind::CloseBrace => {
//                     if !expecting_item_name && !expecting_for_type && !expecting_open_brace && expecting_close_brace {
//                         expecting_close_brace = false;
//                         break;
//                     } else {
//                         panic!("rust_for Premature token '{}'", token_str);
//                     }
//                 },
//                 rustc_lexer::TokenKind::OpenBrace => {
//                     if !expecting_item_name && !expecting_for_type && expecting_open_brace {
//                         // get inner operations
//                         let mut inner_parse_ctx = RustHtmlParseContext::new(parse_ctx);
//                         // inner_parse_ctx.expecting_for_loop.borrow_mut() = true;
//                         *inner_parse_ctx.is_rust_expression.borrow_mut() = true;
//                         *inner_parse_ctx.is_block_expression.borrow_mut() = true;

//                         // new context
//                         let inner_content = RustBlockPartParser::new().parse_part(data, Some(self), &mut inner_parse_ctx)?;
//                         node_children.push(inner_content);

//                         expecting_open_brace = false;
//                         break;
//                     } else {
//                         panic!("rust_for Premature token '{}'", token_str);
//                     }
//                 },
//                 _ => {
//                     panic!("rust_for Unexpected token '{}'", token_str);
//                 }
//             }
//         }
//         let content = data[start_i..*parse_ctx.i.borrow()].to_string();
//         println!("rust_for parsed: {}", content);
//         return Ok(Box::new(CompiledRustHtmlTemplateNode::new(start_i, content.len(), CompiledRustHtmlTemplateNodeType::Keyword, None, &content, node_attributes, node_children)));
//     }

//     fn peek_end_expression(self: &Self, data: &String, current_parser: &dyn RustHtmlPartParser, parse_ctx: &mut RustHtmlParseContext) -> bool {
//         return false;
//     }
// }

// pub struct RustParserHelpers {}
// impl RustParserHelpers {
//     pub fn parse_start_of_rust_directive_keyword_or_identifier(ident: &str, data: &String, current_parser: Option<&dyn RustHtmlPartParser>, parse_ctx: &mut RustHtmlParseContext) -> Result<Box<CompiledRustHtmlTemplateNode>, Box<dyn Error>> {
//         let start_i = *parse_ctx.i.borrow();
//         println!("parse_start_of_rust_directive_keyword_or_identifier: '{}'", ident);
//         let mut compiled_template_data_type: Option<CompiledRustHtmlTemplateNodeType> = None;

//         match ident {
//             "model" => {
//                 return ModelDirectivePartParser::new().parse_part(data, current_parser, parse_ctx);
//             },
//             "functions" => {
//                 // https://learn.microsoft.com/en-us/aspnet/core/mvc/views/razor?view=aspnetcore-7.0#functions
//             },
//             "implements" => {
//                 compiled_template_data_type = Some(CompiledRustHtmlTemplateNodeType::Directive);
//                 // https://learn.microsoft.com/en-us/aspnet/core/mvc/views/razor?view=aspnetcore-7.0#implements
//             },
//             "inject" => {
//                 compiled_template_data_type = Some(CompiledRustHtmlTemplateNodeType::Directive);
//                 // https://learn.microsoft.com/en-us/aspnet/core/mvc/views/razor?view=aspnetcore-7.0#inject
//             },
//             "inherits" => {
//                 compiled_template_data_type = Some(CompiledRustHtmlTemplateNodeType::Directive);
//                 // https://learn.microsoft.com/en-us/aspnet/core/mvc/views/razor?view=aspnetcore-7.0#inject
//             },
//             "section" => {
//                 // https://learn.microsoft.com/en-us/aspnet/core/mvc/views/razor?view=aspnetcore-7.0#section
//             },
//             "using" => {
//                 compiled_template_data_type = Some(CompiledRustHtmlTemplateNodeType::Directive);
//                 // https://learn.microsoft.com/en-us/aspnet/core/mvc/views/razor?view=aspnetcore-7.0#using
//             },
//             "for" => {
//                 return RustForKeywordPartParser::new().parse_part(data, current_parser, parse_ctx);
//             },
//             "ViewData" |
//             "view_data" => {
//                 return RustCompileTimeVariablePartParser::new().parse_part(data, current_parser, parse_ctx);
//             }
//             _ => {
//                 // assume it is referencing an identifier

//             }
//         }
//         panic!("Could not resolve keyword or identifier '{}'", ident);
//     }

//     pub fn parse_rust_code(data: &String, is_block_expression: bool, self_parser: Option<&dyn RustHtmlPartParser>, outer_parse_ctx: &mut RustHtmlParseContext) -> Result<Box<CompiledRustHtmlTemplateNode>, Box<dyn Error>> {
//         let mut parse_ctx = RustHtmlParseContext::new(outer_parse_ctx);
//         *parse_ctx.is_block_expression.borrow_mut() = is_block_expression;

//         // println!("parse_rust_code is_block_expression = {}", is_block_expression);

//         let start_i = *parse_ctx.i.borrow();
//         let mut expression_type = CompiledRustHtmlTemplateNodeType::RustExpression;

//         let mut brace_stack = Vec::new();
//         if is_block_expression {
//             // opening '('
//             let start_token = rustc_lexer::first_token(&data[start_i..]);
//             if start_token.kind != rustc_lexer::TokenKind::OpenBrace {
//                 panic!("Unexpected token '{:?}'", start_token.kind);
//             }
//             *parse_ctx.i.borrow_mut() += start_token.len;

//             brace_stack.push(rustc_lexer::TokenKind::OpenBrace);
//         }
//         let mut tokensToIncludeInStatement = Vec::new();
//         let mut inner_parts = Vec::new();
//         loop {
//             let token_i = *parse_ctx.i.borrow();
//             let try_token_ch = data.chars().nth(token_i);
//             match try_token_ch {
//                 Some(token_ch) => {
//                     if token_ch == '\n' {
//                         *parse_ctx.i.borrow_mut() += 1;
//                         *parse_ctx.line_no.borrow_mut() += 1;
//                         *parse_ctx.col_no.borrow_mut() = 0;
//                         println!("parse_rust_code hit newline {}", is_block_expression);
//                         continue;
//                     }

//                     let token_slice = &data[token_i..];
//                     let token = rustc_lexer::first_token(token_slice);

//                     *parse_ctx.i.borrow_mut() += token.len;
//                     let token_str = &data[token_i..token_i+token.len];
//                     println!("parse_rust_code token_str: '{}' ({} chars)", token_str, token.len);

//                     let mut ignore = false;
//                     match token.kind {
//                         rustc_lexer::TokenKind::Whitespace => {
//                             ignore = true;
//                             if !*parse_ctx.is_block_expression.borrow() {
//                                 println!("hit whitespace");
//                                 *parse_ctx.i.borrow_mut() -= token.len;
//                                 break;
//                             }
//                         },
//                         rustc_lexer::TokenKind::LineComment => {
//                             ignore = true;
//                         },
//                         rustc_lexer::TokenKind::BlockComment { .. } => {
//                             ignore = true;
//                         },
//                         rustc_lexer::TokenKind::Ident => {
//                             let ident = Self::parse_start_of_rust_directive_keyword_or_identifier(token_str, data, self_parser, &mut parse_ctx)?;
//                             if ident.node_type == CompiledRustHtmlTemplateNodeType::Directive {
//                                 return Ok(ident);
//                             } else {
//                                 inner_parts.push(ident);
//                             }
//                         },
//                         rustc_lexer::TokenKind::OpenBrace |
//                         rustc_lexer::TokenKind::OpenBracket => {
//                             brace_stack.push(token.kind);
//                         },
//                         rustc_lexer::TokenKind::CloseBrace |
//                         rustc_lexer::TokenKind::CloseBracket => {
//                             let matching_brace = brace_stack.pop().unwrap();
//                             let expected_brace = match matching_brace {
//                                 rustc_lexer::TokenKind::OpenBrace => rustc_lexer::TokenKind::CloseBrace,
//                                 rustc_lexer::TokenKind::OpenBracket => rustc_lexer::TokenKind::CloseBracket,
//                                 _ => panic!("Unexpected token {:?}", matching_brace)
//                             };

//                             if expected_brace != token.kind {
//                                 panic!("Mismatched brace or bracket (expected {:?}, found {:?})", expected_brace, token.kind);
//                             }

//                             if *parse_ctx.is_block_expression.borrow() && brace_stack.len() == 0 {
//                                 println!("last closing brace, exiting");
//                                 // *parse_ctx.i.borrow_mut() -= 1;
//                                 break;
//                             } else {
//                                 println!("not the last brace ({} remaining)", brace_stack.len());
//                             }
//                         },
//                         rustc_lexer::TokenKind::Lt => {
//                             let next_token = rustc_lexer::first_token(&data[*parse_ctx.i.borrow()..]);
//                             match next_token.kind {
//                                 rustc_lexer::TokenKind::Slash => {
//                                     // exit, this is the end of some HTML </
//                                     break;
//                                 },
//                                 rustc_lexer::TokenKind::Ident => {
//                                     // pause, this is the start of some HTML <a-z+
//                                     *parse_ctx.i.borrow_mut() -= token.len;

//                                     println!("Parsing HTML inside rust");
//                                     let part = HtmlPartParser::new().parse_part(data, self_parser, &mut parse_ctx)?;
//                                     println!("Exit Parsing HTML inside rust: {}", part.token_content);
//                                     inner_parts.push(part)
//                                     //println!("num_read_html: {}", num_read_html?);
//                                     //num_read += num_read_html;
//                                 },
//                                 rustc_lexer::TokenKind::Lt => {
//                                     // start of comment??
//                                     panic!("confused about // in expression {}", data);
//                                 }
//                                 _ => {
//                                     panic!("confused about expression {}", data);
//                                 }
//                             }
//                         },
//                         _ => {

//                         }
//                     }

//                     if !ignore {
//                         tokensToIncludeInStatement.push((token.kind, token_i, token.len));
//                         // println!("token: {:?} ({})", token.kind, token.len);
//                     }
//                 },
//                 None => {
//                     break;
//                 }
//             }
//         }

//         // println!("num_read: {}", num_read);
//         let template_part = &data[start_i..*parse_ctx.i.borrow()].to_string();
//         println!("parsed rust {:?}: {}", expression_type, template_part);
//         return Ok(Box::new(CompiledRustHtmlTemplateNode::new(start_i, template_part.len(), expression_type, None, template_part, HashMap::new(), inner_parts)));
//     }
// }

// pub struct RustExpressionInlinePartParser {

// }
// impl RustExpressionInlinePartParser {
//     pub fn new() -> Self {
//         Self {}
//     }
// }
// impl RustHtmlPartParser for RustExpressionInlinePartParser {
//     fn parse_part(self: &Self, data: &String, parent_parser: Option<&dyn RustHtmlPartParser>, parse_ctx: &mut RustHtmlParseContext) -> Result<Box<CompiledRustHtmlTemplateNode>, Box<dyn Error>> {
//         //Ok(Box::new(CompiledRustHtmlTemplateNode::new(&data, CompiledRustHtmlTemplateNodeType::StaticHtmlString, vec![])))
//         panic!("todo implement me");
//         RustParserHelpers::parse_rust_code(data, false, Some(self), parse_ctx)
//     }
//     fn peek_end_expression(self: &Self, data: &String, current_parser: &dyn RustHtmlPartParser, parse_ctx: &mut RustHtmlParseContext) -> bool {
//         let ch = data.chars().nth(*parse_ctx.i.borrow()).unwrap();
//         return ch == ' ';
//     }
// }

// pub struct RustBlockPartParser {}
// impl RustBlockPartParser {
//     pub fn new() -> Self {
//         Self {}
//     }
// }
// impl RustHtmlPartParser for RustBlockPartParser {
//     fn parse_part(self: &Self, data: &String, parent_parser: Option<&dyn RustHtmlPartParser>, parse_ctx: &mut RustHtmlParseContext) -> Result<Box<CompiledRustHtmlTemplateNode>, Box<dyn Error>> {
//         let start_i = *parse_ctx.i.borrow();
        
//         let mut parts = Vec::new();
//         loop {
//             if *parse_ctx.i.borrow() >= data.len() {
//                 break;
//             }
            
//             *parse_ctx.i.borrow_mut() += 1;
//             let c_i = *parse_ctx.i.borrow();
//             let c = data.chars().nth(c_i).unwrap_or('\0');

//             match c {
//                 '{' => {
//                     parts.push(self.parse_part(data, Some(self), parse_ctx)?);
//                     continue;
//                 },
//                 '\n' | ' ' => { continue; }
//                 '}' => {
//                     break;
//                 },
//                 _ => {
//                     let mut inner_parse_ctx = RustHtmlParseContext::new(parse_ctx);
//                     // inner_parse_ctx.expecting_for_loop.borrow_mut() = true;
//                     *inner_parse_ctx.is_rust_expression.borrow_mut() = true;
//                     *inner_parse_ctx.is_block_expression.borrow_mut() = true;
//                     parts.push(HtmlPartParser::new().parse_part(data, Some(self), &mut inner_parse_ctx)?);
//                 }
//             }
//         }

//         let content = &data[start_i..*parse_ctx.i.borrow()];
//         return Ok(Box::new(CompiledRustHtmlTemplateNode::new(start_i, content.len(), CompiledRustHtmlTemplateNodeType::RustBlock, None, &content.to_string(), HashMap::new(), parts)));
//     }
//     fn peek_end_expression(self: &Self, data: &String, current_parser: &dyn RustHtmlPartParser, parse_ctx: &mut RustHtmlParseContext) -> bool {
//         false
//     }
// }

// pub struct RustExpressionPartParser {}
// impl RustExpressionPartParser {
//     pub fn new() -> Self {
//         Self {}
//     }
// }
// impl RustHtmlPartParser for RustExpressionPartParser {
//     fn parse_part(self: &Self, data: &String, parent_parser: Option<&dyn RustHtmlPartParser>, parse_ctx: &mut RustHtmlParseContext) -> Result<Box<CompiledRustHtmlTemplateNode>, Box<dyn Error>> {
//         //Ok(Box::new(CompiledRustHtmlTemplateNode::new(&data, CompiledRustHtmlTemplateNodeType::StaticHtmlString, vec![])))
//         panic!("todo implement me");
//         RustParserHelpers::parse_rust_code(data, false, Some(self), parse_ctx)
//     }
//     fn peek_end_expression(self: &Self, data: &String, current_parser: &dyn RustHtmlPartParser, parse_ctx: &mut RustHtmlParseContext) -> bool {
//         let ch = data.chars().nth(*parse_ctx.i.borrow()).unwrap();
//         return ch == '}';
//     }
// }

// pub struct RustStatementPartParser {

// }
// impl RustStatementPartParser {
//     pub fn new() -> Self {
//         Self {}
//     }
// }
// impl RustHtmlPartParser for RustStatementPartParser {
//     fn parse_part(self: &Self, data: &String, parent_parser: Option<&dyn RustHtmlPartParser>, parse_ctx: &mut RustHtmlParseContext) -> Result<Box<CompiledRustHtmlTemplateNode>, Box<dyn Error>> {
//         let start_i = *parse_ctx.i.borrow();
//         let mut tokens = Vec::new();
//         loop {
//             let c_i = *parse_ctx.i.borrow();
//             let c = data.chars().nth(c_i).unwrap();
//             if c.is_ascii_alphabetic() {
//                 let token = rustc_lexer::first_token(&data[c_i..]);
//                 *parse_ctx.i.borrow_mut() += token.len;
//                 let token_str = &data[c_i..c_i+token.len];

//                 match token.kind {
//                     rustc_lexer::TokenKind::Ident => {
//                         match token_str {
//                             "ViewData" => {
//                                 tokens.push(token_str.to_string());
//                             },
//                             _ => {
//                                 if _ReservedKeywords.contains(&token_str) {
//                                     panic!("Cannot use reserved keyword '{:?}'", token_str);
//                                     //panic!("Unexpected ident token '{:?}' (not implemented)", token_str);
//                                 } else {
//                                     tokens.push(token_str.to_string());
//                                 }
//                             }
//                         }
//                     },
//                     _ => {
//                         panic!("Unexpected token '{:?}' (not implemented)", token.kind);
//                     }
//                 }
//             } else {
//                 *parse_ctx.i.borrow_mut() += 1;
//                 match c {
//                     '<' => {
//                         // starting HTML
//                         panic!("not implemented");
//                     },
//                     '.' => {
//                         tokens.push(c.to_string());
//                     },
//                     '(' | ';' | '}' => {
//                         *parse_ctx.i.borrow_mut() -= 1;
//                         break;
//                     },
//                     _ => {
//                         panic!("Unexpected char '{}' (not implemented)", c);
//                     }
//                 }
//             }
//         }

//         let content = &data[start_i..*parse_ctx.i.borrow()];
//         println!("RustStatementPartParser: '{}'", content);
//         return Ok(Box::new(CompiledRustHtmlTemplateNode::new(start_i, content.len(), CompiledRustHtmlTemplateNodeType::RustCompileTimeVariable, None, &content.to_string(), HashMap::new(), vec![])));
//     }
//     fn peek_end_expression(self: &Self, data: &String, current_parser: &dyn RustHtmlPartParser, parse_ctx: &mut RustHtmlParseContext) -> bool {
//         let ch = data.chars().nth(*parse_ctx.i.borrow()).unwrap();
//         return ch == '}';
//     }
// }


// #[derive(Debug)]
// pub struct ParsedHtmlTag {
//     pub i: usize,
//     pub len: usize,
//     pub content: String,
//     pub tag_name: String,
//     pub attributes: HashMap<String, Option<String>>,
//     pub is_opening_tag: bool, // does not start with </
//     pub is_self_contained_tag: bool, // starts with <a-z and ends with />
// }

// pub struct HtmlPartParser {}
// impl HtmlPartParser {
//     pub fn new() -> Self {
//         Self {}
//     }

//     // pub fn expect_char_and_increment(self: &Self, c: char, data: &String, parse_ctx: &mut RustHtmlParseContext) -> Result<(), Box<dyn Error>> {
//     //     let c_i = *parse_ctx.i.borrow();
//     //     let actual_c = data.chars().nth(c_i).expect("Could not get next character");
//     //     if c != actual_c {
//     //         return Err(Box::new(RustHtmlError("Unexpected char")));
//     //     }
//     //     *parse_ctx.i.borrow_mut() += 1;
//     //     Ok(())
//     // }

//     // pub fn optional_char_and_increment(self: &Self, c: char, data: &String, parse_ctx: &mut RustHtmlParseContext) -> Result<bool, Box<dyn Error>> {
//     //     let c_i = *parse_ctx.i.borrow();
//     //     let actual_c = data.chars().nth(c_i).unwrap_or('\0');
//     //     if c == actual_c {
//     //         //println!("c ({}) == actual_c ({})", c, actual_c);
//     //         *parse_ctx.i.borrow_mut() += 1;
//     //     }
//     //     Ok(c == actual_c)
//     // }

//     // pub fn skip_whitespace(self: &Self, data: &String, parse_ctx: &mut RustHtmlParseContext) {
//     //     loop {
//     //         let c_i = *parse_ctx.i.borrow();
//     //         if c_i >= data.len() {
//     //             break;
//     //         }

//     //         let c = data.chars().nth(c_i).unwrap();
//     //         if c != ' ' {
//     //             break;
//     //         }

//     //         *parse_ctx.i.borrow_mut() += 1;
//     //     }
//     // }

//     // pub fn parse_html_attr_key(self: &Self, data: &String, parse_ctx: &mut RustHtmlParseContext) -> String {
//     //     let mut data_str = String::new();
//     //     let mut backslash_is_escaped = false;
//     //     loop {
//     //         let c_i = *parse_ctx.i.borrow();
//     //         if c_i >= data.len() {
//     //             break;
//     //         }

//     //         let c = data.chars().nth(c_i).unwrap();
//     //         let peek_c = data.chars().nth(c_i + 1).unwrap_or('\0');

//     //         match c {
//     //             '=' => {
//     //                 break;
//     //             },
//     //             _ => {
//     //                 data_str.push(c);
//     //                 backslash_is_escaped = false;
//     //             }
//     //         }
//     //         *parse_ctx.i.borrow_mut() += 1;
//     //     }

//     //     println!("parse_html_attr_key: '{}'", data_str);
//     //     return data_str;
//     // }

//     // pub fn parse_html_attr_value(self: &Self, data: &String, parse_ctx: &mut RustHtmlParseContext) -> String {
//     //     let mut data_str = String::new();
//     //     let mut backslash_is_escaped = false;
//     //     loop {
//     //         let c_i = *parse_ctx.i.borrow();
//     //         if c_i >= data.len() {
//     //             break;
//     //         }

//     //         let c = data.chars().nth(c_i).unwrap();
//     //         let peek_c = data.chars().nth(c_i + 1).unwrap_or('\0');

//     //         match c {
//     //             '\\' => {
//     //                 backslash_is_escaped = false;
//     //                 if peek_c == '\\' {
//     //                     backslash_is_escaped = true;
//     //                 }
//     //                 data_str.push(peek_c);
//     //                 *parse_ctx.i.borrow_mut() += 1;
//     //             },
//     //             '"' => {
//     //                 if backslash_is_escaped {
//     //                     data_str.push(c);
//     //                     backslash_is_escaped = false;
//     //                 } else {
//     //                     break;
//     //                 }
//     //             },
//     //             _ => {
//     //                 data_str.push(c);
//     //                 backslash_is_escaped = false;
//     //             }
//     //         }
//     //         *parse_ctx.i.borrow_mut() += 1;
//     //     }

//     //     println!("parse_html_attr_value: '{}'", data_str);
//     //     return data_str;
//     // }

//     // pub fn parse_html_attr(self: &Self, data: &String, parse_ctx: &mut RustHtmlParseContext) -> Result<(String, String), Box<dyn Error>> {
//     //     //self.expect_char_and_increment('"', data, parse_ctx)?;
//     //     let key = self.parse_html_attr_key(data, parse_ctx);
//     //     //self.expect_char_and_increment('"', data, parse_ctx)?;
//     //     self.skip_whitespace(data, parse_ctx);
//     //     //self.expect_char_and_increment('=', data, parse_ctx)?;

//     //     Ok((
//     //         key,
//     //         if self.optional_char_and_increment('=', data, parse_ctx)? {
//     //             //println!("not going to rewind");
//     //             self.skip_whitespace(data, parse_ctx);
//     //             self.expect_char_and_increment('"', data, parse_ctx)?;
//     //             let value = self.parse_html_attr_value(data, parse_ctx);
//     //             self.expect_char_and_increment('"', data, parse_ctx)?;
//     //             value
//     //         } else {
//     //             //println!("rewind");
//     //             // rewind
//     //             //*parse_ctx.i.borrow_mut() -= 1;
//     //             String::new()
//     //         }
//     //     ))
//     // }

//     // pub fn parse_html_directive(self: &Self, data: &String, parse_ctx: &mut RustHtmlParseContext) -> Result<Box<CompiledRustHtmlTemplateNode>, Box<dyn Error>> {
//     //     let start_i = *parse_ctx.i.borrow();
        
//     //     // starts with <!
//     //     loop {
//     //         if *parse_ctx.i.borrow() >= data.len() {
//     //             break;
//     //         }
            
//     //         let c_i = *parse_ctx.i.borrow();
//     //         let c = data.chars().nth(c_i).unwrap_or('\0');

//     //         match c {
//     //             '>' => {
//     //                 break;
//     //             },
//     //             _ => {

//     //             }
//     //         }

//     //         *parse_ctx.i.borrow_mut() += 1;
//     //     }
//     //     panic!("todo implement");
//     //     RustParserHelpers::parse_rust_code(data, false, Some(self), parse_ctx)
//     // }

//     // // todo: fully implement and test this
//     // // also test rest of parsing so instead of manual testing
//     // // one can generate razor and expected output as a reference of correctness
//     // pub fn parse_html_tag(self: &Self, data: &String, is_opening_tag: bool, parse_ctx: &mut RustHtmlParseContext) -> Result<ParsedHtmlTag, Box<dyn Error>> {
//     //     let start_i = *parse_ctx.i.borrow();

//     //     let mut tag_name = String::new();
//     //     let mut html_attrs: HashMap<String, Option<String>> = HashMap::new();
//     //     let mut html_attr_key = String::new();
//     //     let mut html_attr_val = String::new();
//     //     let mut parse_attrs = false;
//     //     let mut parse_attr_val = false;
//     //     let mut is_self_contained_tag = false;

//     //     let first_c = data.chars().nth(*parse_ctx.i.borrow()).ok_or(' ');
//     //     if first_c != Ok('<') {
//     //         // all html tags must start with '<', like xml
//     //         return Err(Box::new(RustHtmlError("HTML tag did not start with '<'")));
//     //     }
//     //     *parse_ctx.i.borrow_mut() += 1;

//     //     let mut prev_c = '<';
//     //     loop {
//     //         if *parse_ctx.i.borrow() >= data.len() {
//     //             break;
//     //         }
            
//     //         let c_i = *parse_ctx.i.borrow();
//     //         let try_c = data.chars().nth(c_i);
//     //         *parse_ctx.i.borrow_mut() += 1;
//     //         let peek_c = data.chars().nth(c_i + 1).unwrap_or('\0');

//     //         match try_c {
//     //             Some(c) => {
//     //                 println!("parse_html_tag i: {}, c: {}", c_i, c);
//     //                 if parse_attrs {
//     //                     match c {
//     //                         '>' => {
//     //                             break;
//     //                         },
//     //                         '/' => {
//     //                             // must peek next char to make sure this ends the tag,
//     //                             // otherwise it gets picked up during </a> and attributes
//     //                             if peek_c == '>' {
//     //                                 println!("is_self_contained_tag = true");
//     //                                 is_self_contained_tag = true;
//     //                             } else if prev_c != '<' {
//     //                                 panic!("Unexpected character '{}' (expected '>', prev_c: '{}')", peek_c, prev_c)
//     //                             }
//     //                         },
//     //                         ' ' => {},
//     //                         _ => {
//     //                             *parse_ctx.i.borrow_mut() -= 1;
//     //                             let kvp = self.parse_html_attr(data, parse_ctx)?;
//     //                             if kvp.1.len() > 0 {
//     //                                 html_attrs.insert(kvp.0, Some(kvp.1));
//     //                             } else {
//     //                                 html_attrs.insert(kvp.0, None);
//     //                             }
//     //                         }
//     //                     }
//     //                 } else {
//     //                     match c {
//     //                         '>' => {
//     //                             break;
//     //                         },
//     //                         '/' => {
//     //                             // must peek next char to make sure this ends the tag,
//     //                             // otherwise it gets picked up during </a> and attributes
//     //                             if peek_c == '>' {
//     //                                 println!("is_self_contained_tag = true");
//     //                                 is_self_contained_tag = true;
//     //                             } else if prev_c != '<' {
//     //                                 panic!("Unexpected character '{}' (expected '>', prev_c: '{}')", peek_c, prev_c)
//     //                             }
//     //                         },
//     //                         ' ' => {
//     //                             parse_attrs = true;
//     //                         }
//     //                         _ => {
//     //                             tag_name.push(c);
//     //                         }
//     //                     }
//     //                 }
//     //                 prev_c = c;
//     //             },
//     //             None => {
//     //                 break;
//     //             }
//     //         }
//     //     }

//     //     println!("parsed html tag: {}", tag_name);
//     //     match tag_name.as_str() {
//     //         "input" => {
//     //             is_self_contained_tag = true;
//     //         },
//     //         _ => {}
//     //     }

//     //     let content = &data[start_i..*parse_ctx.i.borrow()];
//     //     return Ok(ParsedHtmlTag {
//     //         i: start_i,
//     //         len: content.len(),
//     //         content: content.to_string(),
//     //         tag_name: tag_name,
//     //         attributes: html_attrs,
//     //         is_opening_tag: is_opening_tag,
//     //         is_self_contained_tag: is_self_contained_tag
//     //     });
//     // }

//     // pub fn handle_less_than(self: &Self, c: char, matching_tags: &mut Vec<ParsedHtmlTag>, parts: &mut Vec<Box<CompiledRustHtmlTemplateNode>>, data: &String, parse_ctx: &mut RustHtmlParseContext) -> Result<Option<Box<CompiledRustHtmlTemplateNode>>, Box<dyn Error>> {
//     //     let next_char = data.chars().nth(*parse_ctx.i.borrow() + 1).expect("could not peek next char");
//     //     match next_char {
//     //         '/' => {
//     //             if matching_tags.len() == 0 {
//     //                 // closing current scope
//     //                 return Ok(None); // break
//     //             } else {
//     //                 // closing tag
//     //                 let end_html_tag = self.parse_html_tag(data, false, parse_ctx)?;
//     //                 println!("Closing tag {:?}", end_html_tag);

//     //                 // check matched tag, must match
//     //                 let matching_tag: ParsedHtmlTag = matching_tags.pop().unwrap();

//     //                 if end_html_tag.tag_name == matching_tag.tag_name {
//     //                     let part = Box::new(CompiledRustHtmlTemplateNode::new(end_html_tag.i, end_html_tag.len, CompiledRustHtmlTemplateNodeType::StaticHtmlString, None, &end_html_tag.content, HashMap::new(), vec![]));
//     //                     return Ok(Some(part));
//     //                 } else {
//     //                     return Err(Box::new(RustHtmlError("Unexpected closing tag")));
//     //                 }
//     //             }
//     //         },
//     //         '!' => {
//     //             return Ok(Some(self.parse_html_directive(data, parse_ctx)?));
//     //         },
//     //         token if token.is_ascii_alphabetic() => {
//     //             // add to matching_tags
//     //             let start_html_tag = self.parse_html_tag(data, true, parse_ctx)?;
//     //             println!("Starting tag {:?}", start_html_tag);
//     //             let part = Box::new(CompiledRustHtmlTemplateNode::new(start_html_tag.i, start_html_tag.len, CompiledRustHtmlTemplateNodeType::StaticHtmlString, None, &start_html_tag.content, HashMap::new(), vec![]));
//     //             if !start_html_tag.is_self_contained_tag {
//     //                 matching_tags.push(start_html_tag);
//     //             }
//     //             return Ok(Some(part));
//     //         },
//     //         _ => {
//     //             return Err(Box::new(RustHtmlError("Unexpected character")));
//     //         }
//     //     }
//     // }
// }
// impl RustHtmlPartParser for HtmlPartParser {
//     fn parse_part(self: &Self, data: &String, parent_parser: Option<&dyn RustHtmlPartParser>, parse_ctx: &mut RustHtmlParseContext) -> Result<Box<CompiledRustHtmlTemplateNode>, Box<dyn Error>> {
//         let start_i = *parse_ctx.i.borrow();

//         let mut text_node = String::new();

//         let mut parts = Vec::new();
//         let mut matching_tags = Vec::new(); // have to match HTML tags to switch between HTML and rust properly (exit if close matches start and not root)
//         loop {
//             let c_i = *parse_ctx.i.borrow();
//             if c_i >= data.len() {
//                 break;
//             }

//             if let Some(pp) = parent_parser {
//                 if pp.peek_end_expression(data, self, parse_ctx) {
//                     break;
//                 }
//             }
            
//             let try_c = data.chars().nth(c_i);
//             match try_c {
//                 Some(c) => {
//                     println!("html i: {}, c: '{}', is_rust_expression: {}", c_i, c, *parse_ctx.is_rust_expression.borrow());
//                     *parse_ctx.col_no.borrow_mut() += 1;
        
//                     if *parse_ctx.is_rust_expression.borrow() {
//                         match c {
//                             '\n' | ' ' => {
//                                 *parse_ctx.i.borrow_mut() += 1;
//                             },
//                             token if token.is_ascii_alphabetic() => {
//                                 // parse rust
//                                 println!("RustHtmlPartParser Start code statement");
//                                 let part = RustStatementPartParser::new().parse_part(data, Some(self), parse_ctx)?;
//                                 println!("RustHtmlPartParser End code statement: {}", part.token_content);

//                                 // check if calling function
//                                 let peek_c = data.chars().nth(c_i + part.len).unwrap();
//                                 match peek_c {
//                                     '(' => {
//                                         println!("This statement calls a function");
//                                         let functionCallPart = RustFunctionCallPartParser::new().parse_part(data, Some(self), parse_ctx)?;
//                                         parts.push(functionCallPart);
//                                     },
//                                     '=' => {
//                                         println!("This statement makes an assignment");
//                                         parts.push(part);
//                                     },
//                                     ';' => {
//                                         println!("This statement is ending");
//                                         parts.push(part);
//                                     },
//                                     _ => {
//                                         panic!("idk what to do with '{}'", peek_c)
//                                     }
//                                 }
//                             },
//                             '<' => {
//                                 let part_option = self.handle_less_than(c, &mut matching_tags, &mut parts, data, parse_ctx)?;
//                                     match part_option {
//                                         Some(part) => {
//                                         parts.push(part);
//                                     },
//                                     None => {
//                                         break;
//                                     }
//                                 }
//                             },
//                             '/' => {
//                                 let next_char = data.chars().nth(*parse_ctx.i.borrow() + 1).expect("could not get next char");
//                                 match next_char {
//                                     '/' => {
//                                         // this is a comment
//                                         CommentPartParser::new().parse_part(data, Some(self), parse_ctx);
//                                     },
//                                     _ => {
//                                         //return Err(Box::new(RustHtmlError(format!("Invalid next character '{}' on line {} (expected one of '{{', '(', '*', or alpha-numeric character)", next_char, line_no))));
//                                         panic!("Todo: fix me");
//                                     }
//                                 }
//                             },
//                             '}' => {
//                                 break;
//                                 // todo: implement me to fix 
//                                 // thread 'main' panicked at 'could not compile: RustHtmlError("Invalid character '}' on line 7 col 10 in '<li></li>\n}\n</ul>\n' (expected one of ' ', '<', or alpha-numeric character)")', src/mvc/view/rust_html_view.rs:47:55
//                             },
//                             _ => {
//                                 let start_index = match *parse_ctx.i.borrow() > 10 { true => *parse_ctx.i.borrow() - 10, _ => 0 };
//                                 let near_content = &data[start_index .. min(data.len(), *parse_ctx.i.borrow() + 10)];
//                                 return Err(Box::new(RustHtmlError("Invalid character (expected one of ' ', '<', or alpha-numeric character)")));
//                             }
//                         }
//                     } else {
//                         match c {
//                             '@' => {
//                                 let next_char = data.chars().nth(*parse_ctx.i.borrow() + 1).expect("could not get next char");
//                                 match next_char {
//                                     '{' => {
//                                         *parse_ctx.i.borrow_mut() += 1; // skip @
//                                         println!("RustHtmlPartParser Start code block");
//                                         let part = RustBlockPartParser::new().parse_part(data, Some(self), parse_ctx)?;
//                                         println!("RustHtmlPartParser Exit code block");
//                                         parts.push(part);
//                                     },
//                                     '(' => {
//                                         println!("RustHtmlPartParser Start code wrapped expression");
//                                         let part = RustBlockPartParser::new().parse_part(data, Some(self), parse_ctx)?;
//                                         println!("RustHtmlPartParser Exit code wrapped expression");
//                                         parts.push(part);
//                                     },
//                                     '*' => {
//                                         println!("RustHtmlPartParser Start comment");
//                                         let part = CommentPartParser::new().parse_part(data, Some(self), parse_ctx)?;
//                                         println!("RustHtmlPartParser Exit comment");
//                                         parts.push(part);
//                                     },
//                                     token if token.is_ascii_alphabetic() => {
//                                         *parse_ctx.i.borrow_mut() += 1; // skip @
//                                         let ident = rustc_lexer::first_token(&data[c_i+1..]);
//                                         println!("RustHtmlPartParser Start code expression");
//                                         let part = RustParserHelpers::parse_start_of_rust_directive_keyword_or_identifier(&data[c_i+1..c_i+1+ident.len], data, Some(self), parse_ctx)?;
//                                         println!("RustHtmlPartParser End code expression: {}", part.token_content);
//                                         parts.push(part);
//                                     },
//                                     _ => {
//                                         let start_index = match *parse_ctx.i.borrow() > 10 { true => *parse_ctx.i.borrow() - 10, _ => 0 };
//                                         let near_content = &data[start_index .. min(data.len(), *parse_ctx.i.borrow() + 10)];
//                                         return Err(Box::new(RustHtmlError("Invalid next character (expected one of '{{', '(', '*', or alpha-numeric character)")));
//                                     }
//                                 }
//                                 // println!("beep 1");
//                             },
//                             '<' => {
//                                 self.handle_less_than(c, &mut matching_tags, &mut parts, data, parse_ctx);
//                             },
//                             '\n' => {
//                                 *parse_ctx.i.borrow_mut() += 1;
//                                 *parse_ctx.line_no.borrow_mut() += 1;
//                                 *parse_ctx.col_no.borrow_mut() = 0;
//                             },
//                             _ => {
//                                 text_node.push(c);
//                                 // panic!("todo: {}", c);
//                             }
//                         }
//                     }
//                 }, None => {
//                     break;
//                 }
//             }
//         }
//         // if current_expression.len() > 0 {
//         //     // last char on line is always newline character
//         //     parts.push(Box::new(CompiledRustHtmlTemplateNode::new(*parse_ctx.i.borrow() - current_expression.len(), current_expression.len(), current_expression_type, None, &current_expression, HashMap::new(), inner_parts)));
//         // }

//         let total_content = data[start_i..*parse_ctx.i.borrow() - 1].to_string();
//         Ok(Box::new(CompiledRustHtmlTemplateNode::new(start_i, total_content.len(), CompiledRustHtmlTemplateNodeType::RustHtml, None, &total_content, HashMap::new(), parts)))
//     }
//     fn peek_end_expression(self: &Self, data: &String, current_parser: &dyn RustHtmlPartParser, parse_ctx: &mut RustHtmlParseContext) -> bool {
//         false
//     }
// }


// pub struct RustHtmlParser {

// }

// impl RustHtmlParser {
//     pub fn new() -> Self {
//         Self {}
//     }

//     pub fn parse(self: &Self, data: &String, parse_ctx: &mut RustHtmlParseContext) -> Result<Box<CompiledRustHtmlTemplateNode>, Box<dyn Error>> {
//         HtmlPartParser::new().parse_part(data, None, parse_ctx)
//     }
// }