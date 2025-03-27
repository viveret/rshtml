use std::hash::{DefaultHasher, Hash, Hasher};
use std::rc::Rc;
use std::sync::{Arc, OnceLock};

use core_lib::asyncly::timer_cancellation_token::TimerCancellationToken;
use proc_macro2::TokenStream;

use quote::quote;
use syn::parse::Parse;

use crate::view::parserv3::core::peekable::stream_peekable_tokentree::StreamPeekableTokenTree;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::parserv3::parserv3::ParserV3;
use crate::view::parserv3::contexts::rusthtml_parser_context::RustHtmlParserContext;
use crate::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;

use super::caching::icompiled_view::CompiledView;
use super::caching::icompiled_view_cache::ICompiledViewCache;
use super::caching::json_compiled_view_cache::JsonCompiledViewCache;
use super::caching::layered_compiled_view_cache::LayeredCompiledViewCache;
use super::caching::memory_compiled_view_cache::MemoryCompiledViewCache;


// Static cache initialized once
static RUSTHTML_MACRO_CACHE: OnceLock<LayeredCompiledViewCache> = OnceLock::new();


fn initialize_cache() -> &'static LayeredCompiledViewCache {
    RUSTHTML_MACRO_CACHE.get_or_init(|| {
        let memory_cache = Arc::new(MemoryCompiledViewCache::new());
        let json_cache = Arc::new(JsonCompiledViewCache::new("rusthtml_cache.json").unwrap());
        LayeredCompiledViewCache::new(memory_cache, json_cache)
    })
}


pub fn check_cache_rusthtml_macro_or_else(input: TokenStream, or_else: &dyn Fn(TokenStream) -> TokenStream) -> TokenStream {
    // Initialize cache on first use
    let cache = initialize_cache();
        
    // Convert TokenStream to String for hashing
    let input_str = input.to_string();

    // Create hash of input
    let mut hasher = DefaultHasher::new();
    input_str.hash(&mut hasher);
    let hash = hasher.finish();

    // Check cache
    if let Some(cached) = cache.get_compiled_view_by_hash(hash) {
        if let Some(output_string) = cached.try_get_output_code() {
            return output_string.parse().unwrap();
        }
    }
    
    let output = or_else(input);

    // Store in cache
    let view = CompiledView::new(
        "rusthtml_macro".to_string(),
        0, // index
        hash as u64,
        Some(input_str),
        Some(output.to_string())
    );
    
    cache.add_compiled_view(Arc::new(view)).unwrap();
    
    output
}

pub fn rusthtml_macro_impl_nocache(input: TokenStream) -> TokenStream {
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

pub fn rusthtml_macro_impl(input: TokenStream) -> TokenStream {
    check_cache_rusthtml_macro_or_else(input, &rusthtml_macro_impl_nocache)
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

    let output_stream = match result {
        Ok(html_render_fn) => generate_view_code(parse_context.clone(), html_render_fn),
        Err(err) => generate_error_code(err),
    }.into();

    (parse_context, output_stream)
}

fn generate_view_code(parse_context: Rc<RustHtmlParserContext>, html_render_fn: TokenStream) -> TokenStream {
    let html_render_fn = TokenStream::from_iter(html_render_fn.into_iter());
    let view_name = parse_context.get_param_string("name").expect("could not get name");
    let view_name_ident = quote::format_ident!("view_{}", view_name);

    let view_functions = parse_context.get_functions_section().map_or_else(|| quote! {}, |section| section.into());
    let view_impl = parse_context.get_impl_section().map_or_else(|| quote! {}, |section| section.into());
    let view_struct = parse_context.get_struct_section().map_or_else(|| quote! {}, |section| section.into());

    let model_type_name = parse_context.get_model_type_name();
    let model_type = parse_context.get_model_type_stream();
    let raw = parse_context.get_raw();
    let raw = raw.as_str();

    let view_model_tokens = generate_view_model_tokens(&model_type_name, &model_type);
    let view_start_tokens = generate_view_start_tokens(&parse_context, &view_name);

    let implicit_use_statements = parse_context.get_implicit_use_statements();
    let use_statements = parse_context.get_use_statements_stream();
    let inject_tokens = parse_context.get_inject_statements_stream();
    let when_compiled = chrono::prelude::Utc::now().to_rfc2822();

    let view_code = quote! {
        #implicit_use_statements
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
                    when_compiled: DateTime::parse_from_rfc2822(#when_compiled)
                                        .expect("could not parse when compiled").into(),
                }
            }

            pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
                vec![Box::new(Rc::new(Self::new()) as Rc<dyn IView>) as Box<dyn Any>]
            }

            pub fn add_to_services(services: &mut ServiceCollection) {
                services.add(ServiceDescriptor::new_from::<dyn IView, Self>(Self::new_service, ServiceScope::Singleton));
            }

            #view_impl
        }

        impl IView for #view_name_ident {
            fn get_path(&self) -> String {
                self.ViewPath.to_string()
            }
        
            fn get_raw(&self) -> String {
                self.raw.to_string()
            }
        
            fn get_model_type_name(&self) -> Option<String> {
                Some(self.model_type_name.to_string())
            }
        
            fn render(&self, view_context: &dyn IViewContext, services: &dyn IServiceCollection) -> Result<HtmlString, RustHtmlError> {
                #view_model_tokens
                #inject_tokens

                let html_output = HtmlBuffer::new();

                #view_start_tokens

                #view_functions
                
                #html_render_fn
                
                Ok(html_output.collect_html())
            }
        }
    };

    view_code
}

fn generate_view_model_tokens(model_type_name: &str, model_type: &TokenStream) -> TokenStream {
    if model_type_name.is_empty() {
        return quote! {};
    }

    let concrete_type_tokens = if model_type_name != "dyn IModel" {
        quote! {
            m.as_ref().as_any().downcast_ref::<#model_type>().expect(
                format!("could not downcast model from Rc<dyn IModel>({:?}) to {:?}", m.get_type_info(), TypeInfo::of::<#model_type>()).as_str()
            ).clone()
        }
    } else {
        quote! {
            m.as_ref()
        }
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
}

fn generate_view_start_tokens(parse_context: &Rc<RustHtmlParserContext>, view_name: &str) -> Option<TokenStream> {
    let viewstart_name = parse_context.try_get_param_string("viewstart")
        .unwrap_or_else(|| {
            if !view_name.ends_with("view_start") && !view_name.ends_with("layout") {
                "view_start.rs".to_string()
            } else {
                String::new()
            }
        });

    if viewstart_name.is_empty() || view_name == viewstart_name {
        return None;
    }

    Some(quote! {
        // println!("doing viewstart {} in {}", #viewstart_name, #view_name);
        match view_context.get_view_renderer()
            .render_view(
                &#viewstart_name.to_string(),
                view_context.get_viewmodel(),
                Some(view_context.clone()),
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
    })
}

fn generate_error_code(err: impl std::fmt::Debug) -> TokenStream {
    let err_str = format!("could not compile rust html: {:?}", err);
    quote! { compile_error!(#err_str); }
}

fn rusthtml_view_macro_impl_nocache(input: TokenStream) -> TokenStream {
    rusthtml_view_macro_with_context(input).1
}

pub fn rusthtml_view_macro_impl(input: TokenStream) -> TokenStream {
    check_cache_rusthtml_macro_or_else(input, &rusthtml_view_macro_impl_nocache)
}