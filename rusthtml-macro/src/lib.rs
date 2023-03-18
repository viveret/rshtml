extern crate proc_macro;
extern crate proc_macro2;
extern crate rusthtml;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};

use rusthtml::rusthtml_parser::RustHtmlParser;


#[proc_macro]
pub fn rusthtml_macro(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let parser = RustHtmlParser::new();
    let result = parser.parse_to_tokenstream(input);
    TokenStream::from(match result {
        Ok(tokens) => tokens,
        Err(err) => {
            let err_str = format!("could not compile rust html: {:?}", err);
            quote! { compile_error!(#err_str); }
        },
    })
}

// puts render FN into a structure with additional debugging information
#[proc_macro]
pub fn rusthtml_view_macro(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let parser = RustHtmlParser::new();
    let result = parser.parse_to_tokenstream(input);
    TokenStream::from(match result {
        Ok(html_render_fn) => {
            let view_name = parser.get_param_string("name");
            let view_name_ident = quote::format_ident!("view_{}", view_name);
            let view_functions = parser.get_functions_section();
            quote! {
                use rusthtml::html_string::HtmlString;
                pub struct #view_name_ident {
                }
                impl #view_name_ident {
                    #view_functions
                    pub fn render() -> HtmlString {
                        #html_render_fn
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