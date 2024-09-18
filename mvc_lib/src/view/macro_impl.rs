use std::rc::Rc;

use core_lib::asyncly::timer_cancellation_token::TimerCancellationToken;
use proc_macro::TokenTree;
use proc_macro2::TokenStream;

use quote::quote;

use crate::view::parserv3::core::peekable::stream_peekable_tokentree::StreamPeekableTokenTree;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::parserv3::parserv3::ParserV3;
use crate::view::parserv3::contexts::rusthtml_parser_context::RustHtmlParserContext;
use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;


pub fn rusthtml_macro_impl(input: TokenStream) -> TokenStream {
    let parser = ParserV3::new_default();
    let ct = Rc::new(TimerCancellationToken::new(std::time::Duration::from_secs(5)));
    let context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let it = Rc::new(StreamPeekableTokenTree::new(input.into()));
    let result = parser.expand(it, context, ct.clone());
    ct.stop().expect("could not stop timer");
    match result {
        Ok(tokens) => {
            tokens.to_stream().into()
        },
        Err(err) => {
            let err_str = format!("could not compile rust html: {:?}", err);
            quote! { compile_error!(#err_str); }.into()
        },
    }
}

fn call_parser_expand(
    input: TokenStream,
    context: Rc<dyn IRustHtmlParserContext>,
) -> (Result<TokenStream, RustHtmlError>, Option<Rc<ParserV3>>) {
    let ct = Rc::new(TimerCancellationToken::new(std::time::Duration::from_secs(5)));
    let parser3 = ParserV3::new_default();
    let res = parser3.expand_tokentree(input.into(), context.clone(), ct.clone());
    let result: Option<(Result<TokenStream, RustHtmlError>, Option<Rc<ParserV3>>)> = Some((res, None));
    ct.stop().expect("could not stop timer");
    result.unwrap()
}

pub fn rusthtml_view_macro_with_context(input: TokenStream) -> (Rc<RustHtmlParserContext>, TokenStream) {
    let parse_context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let (result, _) = call_parser_expand(input, parse_context.clone());

    let log_final_view_to_external_file = true;

    let output_stream = match result {
        Ok(html_render_fn2) => {
            println!("html_render_fn2: {}", html_render_fn2.to_string());
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
            let model_type = parse_context.get_model_type_stream();
            let rawquote = parse_context.get_raw();
            let raw = rawquote.as_str();

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

            let implicit_use_statements = parse_context.get_implicit_use_statements();
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
                #implicit_use_statements
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

            if log_final_view_to_external_file {
                std::fs::create_dir_all("rusthtml-tmp/views/").expect("could not create tmp folder");
                let path = format!("rusthtml-tmp/views/{}.rs", view_name);
                std::fs::remove_file(path.as_str());
                std::fs::write(path.as_str(), s.to_string()).expect("could not write contents");
            }
            s
        },
        Err(err) => {
            let err_str = format!("could not compile rust html: {:?}", err);
            quote! { compile_error!(#err_str); }
        },
    }.into();

    (parse_context, output_stream)
}

pub fn rusthtml_view_macro_impl(input: TokenStream) -> TokenStream {
    rusthtml_view_macro_with_context(input).1
}