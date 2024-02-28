use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use crate::extend_derive::ExtendDerive;


pub(crate) fn reflect_properties(attr: TokenStream, item: TokenStream) -> TokenStream {
    match ExtendDerive::parse(attr, item) {
        Ok(extend_derive) => {
            extend_derive.add_append_processor(&|extend_derive: &ExtendDerive| {
                let props = extend_derive.get_struct_properties();
                let reflected_properties_tokens = props.iter().flat_map(|x| x.finalize());
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
        },
        Err(err) => {
            panic!("{}", err)
        },
    }
}
