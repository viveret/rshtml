extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;


#[proc_macro]
pub fn nameof_member_fn(input: TokenStream) -> TokenStream {
    let mut it = proc_macro2::TokenStream::from(input).into_iter();
    // expect type name
    let type_name = it.next().unwrap();
    match type_name {
        proc_macro2::TokenTree::Ident(ident) => {
            let type_name = ident.to_string();
        },
        _ => panic!("Expected type name."),
    }

    // expect ::
    let colon_first = it.next().unwrap();
    match colon_first {
        proc_macro2::TokenTree::Punct(punct) => {
            if punct.as_char() != ':' {
                panic!("Expected ::");
            }
        },
        _ => panic!("Expected ::"),
    }
    let colon_second = it.next().unwrap();
    match colon_second {
        proc_macro2::TokenTree::Punct(punct) => {
            if punct.as_char() != ':' {
                panic!("Expected ::");
            }
        },
        _ => panic!("Expected ::"),
    }

    // expect member fn name
    let member_fn_name = it.next().unwrap();
    match member_fn_name {
        proc_macro2::TokenTree::Ident(ident) => {
            let member_fn_name = ident.to_string();
            return TokenStream::from(quote! {
                #member_fn_name
            });
        },
        _ => panic!("Expected member fn name."),
    }
}