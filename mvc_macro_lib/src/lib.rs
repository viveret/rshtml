extern crate proc_macro;
extern crate proc_macro2;
extern crate mvc_lib;

use std::rc::Rc;

use core_lib::asyncly::timer_cancellation_token::TimerCancellationToken;
use mvc_lib::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use mvc_lib::view::rusthtml::parser_parts::rusthtmlparser_all::IRustHtmlParserAll;
use mvc_lib::view::rusthtml::parser_parts::rusthtmlparser_all::RustHtmlParserAll;
use mvc_lib::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use proc_macro2::TokenStream;
use proc_macro2::Ident;
use proc_macro2::TokenTree;
use quote::quote;


#[proc_macro]
pub fn rusthtml_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parser = RustHtmlParserAll::new_default();
    let ct = Rc::new(TimerCancellationToken::new(std::time::Duration::from_secs(5)));
    let result = parser.expand_rust(input.into(), ct.clone());
    ct.stop().expect("could not stop timer");
    match result {
        Ok(tokens) => {
            tokens.into()
        },
        Err(err) => {
            let err_str = format!("could not compile rust html: {:?}", err);
            quote! { compile_error!(#err_str); }.into()
        },
    }
}

// puts render function into a structure with additional functionality and information
#[proc_macro]
pub fn rusthtml_view_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parse_context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let parser = RustHtmlParserAll::new_default();
    let ct = Rc::new(TimerCancellationToken::new(std::time::Duration::from_secs(5)));
    let result = parser.expand_rust_with_context(parse_context.clone(), input.into(), ct.clone());
    ct.stop().expect("could not stop timer");
    match result {
        Ok(html_render_fn2) => {
            let html_render_fn = TokenStream::from_iter(html_render_fn2.into_iter());
            let view_name = parse_context.get_param_string("name");
            let view_name = view_name.expect("could not get name");
            let view_name_ident = quote::format_ident!("view_{}", view_name);
            let _view_name_context_ident = quote::format_ident!("view_{}_context", view_name);
            let view_functions = match parse_context.get_functions_section() {
                Some(functions_section) => functions_section.into(),
                None => quote! {},
            };
            let view_impl = match parse_context.get_impl_section() {
                Some(impl_section) => impl_section.into(),
                None => quote! {},
            };
            let view_struct = match parse_context.get_struct_section() {
                Some(struct_section) => struct_section.into(),
                None => quote! {},
            };
            let model_type_name = parse_context.get_model_type_name();
            let model_type = parse_context.get_model_type();
            let raw = parse_context.get_raw();

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

            let use_statements = parse_context.get_use_statements_stream();
            let inject_tokens = parse_context.get_inject_statements_stream();
            let when_compiled = chrono::prelude::Utc::now().to_rfc2822();
            let mut view_start_tokens: Option<TokenStream> = None;
            if let Some(view_start) = parse_context.try_get_param_string("viewstart") {
                // println!("view_start_path: {}", view_start_path);
                view_start_tokens = Some(quote! {
                    match view_context.get_view_renderer()
                        .render_with_layout_if_specified(
                            &#view_start.to_string(),
                            view_context.get_viewmodel(),
                            view_context.get_request_context(),
                            services
                        ) {
                            Ok(html) => {
                                html_output.write_html(html);
                            },
                            Err(err) => {
                                html_output.write_html_str(format!("could not render view_start: {}", err).as_str());
                            }
                        }
                });
            }

            let s = quote! {
                #use_statements

                pub struct #view_name_ident {
                    model_type_name: &'static str,
                    ViewPath: &'static str,
                    raw: &'static str,
                    when_compiled: DateTime<Utc>,
                    // view_context: RefCell<&'static dyn IViewContext>,
                    #view_struct
                }

                impl #view_name_ident {
                    pub fn new() -> Self {
                        Self {
                            model_type_name: #model_type_name,
                            ViewPath: file!(),
                            raw: #raw,
                            when_compiled: DateTime::parse_from_rfc2822(#when_compiled)
                                                        .expect("could not parse when compiled").into(),
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
                        // self.view_context.replace(view_context);

                        #view_model_tokens
                        #inject_tokens

                        let html_output = HtmlBuffer::new();

                        #view_start_tokens

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
    let action_name = match input.into_iter().next() {
        Some(TokenTree::Ident(ident)) => ident.clone(),
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