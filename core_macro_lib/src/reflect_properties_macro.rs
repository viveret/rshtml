use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use crate::extend_derive::ExtendDerive;


pub(crate) fn reflect_properties(attr: TokenStream, item: TokenStream) -> TokenStream {
    let extend_derive = ExtendDerive::parse(attr, item).unwrap();
    extend_derive.add_append_processor(&|extend_derive: &ExtendDerive| {
        let mut reflected_properties_tokens = vec![];
        extend_derive.get_struct_properties().iter().for_each(|x| {
            let name = x.1.to_string();
            let value = TokenStream::from_iter(x.2.clone().into_iter());
            // println!("value: {}", value.to_string());
            reflected_properties_tokens.push(quote::quote! {
                Rc::new(ReflectedProperty::new(
                    #name.to_string(),
                    Box::new(TypeInfo::of::<#value>()),
                )),
            });
        });
        let reflected_properties = proc_macro2::TokenStream::from_iter(reflected_properties_tokens);
        let name = &extend_derive.struct_name;
        quote::quote! {
            impl #name {
                pub fn reflected_properties() -> Vec<Rc<dyn IModelProperty>> {
                    vec![
                        #reflected_properties
                    ]
                }
            }
        }.into_iter().collect::<Vec<TokenTree>>()
    });
    extend_derive.finalize()
}
