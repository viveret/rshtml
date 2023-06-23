
extern crate proc_macro;
extern crate proc_macro2;
extern crate mvc_lib;

use proc_macro2::TokenStream;
use proc_macro2::Ident;
use proc_macro2::TokenTree;
use quote::quote;

use mvc_lib::view::rusthtml::rusthtml_parser::RustHtmlParser;

#[proc_macro]
pub fn rusthtml_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let input = input);
    let parser = RustHtmlParser::new(false, "Development".to_string());
    let result = parser.expand_tokenstream(input.into());
    match result {
        Ok(tokens) => tokens.into(),
        Err(err) => {
            let err_str = format!("could not compile rust html: {:?}", err);
            quote! { compile_error!(#err_str); }.into()
        },
    }
}

// puts render function into a structure with additional functionality and information
#[proc_macro]
pub fn rusthtml_view_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parser = RustHtmlParser::new(false, "Development".to_string());
    let result = parser.expand_tokenstream(input.into());
    match result {
        Ok(html_render_fn2) => {
            let html_render_fn = html_render_fn2;//.into();
            let view_name = parser.parse_context.get_param_string("name").unwrap();
            let view_name_ident = quote::format_ident!("view_{}", view_name);
            let _view_name_context_ident = quote::format_ident!("view_{}_context", view_name);
            let view_functions = match parser.parse_context.get_functions_section() {
                Some(functions_section) => functions_section.into(),
                None => quote! {},
            };
            let view_impl = match parser.parse_context.get_impl_section() {
                Some(impl_section) => impl_section.into(),
                None => quote! {},
            };
            let view_struct = match parser.parse_context.get_struct_section() {
                Some(struct_section) => struct_section.into(),
                None => quote! {},
            };
            let model_type_name = parser.parse_context.get_model_type_name();
            let model_type = TokenStream::from_iter(parser.parse_context.get_model_type().iter().cloned());
            let raw = parser.parse_context.get_raw();

            let view_model_tokens = if model_type_name.len() > 0 {
                let concrete_type_tokens = if model_type_name != "dyn IModel" {
                    // println!("model_type_name: {}", model_type_name);
                    Some(quote! {
                        m.as_ref().as_any().downcast_ref::<#model_type>().expect(
                            format!("could not downcast model from Rc<dyn IModel>({:?}) to {:?}", m.get_type_info(), TypeInfo::of::<#model_type>()).as_str()
                        ).clone()
                    })
                } else {
                    Some(quote! {
                        m.as_ref()
                    })
                };
                
                quote! {
                    let vm = view_context.get_viewmodel();
                    let model = match vm {
                        Some(m) => {
                            #concrete_type_tokens
                        },
                        None => panic!("No model set")
                    };
                }
            } else {
                quote! {}
            };

            let use_statements = TokenStream::from_iter(parser.parse_context.mut_use_statements().iter().cloned().map(|s| s.into_iter()).flatten());
            let inject_tokens = parser.parse_context.get_inject_statements_stream();
            let when_compiled = chrono::prelude::Utc::now().to_rfc2822();

            let s = quote! {
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
                        #inject_tokens

                        let html_output = HtmlBuffer::new();

                        #view_functions
                        
                        #html_render_fn
                        
                        // should all be written to view_context html_output
                        Ok(html_output.collect_html())
                    }
                }
            };

            // if view_name == "dev_log" {
            //     println!("rusthtml_view_macro: {}", s.to_string());
            // }
            s
        },
        Err(err) => {
            let err_str = format!("could not compile rust html: {:?}", err);
            quote! { compile_error!(#err_str); }
        },
    }.into()
}


fn rc_controller_action_impl(new_fn: &Ident, input: TokenStream) -> TokenStream {
    let action_name = match input.into_iter().next().unwrap() {
        TokenTree::Ident(ident) => ident.clone(),
        _ => panic!("expected ident"),
    };
    quote! {
        Rc::new(
            ControllerActionMemberFn::#new_fn(
                vec![],
                None,
                action_name_to_path(
                    IControllerExtensions::get_name_ref(self),
                    nameof_member_fn!(Self::#action_name)
                ),
                nameof_member_fn!(Self::#action_name).into(),
                IControllerExtensions::get_name(self).into(),
                self.get_route_area(),
                Box::new(Self::#action_name)
            )
        )
    }.into()
}

#[proc_macro]
pub fn rc_controller_action(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    rc_controller_action_impl(&Ident::new("new_not_validated", proc_macro2::Span::call_site()), input.into()).into()
}

#[proc_macro]
pub fn rc_controller_action_validate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    rc_controller_action_impl(&Ident::new("new_validated", proc_macro2::Span::call_site()), input.into()).into()
}

#[proc_macro]
pub fn rc_controller_action_validate_typed(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    rc_controller_action_impl(&Ident::new("new_validated_typed", proc_macro2::Span::call_site()), input.into()).into()
}