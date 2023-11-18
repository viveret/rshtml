extern crate proc_macro;
extern crate proc_macro2;

use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;

mod ast;
mod extend_derive;
mod reflect_attributes_macro;
mod reflect_methods_macro;
mod reflect_properties_macro;
mod ihaz_attributes_macro;
mod imodel_macro;
mod iviewmodel_macro;



#[proc_macro]
pub fn nameof_member_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut it = TokenStream::from(input).into_iter();
    // expect type name
    let type_name = match it.next() {
        Some(TokenTree::Ident(ident)) => {
            ident.to_string()
        },
        _ => panic!("Expected type name."),
    };

    // expect ::
    let colon_first = match it.next() {
        Some(TokenTree::Punct(punct)) => {
            if punct.as_char() != ':' {
                panic!("Expected ::");
            }
        },
        _ => panic!("Expected ::"),
    };
    
    let colon_second = match it.next() {
        Some(TokenTree::Punct(punct)) => {
            if punct.as_char() != ':' {
                panic!("Expected ::");
            }
        },
        _ => panic!("Expected ::"),
    };

    // expect member fn name
    match it.next() {
        Some(TokenTree::Ident(ident)) => {
            let member_fn_name = ident.to_string();
            return quote! {
                #member_fn_name
            }.into();
        },
        _ => panic!("Expected member fn name."),
    }
}


#[proc_macro]
pub fn expr_quote(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut expr_tokens = vec![];
    // expecting anonymous function
    let mut it = TokenStream::from(input).into_iter().peekable();
    // expecting closure ||
    for _ in 0..3 {
        match it.next() {
            Some(token) => {
                match &token {
                    TokenTree::Punct(punct) => {
                        if punct.as_char() != '|' {
                            panic!("Expected closure pipe |, not {}.", punct.as_char());
                        } else {
                            expr_tokens.push(token);
                        }
                    },
                    TokenTree::Ident(closure_pipe_or_model_ident) => {
                        // start of identitifer for model argument
                        expr_tokens.push(token);
                    },
                    _ => panic!("Expected closure pipe |"),
                }
            },
            None => panic!("Expected closure pipe |"),
        }
    }

    // expecting closure body, a basic identifier
    loop {
        if let Some(next) = it.peek() {
            match next {
                TokenTree::Ident(_) => {
                    match it.next() {
                        Some(token) => {
                            expr_tokens.push(token);
                        },
                        None => panic!("Expected closure body"),
                    }
                },
                TokenTree::Punct(punct) => {
                    if punct.as_char() == '.' || punct.as_char() == '&' {
                        match it.next() {
                            Some(token) => {
                                expr_tokens.push(token);
                            },
                            None => panic!("Expected closure body"),
                        }
                    } else {
                        // break;
                        panic!("Expected dot / period between identifier parts, not {}.", punct.as_char());
                    }
                },
                _ => break,
            }
        } else {
            break;
        }
    }

    let expr_stream = TokenStream::from_iter(expr_tokens);
    quote! {
        (#expr_stream .to_string(), quote::quote! { #expr_stream })
    }.into()
}

#[proc_macro_derive(IModel)]
pub fn imodel_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).expect("Couldn't parse input.");
    imodel_macro::impl_imodel(&ast).into()
}

#[proc_macro_derive(IViewModel)]
pub fn iviewmodel_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).expect("Couldn't parse input.");
    iviewmodel_macro::impl_iviewmodel(&ast).into()
}

#[proc_macro_derive(IHazAttributes)]
pub fn ihaz_attributes_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).expect("Couldn't parse input.");
    ihaz_attributes_macro::impl_ihaz_attributes(&ast).into()
}

// proc macro for gathering all the attributes used in a type and storing them in a field.
#[proc_macro_attribute]
pub fn reflect_attributes(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    reflect_attributes_macro::reflect_attributes(attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn reflect_properties(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    reflect_properties_macro::reflect_properties(attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn reflect_methods(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    reflect_methods_macro::reflect_methods(attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn display_name(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let display_name: String = attr.into_iter().next().unwrap().to_string();
    item
}

#[proc_macro_attribute]
pub fn fake_property_attribute(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let display_name: String = attr.into_iter().next().unwrap().to_string();
    // println!("attr: \"{}\"", attr.to_string());
    // println!("item: \"{}\"", item.to_string());
    item
}

