use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use crate::extend_derive::ExtendDerive;

// proc macro for gathering all the attributes used in a type and storing them in a field.
pub fn reflect_attributes(attr: TokenStream, item: TokenStream) -> TokenStream {
    let extend_derive = ExtendDerive::parse(attr, item).unwrap();
    let x = |extend_derive: &ExtendDerive| {
        let reflected_attrs_tokens = extend_derive.finalize_attrs();

        let reflected_attrs = TokenStream::from_iter(reflected_attrs_tokens.into_iter());
        let name = &extend_derive.struct_name;

        quote::quote! {
            impl #name {
                pub fn reflected_attributes() -> Vec<Rc<dyn IAttribute>> {
                    vec![
                        #reflected_attrs
                    ]
                }
            }
        }.into_iter().collect::<Vec<TokenTree>>()
    };
    extend_derive.add_append_processor(&x);
    extend_derive.finalize()
}
