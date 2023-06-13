use proc_macro2::TokenStream;
use quote::quote;


// depends on reflected_attributes field in type being derived.
pub(crate) fn impl_ihaz_attributes(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    quote! {
        impl IHazAttributes for #name {
            fn get_attributes(&self) -> Vec<Rc<dyn IAttribute>> {
                Self::reflected_attributes()
            }
        
            fn get_attribute(&self, typeinfo: &TypeInfo) -> Option<Rc<dyn IAttribute>> {
                self.get_attributes().iter().filter(|a| (&a).get_type_info().is_some() && (&a).get_type_info().unwrap().is_same_as(typeinfo)).nth(0).cloned()
            }
        }
    }
}