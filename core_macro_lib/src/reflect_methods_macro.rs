use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use crate::extend_derive::ExtendDerive;


pub(crate) fn reflect_methods(attr: TokenStream, item: TokenStream) -> TokenStream {
    let extend_derive = ExtendDerive::parse(attr, item).unwrap();
    extend_derive.add_append_processor(&|extend_derive: &ExtendDerive| {
        let mut reflected_methods_tokens = vec![];
        extend_derive.get_struct_methods().iter().for_each(|x| {
            reflected_methods_tokens.push(x.finalize());
        });
        let reflected_methods = proc_macro2::TokenStream::from_iter(reflected_methods_tokens);
        let name = &extend_derive.struct_name;
        quote::quote! {
            impl #name {
                pub fn reflected_methods() -> Vec<Rc<dyn IModelMethod>> {
                    vec![
                        #reflected_methods
                    ]
                }
            }
        }.into_iter().collect::<Vec<TokenTree>>()
    });
    extend_derive.finalize()
}