use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::cell::RefCell;
use std::cmp::{min, max};
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::result::Result;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::vec::Vec;

use glob::glob;

use crate::view::rusthtml::html_string::HtmlString;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_parser::RustHtmlParser;

use crate::view::iview::IView;
use crate::view::rust_html_parser::{*};

use crate::contexts::view_context::IViewContext;
use crate::contexts::controller_context::IControllerContext;

use crate::services::service_collection::IServiceCollection;


// pub struct RustHtmlView {
//     // template_path: String,
//     // template_data: String,
//     // model_type_name: Option<String>,

//     // compiled_template: Rc<CompiledRustHtmlTemplateNode>
// }


// impl RustHtmlView {
//     pub fn new(render_fn: Box<dyn Fn() -> HtmlString>) -> Self { // path: String, f: File
//         // let mut template_whole = String::new();
//         // let reader = std::io::BufReader::new(f);
//         // for line_result in reader.lines() {
//         //     if let Ok(line) = line_result {
//         //         template_whole.push_str(&line);
//         //         template_whole.push_str("\n"); // don't forget newline
//         //     }
//         // }

//         // RustHtmlView::compile(&template_whole, &path).expect("could not compile")
//     }

//     // pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Rc<dyn Any>> {
//     //     glob("views/**/*.rshtml")
//     //         .expect("Failed to read glob pattern")
//     //         .map(|path_to_string| {
//     //             path_to_string.unwrap().as_path().to_str().unwrap().to_string()
//     //         })
//     //         .map(|path| {
//     //             Rc::new(Box::new(Self::new(path.clone(), File::open(path).expect("oops"))) as Box<dyn IView>) as Rc<dyn Any>
//     //         })
//     //         .collect()
//     // }

//     // pub fn compile(template_data: &String, path: &String) -> Result<Self, Box<dyn Error>> {
//     //     //let mut root_parse_ctx = RustHtmlParseContext::new_root();
//     //     let parser = RustHtmlParser::new();
//     //     let token_stream = proc_macro2::TokenStream::from_str(template_data)?;
//     //     let parsed_rusthtml = parser.parse_to_tokenstream(token_stream)?;

//     //     Ok(Box::new(Self {
//     //         render_fn: parsed_rusthtml.ast().unwrap(),
//     //         // template_path: path.to_string(),
//     //         // template_data: template_data.clone(),
//     //         // compiled_template: Rc::new((template_data, doc_part)),
//     //         // model_type_name: root_parse_ctx.model_type_name.clone().borrow().clone(),
//     //     }))
//     // }
// }

// impl IView for RustHtmlView {
//     fn get_path(self: &Self) -> String {
//         self.template_path.clone()
//     }

//     fn get_raw(self: &Self) -> String {
//         self.template_data.clone()
//     }

//     // if the view defines a model type, this returns the type id
//     fn get_model_type_name(self: &Self) -> Option<String>  {
//         match &self.model_type_name {
//             Some(type_name)  => Some(type_name.clone()),
//             None => None,
//         }
//     }

//     fn render(self: &Self, ctx: Arc<RwLock<dyn IViewContext>>, services: Arc<RwLock<dyn IServiceCollection>>) -> Result<Box<HtmlString>, Box<dyn Error + 'static>> {
//         self.compiled_template.render_part(ctx.clone(), services.clone())
//     }
// }

// impl IView for &RustHtmlView {
//     fn get_path(self: &Self) -> String {
//         self.template_path.clone()
//     }

//     fn get_raw(self: &Self) -> String {
//         self.template_data.clone()
//     }

//     // if the view defines a model type, this returns the type id
//     fn get_model_type_name(self: &Self) -> Option<String>  {
//         match &self.model_type_name {
//             Some(type_name)  => Some(type_name.clone()),
//             None => None,
//         }
//     }

//     fn render(self: &Self, ctx: Arc<RwLock<dyn IViewContext>>, services: Arc<RwLock<dyn IServiceCollection>>) -> Result<Box<HtmlString>, Box<dyn Error + 'static>> {
//         self.compiled_template.render_part(ctx.clone(), services.clone())
//     }
// }