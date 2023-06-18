use proc_macro2::TokenStream;
use quote::quote;


pub(crate) fn impl_iviewmodel(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    quote! {
        impl IViewModel for #name {
        }
    }
}
