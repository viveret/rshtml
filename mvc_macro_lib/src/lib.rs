extern crate proc_macro;
extern crate proc_macro2;
extern crate mvc_lib;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};

use mvc_lib::view::rusthtml::rusthtml_parser::RustHtmlParser;


#[proc_macro]
pub fn rusthtml_macro(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let parser = RustHtmlParser::new();
    let result = parser.expand_tokenstream(input);
    TokenStream::from(match result {
        Ok(tokens) => tokens,
        Err(err) => {
            let err_str = format!("could not compile rust html: {:?}", err);
            quote! { compile_error!(#err_str); }
        },
    })
}

// puts render function into a structure with additional functionality and information
#[proc_macro]
pub fn rusthtml_view_macro(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let parser = RustHtmlParser::new();
    let result = parser.expand_tokenstream(input);
    TokenStream::from(match result {
        Ok(html_render_fn) => {
            let view_name = parser.get_param_string("name");
            let view_name_ident = quote::format_ident!("view_{}", view_name);
            let view_functions = parser.get_functions_section();
            let model_type_name = parser.get_model_type_name();
            let model_type = proc_macro2::TokenStream::from_iter(parser.get_model_type().iter().cloned());
            let raw = parser.raw.borrow().clone();
            // println!("view_name_ident: {}", view_name_ident);
            //println!("html_render_fn: {:?}", html_render_fn);
            quote! {
                use std::any::Any;
                use std::error::Error;
                use std::cell::RefCell;
                use std::collections::HashMap;
                use std::rc::Rc;
                use std::sync::{Arc, RwLock};

                extern crate mvc_lib;
                use mvc_lib::contexts::view_context::IViewContext;
                use mvc_lib::services::service_collection::IServiceCollection;
                use mvc_lib::view::rusthtml::html_string::HtmlString;
                use mvc_lib::view::iview::IView;

                pub struct #view_name_ident {
                    model_type_name: &'static str,
                    html_buffer: RefCell<String>,
                    ViewData: RefCell<HashMap<&'static str, &'static str>>,
                    ViewPath: &'static str,
                    raw: &'static str,
                    model: Option<#model_type>,
                }
                impl #view_name_ident {
                    pub fn new() -> Self {
                        Self {
                            model_type_name: #model_type_name,
                            html_buffer: RefCell::new(String::new()),
                            ViewData: RefCell::new(HashMap::new()),
                            ViewPath: file!(),
                            model: None,
                            raw: #raw
                        }
                    }

                    #view_functions

                    pub fn get_ViewData(self: &Self, key: &'static str) -> &'static str {
                        self.ViewData.borrow().get(key).unwrap_or(&"")
                    }

                    pub fn append_html(self: &Self, html: &str) {
                        // println!("append_html: {}", html);
                        self.html_buffer.borrow_mut().push_str(html);
                    }

                    pub fn RenderSection(self: &Self, section_name: &str) {
                        println!("RenderSection: {}", section_name);
                    }

                    pub fn RenderSectionOptional(self: &Self, section_name: &str) {
                        println!("RenderSectionOptional: {}", section_name);
                    }

                    pub fn RenderBody(self: &Self) {
                        println!("RenderBody");
                    }

                    pub fn collect_html(self: &Self) -> Result<Box<HtmlString>, Box<dyn Error + 'static>> {
                        Ok(Box::new(HtmlString::new_from_html(self.html_buffer.borrow().clone())))
                    }
                }
                impl IView for #view_name_ident {
                    fn get_path(self: &Self) -> String {
                        self.ViewPath.to_string()
                    }
                
                    fn get_raw(self: &Self) -> String {
                        self.raw.to_string()
                    }
                
                    // if the view defines a model type, this returns the type id
                    fn get_model_type_name(self: &Self) -> Option<String> {
                        Some(self.model_type_name.to_string())
                    }
                
                    // using template, render the view given the current data
                    fn render(self: &Self, ctx: Arc<RwLock<dyn IViewContext>>, services: Arc<RwLock<dyn IServiceCollection>>) -> Result<Box<HtmlString>, Box<dyn Error + 'static>> {
                        //Ok(Box::new(HtmlString::new_from_html(String::new())))
                        #html_render_fn
                        self.collect_html()
                    }
                }
            }
        },
        Err(err) => {
            let err_str = format!("could not compile rust html: {:?}", err);
            quote! { compile_error!(#err_str); }
        },
    })
}