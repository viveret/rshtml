
extern crate proc_macro;
extern crate proc_macro2;
extern crate mvc_lib;

use proc_macro::TokenStream;
use quote::quote;

use mvc_lib::view::{rusthtml::rusthtml_parser::RustHtmlParser};

#[proc_macro]
pub fn rusthtml_macro(input: TokenStream) -> TokenStream {
    // let input = proc_macro2::TokenStream::from(input);
    let parser = RustHtmlParser::new(false, "Development".to_string());
    let result = parser.expand_tokenstream(input);
    TokenStream::from(match result {
        Ok(tokens) => tokens,
        Err(err) => {
            let err_str = format!("could not compile rust html: {:?}", err);
            TokenStream::from(quote! { compile_error!(#err_str); })
        },
    })
}

// puts render function into a structure with additional functionality and information
#[proc_macro]
pub fn rusthtml_view_macro(input: TokenStream) -> TokenStream {
    let parser = RustHtmlParser::new(false, "Development".to_string());
    let result = parser.expand_tokenstream(input);
    TokenStream::from(match result {
        Ok(html_render_fn2) => {
            let html_render_fn = proc_macro2::TokenStream::from(html_render_fn2);
            let view_name = parser.get_param_string("name").unwrap();
            let view_name_ident = quote::format_ident!("view_{}", view_name);
            let _view_name_context_ident = quote::format_ident!("view_{}_context", view_name);
            let view_functions = match parser.get_functions_section() {
                Some(functions_section) => proc_macro2::TokenStream::from(functions_section),
                None => quote! {},
            };
            let view_impl = match parser.get_impl_section() {
                Some(impl_section) => proc_macro2::TokenStream::from(impl_section),
                None => quote! {},
            };
            let view_struct = match parser.get_struct_section() {
                Some(struct_section) => proc_macro2::TokenStream::from(struct_section),
                None => quote! {},
            };
            let model_type_name = parser.get_model_type_name();
            let model_type = proc_macro2::TokenStream::from(TokenStream::from_iter(parser.get_model_type().iter().cloned()));
            let raw = parser.raw.borrow().clone();

            let view_model_tokens = if model_type_name.len() > 0 {
                quote! {
                    let vm = view_context.get_viewmodel();
                    let model: Rc<#model_type> = match vm.deref() {
                        Some(m) => m.as_ref().downcast_ref::<Rc<#model_type>>().expect(format!("could not downcast model from Box<dyn Any> to Rc<{}>", std::any::type_name::<#model_type>()).as_str()).clone(),
                        None => panic!("No model set")
                    };
                }
            } else {
                quote! {}
            };

            let use_statements = proc_macro2::TokenStream::from(TokenStream::from_iter(parser.use_statements.borrow().iter().cloned().map(|s| s.into_iter()).flatten()));

            let when_compiled = chrono::prelude::Utc::now().to_rfc2822();

            quote! {
                use std::any::Any;
                use std::borrow::Cow;
                use std::cell::RefCell;
                use std::collections::HashMap;
                use std::error::Error;
                use std::rc::Rc;
                use std::io::Read;
                use std::ops::Deref;
                use std::sync::{Arc, RwLock};

                use chrono::{DateTime, TimeZone, Utc};

                extern crate mvc_lib;
                use mvc_lib::contexts::controller_context::IControllerContext;
                use mvc_lib::contexts::view_context::IViewContext;
                use mvc_lib::services::service_collection::IServiceCollection;
                use mvc_lib::view::html_helpers::helpers::HtmlHelpers;
                use mvc_lib::view::rusthtml::html_string::HtmlString;
                use mvc_lib::view::rusthtml::rusthtml_error::RustHtmlError;
                use mvc_lib::view::rusthtml::rusthtml_view_macros::RustHtmlViewMacros;
                use mvc_lib::view::iview::IView;

                #use_statements

                pub struct #view_name_ident {
                    model_type_name: &'static str,
                    ViewPath: &'static str,
                    raw: &'static str,
                    when_compiled: DateTime<Utc>,
                    #view_struct
                }

                impl #view_name_ident {
                    pub fn new() -> Self {
                        Self {
                            model_type_name: #model_type_name,
                            ViewPath: file!(),
                            raw: #raw,
                            when_compiled: DateTime::parse_from_rfc2822(#when_compiled).unwrap().into()
                        }
                    }

                    pub fn new_service() -> Box<dyn Any> {
                        Box::new(Rc::new(Self::new()) as Rc<dyn IView>) as Box<dyn Any>
                    }

                    #view_impl
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
                    fn render(self: &Self, view_context: &dyn IViewContext, services: &dyn IServiceCollection) -> Result<HtmlString, RustHtmlError> {
                        #view_model_tokens
                        let html = HtmlHelpers::new();

                        let render_section = |section_name: &str| -> Result<HtmlString, RustHtmlError> {
                            RustHtmlViewMacros::render_section(section_name, self, view_context, services)
                        };

                        let render_section_optional = |section_name: &str| -> Result<HtmlString, RustHtmlError> {
                            RustHtmlViewMacros::render_section_optional(section_name, self, view_context, services)
                        };

                        let try_render_body = || -> Result<HtmlString, RustHtmlError> {
                            RustHtmlViewMacros::render_body(self, view_context, services)
                        };

                        let render_body = || -> HtmlString {
                            match RustHtmlViewMacros::render_body(self, view_context, services) {
                                Ok(html) => html,
                                Err(e) => panic!("{}", e),
                            }
                        };

                        #view_functions
                        
                        #html_render_fn
                        
                        // should all be written to view_context html_buffer
                        Ok(HtmlString::empty())
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